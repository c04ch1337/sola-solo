//! Companion Scout: filtered discovery + metadata extraction + review queue.
//!
//! This module is intentionally conservative: it **does not** move anything into the Soul Vault
//! directly. Instead it places candidates into a **Pending Review** queue.

use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn vault_dir() -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(cwd.join("vault"))
}

fn review_queue_path() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("review_queue.json"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MinResolution {
    P720,
    P1080,
    P1440,
    P2160,
}

impl MinResolution {
    pub fn from_label(label: &str) -> Self {
        match label.trim().to_lowercase().as_str() {
            "2160p" | "4k" => Self::P2160,
            "1440p" => Self::P1440,
            "1080p" => Self::P1080,
            _ => Self::P720,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutFilter {
    /// Minimum resolution constraint (e.g. "1080p").
    #[serde(default = "ScoutFilter::default_min_resolution")]
    pub min_resolution: String,

    /// Research-validated sources/domains (exact host match). Empty means "any".
    #[serde(default)]
    pub preferred_sources: Vec<String>,

    /// Relevance threshold (0..1). Intended to be derived from L6 Archetypal memory.
    #[serde(default = "ScoutFilter::default_relevance_threshold")]
    pub relevance_threshold: f64,
}

impl ScoutFilter {
    fn default_min_resolution() -> String {
        "1080p".to_string()
    }

    fn default_relevance_threshold() -> f64 {
        0.55
    }

    pub fn min_resolution_enum(&self) -> MinResolution {
        MinResolution::from_label(&self.min_resolution)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MoodTag {
    Intense,
    Romantic,
    Educational,
    Playful,
    Calm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutCandidate {
    pub url: String,
    pub title: String,
    pub source_domain: String,
    pub resolution: String,
    pub relevance: f64,
    pub mood_tags: Vec<MoodTag>,
    pub kink_mapping: Vec<String>,
    pub discovered_ms: u64,
    #[serde(default)]
    pub thumbnail_url: Option<String>,
    /// Optional transcript text (captions/subtitles). Stored for review+vault capture.
    #[serde(default)]
    pub transcript_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReviewStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingReviewItem {
    pub id: String,
    pub status: ReviewStatus,
    pub candidate: ScoutCandidate,
    pub added_ms: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct ReviewQueueFile {
    items: Vec<PendingReviewItem>,
}

#[derive(Debug, Default)]
struct ReviewQueueInner {
    items: Vec<PendingReviewItem>,
}

#[derive(Debug, Default, Clone)]
pub struct ReviewQueueState {
    pub inner: Arc<RwLock<ReviewQueueInner>>,
}

impl ReviewQueueState {
    pub async fn load_or_default() -> Result<Self, String> {
        let state = Self::default();
        let path = review_queue_path()?;
        if let Ok(raw) = tokio::fs::read_to_string(&path).await {
            if let Ok(parsed) = serde_json::from_str::<ReviewQueueFile>(&raw) {
                state.inner.write().await.items = parsed.items;
            }
        }
        Ok(state)
    }

    pub async fn persist(&self) -> Result<(), String> {
        let path = review_queue_path()?;
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| e.to_string())?;
        }
        let items = self.inner.read().await.items.clone();
        let file = ReviewQueueFile { items };
        let bytes = serde_json::to_vec_pretty(&file).map_err(|e| e.to_string())?;
        tokio::fs::write(&path, bytes).await.map_err(|e| e.to_string())
    }

    pub async fn list(&self) -> Vec<PendingReviewItem> {
        self.inner.read().await.items.clone()
    }

    pub async fn enqueue(&self, candidate: ScoutCandidate) -> Result<PendingReviewItem, String> {
        let id = format!("rq_{}", now_ms());
        let item = PendingReviewItem {
            id,
            status: ReviewStatus::Pending,
            candidate,
            added_ms: now_ms(),
        };
        self.inner.write().await.items.push(item.clone());
        self.persist().await?;
        Ok(item)
    }

    pub async fn set_status(&self, id: &str, status: ReviewStatus) -> Result<(), String> {
        let mut inner = self.inner.write().await;
        let mut found = false;
        for it in &mut inner.items {
            if it.id == id {
                it.status = status.clone();
                found = true;
                break;
            }
        }
        if !found {
            return Err("Review item not found".to_string());
        }
        drop(inner);
        self.persist().await
    }

    /// Fetch a single item by id.
    pub async fn get(&self, id: &str) -> Option<PendingReviewItem> {
        self.inner
            .read()
            .await
            .items
            .iter()
            .find(|it| it.id == id)
            .cloned()
    }

    /// Remove and return an item by id.
    pub async fn take(&self, id: &str) -> Result<PendingReviewItem, String> {
        let mut inner = self.inner.write().await;
        let idx = inner
            .items
            .iter()
            .position(|it| it.id == id)
            .ok_or_else(|| "Review item not found".to_string())?;
        let item = inner.items.remove(idx);
        drop(inner);
        self.persist().await?;
        Ok(item)
    }
}

fn domain_from_url(url: &str) -> String {
    // Minimal parse: scheme://host/path
    let s = url.trim();
    let s = s.split("//").nth(1).unwrap_or(s);
    s.split('/').next().unwrap_or_default().to_lowercase()
}

fn infer_mood_tags(title: &str) -> Vec<MoodTag> {
    let t = title.to_lowercase();
    let mut out = Vec::new();
    if t.contains("intense") || t.contains("deep") || t.contains("raw") {
        out.push(MoodTag::Intense);
    }
    if t.contains("romantic") || t.contains("tender") || t.contains("date") {
        out.push(MoodTag::Romantic);
    }
    if t.contains("guide") || t.contains("tutorial") || t.contains("explained") || t.contains("research") {
        out.push(MoodTag::Educational);
    }
    if t.contains("playful") || t.contains("fun") {
        out.push(MoodTag::Playful);
    }
    if t.contains("calm") || t.contains("soft") {
        out.push(MoodTag::Calm);
    }
    if out.is_empty() {
        out.push(MoodTag::Educational);
    }
    out
}

fn infer_kink_mapping(text: &str, user_kinks: &[String]) -> Vec<String> {
    let t = text.to_lowercase();
    let mut matches = Vec::new();
    for k in user_kinks {
        let kk = k.to_lowercase();
        if kk.len() >= 3 && t.contains(&kk) {
            matches.push(k.clone());
        }
    }
    matches
}

fn passes_filter(candidate: &ScoutCandidate, filter: &ScoutFilter) -> bool {
    let min_res = filter.min_resolution_enum();
    let cand_res = MinResolution::from_label(&candidate.resolution);
    if cand_res < min_res {
        return false;
    }

    if candidate.relevance < filter.relevance_threshold {
        return false;
    }

    if filter.preferred_sources.is_empty() {
        return true;
    }

    let allowed: HashSet<String> = filter
        .preferred_sources
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    allowed.contains(&candidate.source_domain)
}

fn mock_discovery(query: &str) -> Vec<(String, String, String)> {
    // (url, title, resolution)
    vec![
        (
            "https://example.com/guide".to_string(),
            format!("Educational guide: {query}"),
            "1080p".to_string(),
        ),
        (
            "https://research.example.org/video".to_string(),
            format!("Research explained: {query}"),
            "1440p".to_string(),
        ),
        (
            "https://cdn.example.net/clip".to_string(),
            format!("Intense deep dive: {query}"),
            "720p".to_string(),
        ),
    ]
}

/// Primary entrypoint for the scout agent.
///
/// - discovers candidates
/// - extracts metadata
/// - applies filter
/// - enqueues into Pending Review
pub async fn scout_and_enqueue(
    filter: ScoutFilter,
    query: String,
    user_kinks: Vec<String>,
    queue: &ReviewQueueState,
) -> Result<Vec<PendingReviewItem>, String> {
    let discovered = mock_discovery(&query);
    let mut out = Vec::new();

    for (url, title, resolution) in discovered {
        let source_domain = domain_from_url(&url);

        // Simple relevance: presence of query tokens.
        let mut relevance: f64 = 0.5;
        for token in query.to_lowercase().split_whitespace() {
            if token.len() >= 3 && title.to_lowercase().contains(token) {
                relevance += 0.15;
            }
        }
        relevance = relevance.clamp(0.0, 1.0);

        let mood_tags = infer_mood_tags(&title);
        let kink_mapping = infer_kink_mapping(&title, &user_kinks);

        let cand = ScoutCandidate {
            url,
            title,
            source_domain,
            resolution,
            relevance,
            mood_tags,
            kink_mapping,
            discovered_ms: now_ms(),
            thumbnail_url: None,
            transcript_text: None,
        };

        if passes_filter(&cand, &filter) {
            let item = queue.enqueue(cand).await?;
            out.push(item);
        }
    }

    Ok(out)
}

/// Best-effort cleanup for any stale temp files.
pub fn cleanup_stale_downloads(_dir: &Path) {
    // placeholder; actual download staging area can be cleaned here.
}

