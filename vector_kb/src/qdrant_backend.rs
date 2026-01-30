//! Qdrant Vector Database Backend
//!
//! This module provides a high-performance vector database backend using Qdrant
//! for production RAG (Retrieval-Augmented Generation) workloads.
//!
//! Features:
//! - Persistent vector storage with Qdrant
//! - Efficient approximate nearest neighbor search
//! - Support for metadata filtering
//! - Auto-collection creation and management

#[cfg(feature = "qdrant-backend")]
use qdrant_client::{
    qdrant::{
        CreateCollectionBuilder, Distance, PointStruct, SearchPointsBuilder,
        UpsertPointsBuilder, VectorParamsBuilder, vectors_config::Config,
        VectorsConfig, Value as QdrantValue, ScrollPointsBuilder,
    },
    Qdrant,
};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{MemoryEntry, MemoryResult, VectorKbError};

/// Default collection name for Sola's long-term memory
const DEFAULT_COLLECTION: &str = "sola_history";

/// Default embedding dimension (matches MiniLM-L6-v2)
const DEFAULT_DIM: usize = 384;

/// Collection name for biometric identities (face/voice embeddings)
const IDENTITY_COLLECTION: &str = "sola_identities";

/// Embedding dimension for identity vectors (matches common face-embedding models)
const IDENTITY_DIM: usize = 512;

#[cfg(feature = "qdrant-backend")]
fn point_id_to_string(id: qdrant_client::qdrant::PointId) -> Option<String> {
    use qdrant_client::qdrant::point_id::PointIdOptions;
    match id.point_id_options? {
        PointIdOptions::Num(n) => Some(n.to_string()),
        PointIdOptions::Uuid(u) => Some(u),
    }
}

/// Qdrant-backed vector knowledge base for production RAG
#[derive(Clone)]
pub struct QdrantVectorKB {
    inner: Arc<RwLock<QdrantInner>>,
}

struct QdrantInner {
    #[cfg(feature = "qdrant-backend")]
    client: Qdrant,
    #[cfg(not(feature = "qdrant-backend"))]
    _phantom: std::marker::PhantomData<()>,
    collection_name: String,
    embedding_dim: usize,
    url: String,
}

/// Configuration for Qdrant connection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QdrantConfig {
    /// Qdrant server URL (e.g., "http://localhost:6333")
    pub url: String,
    /// Collection name for storing memories
    pub collection_name: Option<String>,
    /// Embedding dimension (default: 384 for MiniLM-L6-v2)
    pub embedding_dim: Option<usize>,
}

impl Default for QdrantConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:6333".to_string(),
            collection_name: Some(DEFAULT_COLLECTION.to_string()),
            embedding_dim: Some(DEFAULT_DIM),
        }
    }
}

impl QdrantVectorKB {
    /// Create a new Qdrant-backed vector KB
    ///
    /// # Arguments
    /// * `config` - Qdrant connection configuration
    ///
    /// # Returns
    /// A new QdrantVectorKB instance or an error if connection fails
    #[cfg(feature = "qdrant-backend")]
    pub async fn new(config: QdrantConfig) -> Result<Self, VectorKbError> {
        let collection_name = config.collection_name.unwrap_or_else(|| DEFAULT_COLLECTION.to_string());
        let embedding_dim = config.embedding_dim.unwrap_or(DEFAULT_DIM);
        
        // Connect to Qdrant
        let client = Qdrant::from_url(&config.url)
            .build()
            .map_err(|e| VectorKbError::Config(format!("Failed to connect to Qdrant at {}: {}", config.url, e)))?;
        
        let kb = Self {
            inner: Arc::new(RwLock::new(QdrantInner {
                client,
                collection_name: collection_name.clone(),
                embedding_dim,
                url: config.url.clone(),
            })),
        };
        
        // Ensure collection exists
        kb.ensure_collection().await?;

        // Ensure identity collection exists (biometric schema)
        kb.ensure_identity_collection().await?;
        
        Ok(kb)
    }

    /// Ensure the biometric identity collection exists, creating it if needed.
    ///
    /// Collection: `sola_identities`
    /// - dim: 512
    /// - distance: cosine
    #[cfg(feature = "qdrant-backend")]
    pub async fn ensure_identity_collection(&self) -> Result<(), VectorKbError> {
        let inner = self.inner.read().await;
        let collection_name = IDENTITY_COLLECTION;

        let exists = inner
            .client
            .collection_exists(collection_name)
            .await
            .map_err(|e| VectorKbError::Config(format!("Failed to check collection: {}", e)))?;

        if !exists {
            let create_collection = CreateCollectionBuilder::new(collection_name.to_string())
                .vectors_config(VectorsConfig {
                    config: Some(Config::Params(
                        VectorParamsBuilder::new(IDENTITY_DIM as u64, Distance::Cosine).build(),
                    )),
                });

            inner
                .client
                .create_collection(create_collection)
                .await
                .map_err(|e| VectorKbError::Config(format!("Failed to create collection: {}", e)))?;

            tracing::info!(
                "Created Qdrant collection '{}' with dim={} (identity vectors)",
                collection_name,
                IDENTITY_DIM
            );
        }

        Ok(())
    }

    #[cfg(not(feature = "qdrant-backend"))]
    pub async fn ensure_identity_collection(&self) -> Result<(), VectorKbError> {
        Err(VectorKbError::Config(
            "Qdrant backend is not enabled. Compile with --features qdrant-backend".to_string(),
        ))
    }

    /// Upsert a biometric embedding into the identity collection.
    ///
    /// This is intentionally minimal: callers supply an embedding + metadata.
    #[cfg(feature = "qdrant-backend")]
    pub async fn upsert_identity_embedding(
        &self,
        label: Option<String>,
        embedding: Vec<f32>,
        metadata: JsonValue,
    ) -> Result<String, VectorKbError> {
        if embedding.len() != IDENTITY_DIM {
            return Err(VectorKbError::Config(format!(
                "identity embedding dim mismatch: expected {}, got {}",
                IDENTITY_DIM,
                embedding.len()
            )));
        }

        let id = Uuid::new_v4().to_string();
        let mut payload: HashMap<String, QdrantValue> = HashMap::new();
        if let Some(label) = label {
            payload.insert("label".to_string(), QdrantValue::from(label));
        }
        payload.insert("metadata".to_string(), QdrantValue::from(metadata.to_string()));
        payload.insert(
            "created_at".to_string(),
            QdrantValue::from(chrono::Utc::now().to_rfc3339()),
        );

        let point = PointStruct::new(id.clone(), embedding, payload);
        let inner = self.inner.read().await;

        let upsert = UpsertPointsBuilder::new(IDENTITY_COLLECTION, vec![point]);
        inner
            .client
            .upsert_points(upsert)
            .await
            .map_err(|e| VectorKbError::Config(format!("Failed to upsert identity point: {}", e)))?;

        Ok(id)
    }

    #[cfg(not(feature = "qdrant-backend"))]
    pub async fn upsert_identity_embedding(
        &self,
        _label: Option<String>,
        _embedding: Vec<f32>,
        _metadata: JsonValue,
    ) -> Result<String, VectorKbError> {
        Err(VectorKbError::Config("Qdrant backend is not enabled".to_string()))
    }
    
    /// Stub implementation when qdrant-backend feature is not enabled
    #[cfg(not(feature = "qdrant-backend"))]
    pub async fn new(config: QdrantConfig) -> Result<Self, VectorKbError> {
        Err(VectorKbError::Config(
            "Qdrant backend is not enabled. Compile with --features qdrant-backend".to_string()
        ))
    }
    
    /// Ensure the collection exists, creating it if necessary
    #[cfg(feature = "qdrant-backend")]
    async fn ensure_collection(&self) -> Result<(), VectorKbError> {
        let inner = self.inner.read().await;
        let collection_name = &inner.collection_name;
        let dim = inner.embedding_dim;
        
        // Check if collection exists
        let exists = inner.client
            .collection_exists(collection_name)
            .await
            .map_err(|e| VectorKbError::Config(format!("Failed to check collection: {}", e)))?;
        
        if !exists {
            // Create collection with cosine distance
            let create_collection = CreateCollectionBuilder::new(collection_name.clone())
                .vectors_config(VectorsConfig {
                    config: Some(Config::Params(
                        VectorParamsBuilder::new(dim as u64, Distance::Cosine).build()
                    )),
                });
            
            inner.client
                .create_collection(create_collection)
                .await
                .map_err(|e| VectorKbError::Config(format!("Failed to create collection: {}", e)))?;
            
            tracing::info!("Created Qdrant collection '{}' with dim={}", collection_name, dim);
        }
        
        Ok(())
    }
    
    #[cfg(not(feature = "qdrant-backend"))]
    async fn ensure_collection(&self) -> Result<(), VectorKbError> {
        Ok(())
    }
    
    /// Get the Qdrant server URL
    pub async fn url(&self) -> String {
        self.inner.read().await.url.clone()
    }
    
    /// Get the collection name
    pub async fn collection_name(&self) -> String {
        self.inner.read().await.collection_name.clone()
    }
    
    /// Get the embedding dimension
    pub async fn embedding_dim(&self) -> usize {
        self.inner.read().await.embedding_dim
    }
    
    /// Store a memory with its embedding in Qdrant
    ///
    /// # Arguments
    /// * `text` - The text content to store
    /// * `embedding` - Pre-computed embedding vector
    /// * `metadata` - Additional metadata as JSON
    ///
    /// # Returns
    /// The stored MemoryEntry with generated ID
    #[cfg(feature = "qdrant-backend")]
    pub async fn add_memory_with_embedding(
        &self,
        text: &str,
        embedding: Vec<f32>,
        metadata: JsonValue,
    ) -> Result<MemoryEntry, VectorKbError> {
        let text = text.trim();
        if text.is_empty() {
            return Err(VectorKbError::Config("text is empty".to_string()));
        }
        
        let id = Uuid::new_v4();
        let id_str = id.to_string();
        
        // Convert metadata to Qdrant payload
        let mut payload: HashMap<String, QdrantValue> = HashMap::new();
        payload.insert("text".to_string(), QdrantValue::from(text.to_string()));
        payload.insert("metadata".to_string(), QdrantValue::from(metadata.to_string()));
        payload.insert("created_at".to_string(), QdrantValue::from(chrono::Utc::now().to_rfc3339()));
        
        // Create point
        let point = PointStruct::new(id_str.clone(), embedding.clone(), payload);
        
        let inner = self.inner.read().await;
        let upsert = UpsertPointsBuilder::new(&inner.collection_name, vec![point]);
        
        inner.client
            .upsert_points(upsert)
            .await
            .map_err(|e| VectorKbError::Config(format!("Failed to upsert point: {}", e)))?;
        
        Ok(MemoryEntry {
            id: id_str,
            text: text.to_string(),
            embedding,
            metadata,
        })
    }
    
    #[cfg(not(feature = "qdrant-backend"))]
    pub async fn add_memory_with_embedding(
        &self,
        _text: &str,
        _embedding: Vec<f32>,
        _metadata: JsonValue,
    ) -> Result<MemoryEntry, VectorKbError> {
        Err(VectorKbError::Config(
            "Qdrant backend is not enabled".to_string()
        ))
    }
    
    /// Semantic search using a pre-computed query embedding
    ///
    /// # Arguments
    /// * `query_embedding` - The query vector
    /// * `top_k` - Number of results to return
    ///
    /// # Returns
    /// Vector of MemoryResult sorted by similarity score
    #[cfg(feature = "qdrant-backend")]
    pub async fn search_with_embedding(
        &self,
        query_embedding: Vec<f32>,
        top_k: usize,
    ) -> Result<Vec<MemoryResult>, VectorKbError> {
        let top_k = top_k.clamp(1, 100);
        
        let inner = self.inner.read().await;
        let search = SearchPointsBuilder::new(&inner.collection_name, query_embedding, top_k as u64)
            .with_payload(true);
        
        let results = inner.client
            .search_points(search)
            .await
            .map_err(|e| VectorKbError::Config(format!("Search failed: {}", e)))?;
        
        let memories: Vec<MemoryResult> = results.result
            .into_iter()
            .filter_map(|point| {
                let id = point_id_to_string(point.id?)?;
                let payload = point.payload;
                
                let text = payload.get("text")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                
                let metadata_str = payload
                    .get("metadata")
                    .and_then(|v| v.as_str())
                    .map_or("{}", |v| v);
                
                let metadata: JsonValue = serde_json::from_str(metadata_str).unwrap_or(JsonValue::Null);
                
                // Qdrant returns cosine similarity in range [-1, 1], normalize to [0, 1]
                let score = ((point.score + 1.0) / 2.0).clamp(0.0, 1.0);
                
                Some(MemoryResult {
                    id,
                    text,
                    score,
                    metadata,
                })
            })
            .collect();
        
        Ok(memories)
    }
    
    #[cfg(not(feature = "qdrant-backend"))]
    pub async fn search_with_embedding(
        &self,
        _query_embedding: Vec<f32>,
        _top_k: usize,
    ) -> Result<Vec<MemoryResult>, VectorKbError> {
        Err(VectorKbError::Config(
            "Qdrant backend is not enabled".to_string()
        ))
    }
    
    /// Get all memories from the collection
    #[cfg(feature = "qdrant-backend")]
    pub async fn all(&self) -> Result<Vec<MemoryEntry>, VectorKbError> {
        let inner = self.inner.read().await;
        
        let scroll = ScrollPointsBuilder::new(&inner.collection_name)
            .with_payload(true)
            .with_vectors(true)
            .limit(10000); // Reasonable limit
        
        let results = inner.client
            .scroll(scroll)
            .await
            .map_err(|e| VectorKbError::Config(format!("Scroll failed: {}", e)))?;
        
        let entries: Vec<MemoryEntry> = results.result
            .into_iter()
            .filter_map(|point| {
                let id = point_id_to_string(point.id?)?;
                let payload = point.payload;
                
                let text = payload.get("text")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                
                let metadata_str = payload
                    .get("metadata")
                    .and_then(|v| v.as_str())
                    .map_or("{}", |v| v);
                
                let metadata: JsonValue = serde_json::from_str(metadata_str).unwrap_or(JsonValue::Null);
                
                // Extract embedding from vectors
                let embedding = point.vectors
                    .and_then(|v| v.vectors_options)
                    .and_then(|vo| {
                        use qdrant_client::qdrant::vectors_output::VectorsOptions;
                        match vo {
                            VectorsOptions::Vector(v) => Some(v.data),
                            _ => None,
                        }
                    })
                    .unwrap_or_default();
                
                Some(MemoryEntry {
                    id,
                    text,
                    embedding,
                    metadata,
                })
            })
            .collect();
        
        Ok(entries)
    }
    
    #[cfg(not(feature = "qdrant-backend"))]
    pub async fn all(&self) -> Result<Vec<MemoryEntry>, VectorKbError> {
        Err(VectorKbError::Config(
            "Qdrant backend is not enabled".to_string()
        ))
    }
    
    /// Get collection statistics
    #[cfg(feature = "qdrant-backend")]
    pub async fn stats(&self) -> Result<CollectionStats, VectorKbError> {
        let inner = self.inner.read().await;
        
        let info = inner.client
            .collection_info(&inner.collection_name)
            .await
            .map_err(|e| VectorKbError::Config(format!("Failed to get collection info: {}", e)))?;
        
        Ok(CollectionStats {
            collection_name: inner.collection_name.clone(),
            points_count: info
                .result
                .as_ref()
                .map(|r| r.points_count.unwrap_or(0))
                .unwrap_or(0),
            // Older/newer Qdrant APIs vary; keep this best-effort.
            vectors_count: 0,
            status: "ready".to_string(),
        })
    }
    
    #[cfg(not(feature = "qdrant-backend"))]
    pub async fn stats(&self) -> Result<CollectionStats, VectorKbError> {
        Err(VectorKbError::Config(
            "Qdrant backend is not enabled".to_string()
        ))
    }

    /// Search for matching identities in the sola_identities collection.
    ///
    /// # Arguments
    /// * `embedding` - The query embedding vector (512-dim)
    /// * `top_k` - Number of results to return
    ///
    /// # Returns
    /// Vector of IdentitySearchResult sorted by similarity score
    #[cfg(feature = "qdrant-backend")]
    pub async fn search_identity(
        &self,
        embedding: Vec<f32>,
        top_k: usize,
    ) -> Result<Vec<IdentitySearchResult>, VectorKbError> {
        let top_k = top_k.clamp(1, 10);

        let inner = self.inner.read().await;
        let search = SearchPointsBuilder::new(IDENTITY_COLLECTION, embedding, top_k as u64)
            .with_payload(true);

        let results = inner.client
            .search_points(search)
            .await
            .map_err(|e| VectorKbError::Config(format!("Identity search failed: {}", e)))?;

        let identities: Vec<IdentitySearchResult> = results.result
            .into_iter()
            .filter_map(|point| {
                let id = point_id_to_string(point.id?)?;
                let payload = point.payload;

                let label = payload.get("label")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());

                // Qdrant returns cosine similarity in range [-1, 1], normalize to [0, 1]
                let score = ((point.score + 1.0) / 2.0).clamp(0.0, 1.0);

                Some(IdentitySearchResult {
                    id,
                    label,
                    score,
                })
            })
            .collect();

        Ok(identities)
    }

    #[cfg(not(feature = "qdrant-backend"))]
    pub async fn search_identity(
        &self,
        _embedding: Vec<f32>,
        _top_k: usize,
    ) -> Result<Vec<IdentitySearchResult>, VectorKbError> {
        Err(VectorKbError::Config(
            "Qdrant backend is not enabled".to_string()
        ))
    }
}

/// Statistics about a Qdrant collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionStats {
    pub collection_name: String,
    pub points_count: u64,
    pub vectors_count: u64,
    pub status: String,
}

/// Result from identity search in sola_identities collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentitySearchResult {
    /// Point ID in Qdrant
    pub id: String,
    /// Human-readable label (e.g., "Master", "jamey")
    pub label: Option<String>,
    /// Similarity score (0.0 to 1.0)
    pub score: f32,
}

/// Helper to check if Qdrant is available at the given URL
#[cfg(feature = "qdrant-backend")]
pub async fn check_qdrant_health(url: &str) -> Result<bool, VectorKbError> {
    let client = Qdrant::from_url(url)
        .build()
        .map_err(|e| VectorKbError::Config(format!("Failed to connect: {}", e)))?;
    
    // Try to list collections as a health check
    client.list_collections()
        .await
        .map(|_| true)
        .map_err(|e| VectorKbError::Config(format!("Health check failed: {}", e)))
}

#[cfg(not(feature = "qdrant-backend"))]
pub async fn check_qdrant_health(_url: &str) -> Result<bool, VectorKbError> {
    Err(VectorKbError::Config(
        "Qdrant backend is not enabled".to_string()
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    #[ignore] // Requires running Qdrant instance
    async fn test_qdrant_connection() {
        let config = QdrantConfig::default();
        let kb = QdrantVectorKB::new(config).await.unwrap();
        assert_eq!(kb.collection_name().await, DEFAULT_COLLECTION);
    }
}
