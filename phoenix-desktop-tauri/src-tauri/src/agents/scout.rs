//! Scout Agent: video/media research using yt-dlp (managed via the `yt-dlp` crate).
//!
//! The `yt-dlp` crate downloads/installs `yt-dlp` + `ffmpeg` into a local libs directory.
//! We then invoke the installed `yt-dlp` binary to perform `ytsearch` and transcript capture.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;

use crate::tools::video_scout::{MoodTag, ScoutCandidate, ScoutFilter};

fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn vault_dir() -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(cwd.join("vault"))
}

fn libs_dir() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("libs"))
}

fn scout_temp_dir() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("research").join("scout_tmp"))
}

fn domain_from_url(url: &str) -> String {
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

fn compute_relevance(query: &str, haystack: &str) -> f64 {
    let mut relevance: f64 = 0.35;
    let q = query.to_lowercase();
    let h = haystack.to_lowercase();
    for token in q.split_whitespace() {
        if token.len() >= 3 && h.contains(token) {
            relevance += 0.12;
        }
    }
    relevance.clamp(0.0, 1.0)
}

fn resolution_from_height(height: Option<u64>) -> String {
    match height.unwrap_or(0) {
        h if h >= 2160 => "2160p".to_string(),
        h if h >= 1440 => "1440p".to_string(),
        h if h >= 1080 => "1080p".to_string(),
        h if h >= 720 => "720p".to_string(),
        _ => "720p".to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutFinding {
    pub candidate: ScoutCandidate,
}

#[derive(Debug, Deserialize)]
struct YtDlpJsonLine {
    title: Option<String>,
    webpage_url: Option<String>,
    original_url: Option<String>,
    thumbnail: Option<String>,
    height: Option<u64>,
}

pub struct ScoutAgent;

impl ScoutAgent {
    async fn ensure_local_binaries() -> Result<(PathBuf, PathBuf), String> {
        let libs = libs_dir()?;
        tokio::fs::create_dir_all(&libs).await.map_err(|e| e.to_string())?;

        // Install binaries (or reuse if already present).
        let yt_path = libs.join("yt-dlp");
        let ff_path = libs.join("ffmpeg");
        let libraries = yt_dlp::client::deps::Libraries::new(yt_path, ff_path);
        let installed = libraries
            .install_dependencies()
            .await
            .map_err(|e| e.to_string())?;
        Ok((installed.youtube, installed.ffmpeg))
    }

    async fn run_yt_dlp_lines(args: &[String]) -> Result<Vec<String>, String> {
        let (yt_bin, _ff) = Self::ensure_local_binaries().await?;
        let tmp = scout_temp_dir()?;
        tokio::fs::create_dir_all(&tmp).await.map_err(|e| e.to_string())?;

        let mut cmd = tokio::process::Command::new(&yt_bin);
        cmd.current_dir(&tmp);
        for a in args {
            cmd.arg(a);
        }
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| e.to_string())?;
        let mut stdout = String::new();
        let mut stderr = String::new();

        if let Some(mut out) = child.stdout.take() {
            out.read_to_string(&mut stdout).await.map_err(|e| e.to_string())?;
        }
        if let Some(mut err) = child.stderr.take() {
            err.read_to_string(&mut stderr).await.map_err(|e| e.to_string())?;
        }
        let status = child.wait().await.map_err(|e| e.to_string())?;
        if !status.success() {
            return Err(format!("yt-dlp failed: {}", stderr.trim()));
        }

        Ok(stdout
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect())
    }

    async fn try_fetch_transcript(url: &str) -> Result<Option<String>, String> {
        // Best-effort transcript extraction. This uses yt-dlp subtitle download.
        // We keep it conservative: skip if it fails.
        let tmp = scout_temp_dir()?;
        tokio::fs::create_dir_all(&tmp).await.map_err(|e| e.to_string())?;

        // Clean old subtitle files to avoid mixing results.
        cleanup_dir_by_extension(&tmp, &["vtt", "srt"]).await;

        let mut args: Vec<String> = Vec::new();
        args.push("--skip-download".to_string());
        args.push("--no-warnings".to_string());
        args.push("--no-progress".to_string());
        args.push("--write-auto-subs".to_string());
        args.push("--write-subs".to_string());
        args.push("--sub-lang".to_string());
        args.push("en".to_string());
        args.push("--sub-format".to_string());
        args.push("vtt/srt/best".to_string());
        args.push("--output".to_string());
        args.push("%(id)s.%(ext)s".to_string());
        args.push(url.to_string());

        // Ignore errors (return None).
        let _ = Self::run_yt_dlp_lines(&args).await;

        // Pick the newest subtitle file.
        let subtitle = newest_file_with_ext(&tmp, &["vtt", "srt"]).await;
        let Some(path) = subtitle else {
            return Ok(None);
        };
        let raw = tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())?;
        Ok(Some(strip_vtt_srt(&raw)))
    }

    pub async fn search_media(
        query: String,
        filter: ScoutFilter,
        user_kinks: Vec<String>,
    ) -> Result<Vec<ScoutFinding>, String> {
        // yt-dlp search. Use a small cap for responsiveness.
        let search = format!("ytsearch10:{query}");

        let args: Vec<String> = vec![
            "--dump-json".to_string(),
            "--skip-download".to_string(),
            "--no-warnings".to_string(),
            "--no-progress".to_string(),
            "--no-playlist".to_string(),
            search,
        ];

        let lines = Self::run_yt_dlp_lines(&args).await?;
        let mut out: Vec<ScoutFinding> = Vec::new();

        for line in lines {
            let parsed: YtDlpJsonLine = match serde_json::from_str(&line) {
                Ok(p) => p,
                Err(_) => continue,
            };
            let title = parsed.title.unwrap_or_else(|| "Untitled".to_string());
            let url = parsed
                .webpage_url
                .or(parsed.original_url)
                .unwrap_or_else(|| "".to_string());
            if url.trim().is_empty() {
                continue;
            }

            let resolution = resolution_from_height(parsed.height);
            let source_domain = domain_from_url(&url);
            let relevance = compute_relevance(&query, &title);
            let mood_tags = infer_mood_tags(&title);
            let kink_mapping = infer_kink_mapping(&title, &user_kinks);

            let cand = ScoutCandidate {
                url: url.clone(),
                title: title.clone(),
                source_domain,
                resolution,
                relevance,
                mood_tags,
                kink_mapping,
                discovered_ms: now_ms(),
                thumbnail_url: parsed.thumbnail,
                transcript_text: None,
            };

            if !passes_filter_local(&cand, &filter) {
                continue;
            }

            let transcript_text = Self::try_fetch_transcript(&url).await.ok().flatten();
            let cand = ScoutCandidate {
                transcript_text,
                ..cand
            };

            out.push(ScoutFinding { candidate: cand });
        }

        Ok(out)
    }
}

fn min_resolution_value(label: &str) -> u32 {
    match label.trim().to_lowercase().as_str() {
        "2160p" | "4k" => 2160,
        "1440p" => 1440,
        "1080p" => 1080,
        _ => 720,
    }
}

fn candidate_resolution_value(label: &str) -> u32 {
    min_resolution_value(label)
}

fn passes_filter_local(c: &ScoutCandidate, filter: &ScoutFilter) -> bool {
    if candidate_resolution_value(&c.resolution) < min_resolution_value(&filter.min_resolution) {
        return false;
    }
    if c.relevance < filter.relevance_threshold {
        return false;
    }
    if filter.preferred_sources.is_empty() {
        return true;
    }
    let allowed: std::collections::HashSet<String> = filter
        .preferred_sources
        .iter()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    allowed.contains(&c.source_domain)
}

async fn newest_file_with_ext(dir: &Path, exts: &[&str]) -> Option<PathBuf> {
    let mut best: Option<(u64, PathBuf)> = None;
    let mut rd = tokio::fs::read_dir(dir).await.ok()?;
    while let Ok(Some(ent)) = rd.next_entry().await {
        let path = ent.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
        if !exts.iter().any(|e| e.eq_ignore_ascii_case(&ext)) {
            continue;
        }
        if let Ok(meta) = ent.metadata().await {
            let ts = meta
                .modified()
                .ok()
                .and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_millis() as u64)
                .unwrap_or(0);
            match &best {
                Some((bts, _)) if ts <= *bts => {}
                _ => best = Some((ts, path)),
            }
        }
    }
    best.map(|(_, p)| p)
}

async fn cleanup_dir_by_extension(dir: &Path, exts: &[&str]) {
    let mut rd = match tokio::fs::read_dir(dir).await {
        Ok(r) => r,
        Err(_) => return,
    };
    while let Ok(Some(ent)) = rd.next_entry().await {
        let path = ent.path();
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();
        if exts.iter().any(|e| e.eq_ignore_ascii_case(&ext)) {
            let _ = tokio::fs::remove_file(path).await;
        }
    }
}

fn strip_vtt_srt(raw: &str) -> String {
    // Quick normalization: remove timestamps and WEBVTT header; keep readable lines.
    raw.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .filter(|l| !l.starts_with("WEBVTT"))
        .filter(|l| !l.contains("-->") && !l.chars().all(|c| c.is_ascii_digit()))
        .collect::<Vec<_>>()
        .join("\n")
}

