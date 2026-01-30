//! Sensory layer for Sola's physical awareness.
//!
//! This module provides access to local hardware sensors (camera, microphone)
//! and processes their data for identity recognition and voice activity detection.
//!
//! ## Feature Flags
//!
//! - `sensory`: Enables real hardware access via OpenCV and CPAL.
//!   Without this flag, only placeholder implementations are available.
//!
//! ## Architecture
//!
//! The sensory subsystem uses a non-blocking architecture:
//! - **Vision**: A dedicated thread captures camera frames and broadcasts them
//!   via `tokio::sync::watch` channels.
//! - **Audio**: CPAL streams audio to a Silero VAD model for voice detection.

pub mod audio;
pub mod vision;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

pub use audio::{extract_voiceprint, AudioPipeline, AudioStatus, VoiceActivity, VoiceprintEmbedding};
pub use vision::{
    extract_embedding_from_jpeg, detect_liveness, enroll_with_liveness,
    CameraFrame, CameraStatus, CameraWorker, FaceEmbedding, LivenessResult, SecurityError,
};

/// Result of a best-effort identity lookup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityMatch {
    /// Human-readable label (e.g. username). `None` means "unknown".
    pub label: Option<String>,
    /// 0.0..=1.0 confidence score.
    pub confidence: f32,
}

/// Combined status of all sensory subsystems.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SensoryStatus {
    pub camera: CameraStatusDto,
    pub audio: AudioStatusDto,
}

/// Camera status for API responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CameraStatusDto {
    pub running: bool,
    pub camera_index: i32,
    pub fps: f32,
    pub last_error: Option<String>,
}

/// Audio status for API responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioStatusDto {
    pub running: bool,
    pub sample_rate: u32,
    pub channels: u16,
    pub last_error: Option<String>,
    pub speech_events: u64,
}

impl From<CameraStatus> for CameraStatusDto {
    fn from(s: CameraStatus) -> Self {
        Self {
            running: s.running,
            camera_index: s.camera_index,
            fps: s.fps,
            last_error: s.last_error,
        }
    }
}

impl From<AudioStatus> for AudioStatusDto {
    fn from(s: AudioStatus) -> Self {
        Self {
            running: s.running,
            sample_rate: s.sample_rate,
            channels: s.channels,
            last_error: s.last_error,
            speech_events: s.speech_events,
        }
    }
}

/// Central access point for local sensory inputs.
///
/// This hub manages the camera and audio pipelines and provides
/// a unified interface for querying sensory data.
pub struct SensoryHub {
    camera: Arc<RwLock<CameraWorker>>,
    audio: Arc<RwLock<AudioPipeline>>,
}

impl SensoryHub {
    /// Create a new hub with uninitialized backends.
    pub fn new() -> Self {
        Self {
            camera: Arc::new(RwLock::new(CameraWorker::new())),
            audio: Arc::new(RwLock::new(AudioPipeline::new())),
        }
    }

    /// Start all sensory subsystems.
    ///
    /// This will attempt to start the camera and audio pipelines.
    /// Errors are logged but do not prevent other subsystems from starting.
    pub async fn start_all(&self) {
        // Start camera
        {
            let mut camera = self.camera.write().await;
            if let Err(e) = camera.start(0) {
                eprintln!("[sensory] Failed to start camera: {}", e);
            } else {
                eprintln!("[sensory] Camera started");
            }
        }

        // Start audio
        {
            let mut audio = self.audio.write().await;
            if let Err(e) = audio.start() {
                eprintln!("[sensory] Failed to start audio: {}", e);
            } else {
                eprintln!("[sensory] Audio pipeline started");
            }
        }
    }

    /// Stop all sensory subsystems.
    pub async fn stop_all(&self) {
        {
            let mut camera = self.camera.write().await;
            camera.stop();
        }
        {
            let mut audio = self.audio.write().await;
            audio.stop();
        }
        eprintln!("[sensory] All subsystems stopped");
    }

    /// Get the combined status of all sensory subsystems.
    pub async fn status(&self) -> SensoryStatus {
        let camera_status = {
            let camera = self.camera.read().await;
            camera.status()
        };
        let audio_status = {
            let audio = self.audio.read().await;
            audio.status()
        };

        SensoryStatus {
            camera: camera_status.into(),
            audio: audio_status.into(),
        }
    }

    /// Get the latest camera frame.
    pub async fn latest_frame(&self) -> CameraFrame {
        let camera = self.camera.read().await;
        camera.latest_frame()
    }

    /// Get the latest voice activity detection result.
    pub async fn latest_vad(&self) -> VoiceActivity {
        let audio = self.audio.read().await;
        audio.latest_vad()
    }

    /// Sola can call this tool to "Look" at who is in front of the PC.
    ///
    /// This method performs multi-factor biometric identification:
    /// 1. Gets the latest camera frame and extracts face embedding
    /// 2. Gets captured audio and extracts voice embedding (if available)
    /// 3. Queries Qdrant for best matches for both modalities
    /// 4. Applies biometric fusion: if BOTH face and voice confidence > 0.7,
    ///    sets final confidence to 1.0 (multi-factor authenticated)
    ///
    /// Returns an IdentityMatch with label and confidence.
    pub async fn identify_presence(&self) -> Result<IdentityMatch> {
        let frame = self.latest_frame().await;

        if frame.jpeg_data.is_empty() {
            return Ok(IdentityMatch {
                label: None,
                confidence: 0.0,
            });
        }

        // Extract face embedding from the frame
        let face_embedding = match extract_embedding_from_jpeg(&frame.jpeg_data) {
            Ok(Some(emb)) => Some(emb),
            Ok(None) => {
                eprintln!("[sensory] No face detected in frame");
                None
            }
            Err(e) => {
                eprintln!("[sensory] Face embedding extraction failed: {}", e);
                None
            }
        };

        // Try to get voice embedding from captured audio
        let audio_samples = self.get_captured_audio().await;
        let voice_embedding = if audio_samples.len() >= 16000 {
            // At least 1 second of audio
            let voiceprint = extract_voiceprint(&audio_samples);
            if voiceprint.confidence > 0.0 {
                Some(voiceprint)
            } else {
                None
            }
        } else {
            None
        };

        // If no face detected, return early
        let face_emb = match face_embedding {
            Some(emb) => emb,
            None => {
                return Ok(IdentityMatch {
                    label: None,
                    confidence: 0.0,
                });
            }
        };

        // Query Qdrant for best match
        let qdrant_url = std::env::var("QDRANT_URL")
            .unwrap_or_else(|_| "http://localhost:6333".to_string());

        let kb = match vector_kb::qdrant_backend::QdrantVectorKB::new(
            vector_kb::qdrant_backend::QdrantConfig {
                url: qdrant_url,
                collection_name: None,
                embedding_dim: None,
            },
        )
        .await
        {
            Ok(kb) => kb,
            Err(e) => {
                eprintln!("[sensory] Failed to connect to Qdrant: {}", e);
                return Ok(IdentityMatch {
                    label: None,
                    confidence: 0.0,
                });
            }
        };

        // Search for face match
        let face_match = match kb.search_identity(face_emb.embedding.clone(), 1).await {
            Ok(results) if !results.is_empty() => Some((
                results[0].label.clone(),
                results[0].score,
            )),
            Ok(_) => None,
            Err(e) => {
                eprintln!("[sensory] Face identity search failed: {}", e);
                None
            }
        };

        // Search for voice match (if we have voice embedding)
        let voice_match = if let Some(ref voiceprint) = voice_embedding {
            match kb.search_identity(voiceprint.embedding.clone(), 1).await {
                Ok(results) if !results.is_empty() => {
                    // Voice embeddings are stored with "_voice" suffix
                    let label = results[0].label.as_ref().map(|l| {
                        l.strip_suffix("_voice").unwrap_or(l).to_string()
                    });
                    Some((label, results[0].score))
                }
                Ok(_) => None,
                Err(e) => {
                    eprintln!("[sensory] Voice identity search failed: {}", e);
                    None
                }
            }
        } else {
            None
        };

        // Biometric fusion logic
        match (face_match, voice_match) {
            // Both modalities matched with high confidence to the same person
            (Some((Some(face_label), face_conf)), Some((Some(voice_label), voice_conf)))
                if face_label == voice_label && face_conf > 0.7 && voice_conf > 0.7 =>
            {
                eprintln!(
                    "[sensory] Multi-factor authentication: {} (face: {:.2}, voice: {:.2})",
                    face_label, face_conf, voice_conf
                );
                Ok(IdentityMatch {
                    label: Some(face_label),
                    confidence: 1.0, // Multi-factor authenticated!
                })
            }
            // Face matched with high confidence
            (Some((label, conf)), _) if conf > 0.7 => {
                eprintln!("[sensory] Face match: {:?} ({:.2})", label, conf);
                Ok(IdentityMatch {
                    label,
                    confidence: conf,
                })
            }
            // Voice matched with high confidence (face didn't match well)
            (_, Some((label, conf))) if conf > 0.7 => {
                eprintln!("[sensory] Voice match: {:?} ({:.2})", label, conf);
                Ok(IdentityMatch {
                    label,
                    confidence: conf * 0.9, // Slightly lower confidence for voice-only
                })
            }
            // Face detected but low confidence match
            (Some((label, conf)), _) => {
                eprintln!("[sensory] Low confidence face match: {:?} ({:.2})", label, conf);
                Ok(IdentityMatch {
                    label,
                    confidence: conf,
                })
            }
            // No match found
            _ => {
                eprintln!("[sensory] No identity match found");
                Ok(IdentityMatch {
                    label: None,
                    confidence: 0.0,
                })
            }
        }
    }

    /// Extract face embedding from the current camera frame.
    ///
    /// Returns None if no face is detected or if the sensory feature is disabled.
    pub async fn extract_face_embedding(&self) -> Option<FaceEmbedding> {
        let frame = self.latest_frame().await;
        if frame.jpeg_data.is_empty() {
            return None;
        }

        match extract_embedding_from_jpeg(&frame.jpeg_data) {
            Ok(emb) => emb,
            Err(e) => {
                eprintln!("[sensory] Face embedding extraction failed: {}", e);
                None
            }
        }
    }

    /// Get a reference to the camera worker.
    pub fn camera(&self) -> Arc<RwLock<CameraWorker>> {
        self.camera.clone()
    }

    /// Get a reference to the audio pipeline.
    pub fn audio(&self) -> Arc<RwLock<AudioPipeline>> {
        self.audio.clone()
    }

    /// Start capturing audio samples for voiceprint extraction.
    pub async fn start_audio_capture(&self) {
        let audio = self.audio.read().await;
        audio.start_capture();
    }

    /// Stop capturing audio samples.
    pub async fn stop_audio_capture(&self) {
        let audio = self.audio.read().await;
        audio.stop_capture();
    }

    /// Get captured audio samples (16kHz mono).
    pub async fn get_captured_audio(&self) -> Vec<f32> {
        let audio = self.audio.read().await;
        audio.get_captured_samples()
    }

    /// Extract voiceprint from captured audio.
    pub async fn extract_voiceprint_from_capture(&self) -> VoiceprintEmbedding {
        let samples = self.get_captured_audio().await;
        extract_voiceprint(&samples)
    }
}

impl Default for SensoryHub {
    fn default() -> Self {
        Self::new()
    }
}
