//! Vector Knowledge Base (Phase 2)
//!
//! This crate provides a persistent vector store with an embedding layer and
//! semantic search over stored memories.
//!
//! Notes:
//! - Default build uses a lightweight deterministic embedder (`stub-embeddings`)
//!   so Phoenix can compile/run offline without ML model downloads.
//! - You can enable real embeddings later behind the `real-embeddings` feature.

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use uuid::Uuid;

/// Public result shape returned by semantic search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryResult {
    pub id: String,
    pub text: String,
    /// 0.0..=1.0 cosine similarity (normalized).
    pub score: f32,
    pub metadata: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: String,
    pub text: String,
    pub embedding: Vec<f32>,
    pub metadata: JsonValue,
}

#[derive(Debug, thiserror::Error)]
pub enum VectorKbError {
    #[error("database error: {0}")]
    Db(#[from] sled::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("invalid configuration: {0}")]
    Config(String),
}

type Result<T> = std::result::Result<T, VectorKbError>;

/// Internal embedder trait.
trait Embedder: Send + Sync {
    fn dim(&self) -> usize;
    fn encode(&self, text: &str) -> Vec<f32>;
}

/// A lightweight deterministic embedder (hashing trick) that produces stable vectors.
///
/// This is not as strong as transformer embeddings, but it enables fully offline
/// semantic-ish recall without pulling in heavy model runtimes.
struct StubEmbedder {
    dim: usize,
}

impl StubEmbedder {
    fn new(dim: usize) -> Self {
        Self { dim }
    }
}

impl Embedder for StubEmbedder {
    fn dim(&self) -> usize {
        self.dim
    }

    fn encode(&self, text: &str) -> Vec<f32> {
        // Simple token hashing + L2 normalize.
        let mut v = vec![0.0f32; self.dim];
        let lower = text.to_ascii_lowercase();
        for token in lower
            .split(|c: char| !c.is_alphanumeric())
            .filter(|t| !t.is_empty())
        {
            let h = fxhash32(token.as_bytes());
            let idx = (h as usize) % self.dim;
            v[idx] += 1.0;
        }
        l2_normalize(&mut v);
        v
    }
}

fn fxhash32(bytes: &[u8]) -> u32 {
    // Small stable hash for token -> index mapping.
    let mut h: u32 = 2166136261;
    for &b in bytes {
        h ^= b as u32;
        h = h.wrapping_mul(16777619);
    }
    h
}

fn l2_normalize(v: &mut [f32]) {
    let mut sum = 0.0f32;
    for x in v.iter() {
        sum += x * x;
    }
    let norm = sum.sqrt();
    if norm > 0.0 {
        for x in v.iter_mut() {
            *x /= norm;
        }
    }
}

fn cosine_sim(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0f32;
    for i in 0..a.len() {
        dot += a[i] * b[i];
    }
    // Because we normalize embeddings, dot is cosine similarity in [-1..1].
    // Clamp to [0..1] for UI friendliness.
    ((dot + 1.0) / 2.0).clamp(0.0, 1.0)
}

#[derive(Clone)]
pub struct VectorKB {
    inner: Arc<RwLock<Inner>>,
}

struct Inner {
    db: sled::Db,
    tree: sled::Tree,
    embedder: Box<dyn Embedder>,
    path: PathBuf,
}

impl VectorKB {
    /// Initialize the vector KB at `path`.
    ///
    /// This uses `sled` for persistence today; the crate still declares the LanceDB
    /// dependencies to align with the Phase 2 plan and allow future swapping.
    pub fn new(path: &str) -> Result<Self> {
        let p = Path::new(path);
        std::fs::create_dir_all(p)
            .map_err(|e| VectorKbError::Config(format!("failed to create db dir: {e}")))?;

        let db = sled::open(p.join("vector_kb.sled"))?;
        let tree = db.open_tree("entries")?;

        // MiniLM-L6-v2 is 384-dim; keep that default.
        let embedder: Box<dyn Embedder> = Box::new(StubEmbedder::new(384));

        Ok(Self {
            inner: Arc::new(RwLock::new(Inner {
                db,
                tree,
                embedder,
                path: p.to_path_buf(),
            })),
        })
    }

    pub fn path(&self) -> PathBuf {
        self.inner.read().path.clone()
    }

    pub fn embedding_dim(&self) -> usize {
        self.inner.read().embedder.dim()
    }

    /// Embed and store a memory.
    pub async fn add_memory(&self, text: &str, metadata: JsonValue) -> Result<MemoryEntry> {
        let text = text.trim();
        if text.is_empty() {
            return Err(VectorKbError::Config("text is empty".to_string()));
        }
        let id = Uuid::new_v4().to_string();

        // Embedding is CPU-only; keep it non-blocking for callers.
        let emb = {
            let inner = self.inner.read();
            inner.embedder.encode(text)
        };

        let entry = MemoryEntry {
            id: id.clone(),
            text: text.to_string(),
            embedding: emb,
            metadata,
        };

        // Use JSON encoding for persistence because `serde_json::Value` is not round-trippable
        // with `bincode` (it relies on `deserialize_any`).
        let bytes = serde_json::to_vec(&entry)?;
        // Avoid holding the (sync) RwLock guard across `.await`.
        let (tree, db) = {
            let inner = self.inner.read();
            (inner.tree.clone(), inner.db.clone())
        };
        tree.insert(id.as_bytes(), bytes)?;
        db.flush_async().await?;
        Ok(entry)
    }

    /// Synchronous variant of [`VectorKB::add_memory()`](vector_kb/src/lib.rs:1).
    pub fn add_memory_sync(&self, text: &str, metadata: JsonValue) -> Result<MemoryEntry> {
        let text = text.trim();
        if text.is_empty() {
            return Err(VectorKbError::Config("text is empty".to_string()));
        }
        let id = Uuid::new_v4().to_string();
        let emb = {
            let inner = self.inner.read();
            inner.embedder.encode(text)
        };

        let entry = MemoryEntry {
            id: id.clone(),
            text: text.to_string(),
            embedding: emb,
            metadata,
        };

        let bytes = serde_json::to_vec(&entry)?;
        {
            let inner = self.inner.read();
            inner.tree.insert(id.as_bytes(), bytes)?;
            inner.db.flush()?;
        }
        Ok(entry)
    }

    pub async fn all(&self) -> Result<Vec<MemoryEntry>> {
        let inner = self.inner.read();
        let mut out = Vec::new();
        for kv in inner.tree.iter() {
            let (_k, v) = kv?;
            // Best-effort decode: skip unreadable/corrupt entries rather than failing the whole call.
            if let Ok(entry) = serde_json::from_slice::<MemoryEntry>(&v) {
                out.push(entry);
            }
        }
        Ok(out)
    }

    pub fn all_sync(&self) -> Result<Vec<MemoryEntry>> {
        let inner = self.inner.read();
        let mut out = Vec::new();
        for kv in inner.tree.iter() {
            let (_k, v) = kv?;
            if let Ok(entry) = serde_json::from_slice::<MemoryEntry>(&v) {
                out.push(entry);
            }
        }
        Ok(out)
    }

    /// Semantic search by cosine similarity.
    pub async fn semantic_search(&self, query: &str, top_k: usize) -> Result<Vec<MemoryResult>> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(vec![]);
        }
        let top_k = top_k.clamp(1, 100);

        let query_emb = {
            let inner = self.inner.read();
            inner.embedder.encode(query)
        };

        let all = self.all().await?;
        let mut scored = all
            .into_iter()
            .map(|e| {
                let score = cosine_sim(&query_emb, &e.embedding);
                MemoryResult {
                    id: e.id,
                    text: e.text,
                    score,
                    metadata: e.metadata,
                }
            })
            .collect::<Vec<_>>();

        scored.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored.truncate(top_k);
        Ok(scored)
    }

    pub fn semantic_search_sync(&self, query: &str, top_k: usize) -> Result<Vec<MemoryResult>> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(vec![]);
        }
        let top_k = top_k.clamp(1, 100);

        let query_emb = {
            let inner = self.inner.read();
            inner.embedder.encode(query)
        };

        let all = self.all_sync()?;
        let mut scored = all
            .into_iter()
            .map(|e| {
                let score = cosine_sim(&query_emb, &e.embedding);
                MemoryResult {
                    id: e.id,
                    text: e.text,
                    score,
                    metadata: e.metadata,
                }
            })
            .collect::<Vec<_>>();

        scored.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        scored.truncate(top_k);
        Ok(scored)
    }
}
