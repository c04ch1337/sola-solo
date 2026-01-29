//! Audio Intelligence Module for Phoenix AGI
//!
//! Provides continuous ambient recording, meeting transcription, and audio analysis.
//!
//! Features:
//! - Continuous ambient recording with ring buffer
//! - Voice activity detection (VAD)
//! - Wake word detection
//! - Meeting transcription with speaker diarization
//! - Audio analysis (tone, sentiment, keywords)
//!
//! Memory Integration:
//! - L1 (Instant Cache): STM layer - `stm:sensory:audio:{timestamp}`
//! - L2 (Working Memory): WM layer - `wm:sensory:screen:{timestamp}`
//! - L3 (Episodic): EPM layer - `epm:sensory:session:{timestamp}`
//! - L4 (Semantic): Mind Vault - `mind:sensory:extracted:{category}`

use chrono::Utc;
use neural_cortex_strata::{MemoryLayer, NeuralCortexStrata};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use thiserror::Error;
use tokio::sync::Mutex;
use uuid::Uuid;
use vital_organ_vaults::VitalOrganVaults;

#[derive(Debug, Error)]
pub enum AudioIntelligenceError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Audio capture error: {0}")]
    AudioCapture(String),

    #[error("Transcription error: {0}")]
    Transcription(String),

    #[error("Feature not enabled: {0}")]
    FeatureDisabled(&'static str),

    #[error("Memory storage error: {0}")]
    MemoryStorage(String),
}

/// Audio chunk for ring buffer storage
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioChunk {
    pub timestamp: i64,
    pub duration_ms: u32,
    pub data: Vec<u8>, // Compressed audio data (OPUS)
    pub sample_rate: u32,
    pub channels: u16,
}

/// Meeting transcript with speaker diarization
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MeetingTranscript {
    pub session_id: String,
    pub start_time: i64,
    pub end_time: i64,
    pub participants: Vec<Participant>,
    pub segments: Vec<TranscriptSegment>,
    pub summary: String,
    pub keywords: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Participant {
    pub speaker_id: String,
    pub name: Option<String>,
    pub segments: Vec<TranscriptSegment>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub speaker_id: String,
    pub start_time: f64,
    pub end_time: f64,
    pub text: String,
    pub confidence: f32,
}

/// Audio analysis results
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioAnalysis {
    pub tone: ToneAnalysis,
    pub sentiment: SentimentAnalysis,
    pub keywords: Vec<Keyword>,
    pub voice_activity: f32, // 0.0-1.0
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToneAnalysis {
    pub energy: f32,     // 0.0-1.0
    pub pitch: f32,      // Hz
    pub tempo: f32,      // BPM
    pub emotion: String, // "calm", "excited", "stressed", etc.
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SentimentAnalysis {
    pub positive: f32,   // 0.0-1.0
    pub negative: f32,   // 0.0-1.0
    pub neutral: f32,    // 0.0-1.0
    pub overall: String, // "positive", "negative", "neutral"
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Keyword {
    pub word: String,
    pub frequency: u32,
    pub importance: f32, // 0.0-1.0
}

/// Main Audio Intelligence manager
pub struct AudioIntelligence {
    // Configuration
    ring_buffer_size: usize, // Number of chunks (30s buffer = ~30 chunks at 1s each)
    wake_word: String,
    vad_threshold: f32,

    // State
    ring_buffer: Arc<Mutex<Vec<AudioChunk>>>,
    is_listening: Arc<AtomicBool>,
    is_recording: Arc<AtomicBool>,

    // Memory integration
    neural_cortex: Arc<NeuralCortexStrata>,
    vaults: Arc<VitalOrganVaults>,

    // Current session
    current_session: Arc<Mutex<Option<RecordingSession>>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RecordingSession {
    session_id: String,
    start_time: i64,
    chunks: Vec<AudioChunk>,
    transcript: Option<MeetingTranscript>,
    analysis: Option<AudioAnalysis>,
}

impl AudioIntelligence {
    /// Create a new AudioIntelligence instance
    pub fn new(neural_cortex: Arc<NeuralCortexStrata>, vaults: Arc<VitalOrganVaults>) -> Self {
        let ring_buffer_size = std::env::var("AUDIO_RING_BUFFER_SIZE")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(30); // 30 seconds at 1 chunk/second

        let wake_word = std::env::var("WAKE_WORD").unwrap_or_else(|_| "Phoenix".to_string());

        let vad_threshold = std::env::var("VAD_THRESHOLD")
            .ok()
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.5);

        Self {
            ring_buffer_size,
            wake_word,
            vad_threshold,
            ring_buffer: Arc::new(Mutex::new(Vec::with_capacity(ring_buffer_size))),
            is_listening: Arc::new(AtomicBool::new(false)),
            is_recording: Arc::new(AtomicBool::new(false)),
            neural_cortex,
            vaults,
            current_session: Arc::new(Mutex::new(None)),
        }
    }

    /// Returns the configured voice-activity-detection threshold.
    pub fn vad_threshold(&self) -> f32 {
        self.vad_threshold
    }

    /// Start continuous ambient listening
    ///
    /// This maintains a ring buffer of the last N seconds of audio
    /// and triggers recording when wake word is detected.
    pub async fn start_ambient_listening(&self) -> Result<(), AudioIntelligenceError> {
        if self.is_listening.load(Ordering::Relaxed) {
            return Ok(()); // Already listening
        }

        self.is_listening.store(true, Ordering::Relaxed);

        // Spawn background task for continuous listening
        let ring_buffer = self.ring_buffer.clone();
        let is_listening = self.is_listening.clone();
        let _wake_word = self.wake_word.clone();
        let buffer_size = self.ring_buffer_size;

        tokio::spawn(async move {
            while is_listening.load(Ordering::Relaxed) {
                // TODO: Implement actual audio capture
                // For now, this is a placeholder

                // Simulate audio chunk capture
                let chunk = AudioChunk {
                    timestamp: Utc::now().timestamp(),
                    duration_ms: 1000,
                    data: Vec::new(),
                    sample_rate: 16000,
                    channels: 1,
                };

                // Add to ring buffer
                let mut buffer = ring_buffer.lock().await;
                buffer.push(chunk);

                // Maintain ring buffer size
                if buffer.len() > buffer_size {
                    buffer.remove(0);
                }

                // TODO: Check for wake word
                // TODO: Trigger recording if wake word detected

                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        });

        Ok(())
    }

    /// Stop ambient listening
    pub fn stop_listening(&self) {
        self.is_listening.store(false, Ordering::Relaxed);
    }

    /// Start recording a meeting/session
    pub async fn start_recording(
        &self,
        _purpose: Option<String>,
    ) -> Result<String, AudioIntelligenceError> {
        if self.is_recording.load(Ordering::Relaxed) {
            return Err(AudioIntelligenceError::AudioCapture(
                "Already recording".to_string(),
            ));
        }

        self.is_recording.store(true, Ordering::Relaxed);

        let session_id = Uuid::new_v4().to_string();
        let start_time = Utc::now().timestamp();
        let session = RecordingSession {
            session_id: session_id.clone(),
            start_time,
            chunks: Vec::new(),
            transcript: None,
            analysis: None,
        };

        // Store in L2 working memory before moving
        let key = format!("wm:sensory:screen:{}", start_time);
        let value = serde_json::to_string(&session).map_err(|e| {
            AudioIntelligenceError::MemoryStorage(format!("JSON serialization failed: {}", e))
        })?;

        self.neural_cortex
            .etch(MemoryLayer::WM(value), &key)
            .map_err(|e| {
                AudioIntelligenceError::MemoryStorage(format!("Failed to store in WM: {}", e))
            })?;

        *self.current_session.lock().await = Some(session);

        Ok(session_id)
    }

    /// Stop recording and process
    pub async fn stop_recording(&self) -> Result<MeetingTranscript, AudioIntelligenceError> {
        if !self.is_recording.load(Ordering::Relaxed) {
            return Err(AudioIntelligenceError::AudioCapture(
                "Not currently recording".to_string(),
            ));
        }

        self.is_recording.store(false, Ordering::Relaxed);

        let mut session =
            self.current_session.lock().await.take().ok_or_else(|| {
                AudioIntelligenceError::AudioCapture("No active session".to_string())
            })?;

        session.chunks = self.ring_buffer.lock().await.clone();

        // TODO: Transcribe with Whisper
        // TODO: Diarize speakers
        // TODO: Analyze audio

        // For now, create placeholder transcript
        let transcript = MeetingTranscript {
            session_id: session.session_id.clone(),
            start_time: session.start_time,
            end_time: Utc::now().timestamp(),
            participants: Vec::new(),
            segments: Vec::new(),
            summary: "Meeting transcription placeholder".to_string(),
            keywords: Vec::new(),
        };

        session.transcript = Some(transcript.clone());

        // Store in L3 episodic memory
        let start_time = session.start_time;
        let session_clone = session.clone();
        let key = format!("epm:sensory:session:{}", start_time);
        let value = serde_json::to_string(&session_clone).map_err(|e| {
            AudioIntelligenceError::MemoryStorage(format!("JSON serialization failed: {}", e))
        })?;

        self.neural_cortex
            .etch(MemoryLayer::EPM(value), &key)
            .map_err(|e| {
                AudioIntelligenceError::MemoryStorage(format!("Failed to store in EPM: {}", e))
            })?;

        Ok(transcript)
    }

    /// Process meeting audio and generate transcript
    pub async fn process_meeting(
        &self,
        _audio_path: PathBuf,
    ) -> Result<MeetingTranscript, AudioIntelligenceError> {
        // TODO: Implement Whisper transcription
        // TODO: Implement speaker diarization (pyannote.audio via Python bridge)
        // TODO: Generate summary
        // TODO: Extract keywords

        // Placeholder implementation
        let transcript = MeetingTranscript {
            session_id: Uuid::new_v4().to_string(),
            start_time: Utc::now().timestamp(),
            end_time: Utc::now().timestamp(),
            participants: Vec::new(),
            segments: Vec::new(),
            summary: "Meeting transcription not yet implemented".to_string(),
            keywords: Vec::new(),
        };

        // Store in L3 episodic memory
        let key = format!("epm:sensory:session:{}", transcript.start_time);
        let value = serde_json::to_string(&transcript).map_err(|e| {
            AudioIntelligenceError::MemoryStorage(format!("JSON serialization failed: {}", e))
        })?;

        self.neural_cortex
            .etch(MemoryLayer::EPM(value), &key)
            .map_err(|e| {
                AudioIntelligenceError::MemoryStorage(format!("Failed to store in EPM: {}", e))
            })?;

        Ok(transcript)
    }

    /// Analyze audio for tone, sentiment, and keywords
    pub async fn analyze_audio(
        &self,
        audio_chunk: &AudioChunk,
    ) -> Result<AudioAnalysis, AudioIntelligenceError> {
        // TODO: Implement tone analysis
        // TODO: Implement sentiment detection
        // TODO: Extract keywords

        // Placeholder implementation
        let analysis = AudioAnalysis {
            tone: ToneAnalysis {
                energy: 0.5,
                pitch: 200.0,
                tempo: 120.0,
                emotion: "neutral".to_string(),
            },
            sentiment: SentimentAnalysis {
                positive: 0.5,
                negative: 0.3,
                neutral: 0.2,
                overall: "neutral".to_string(),
            },
            keywords: Vec::new(),
            voice_activity: 0.5,
        };

        // Store extracted knowledge in L4 semantic memory (Mind Vault)
        let key = format!(
            "mind:sensory:extracted:audio_analysis:{}",
            audio_chunk.timestamp
        );
        let value = serde_json::to_string(&analysis).map_err(|e| {
            AudioIntelligenceError::MemoryStorage(format!("JSON serialization failed: {}", e))
        })?;

        self.vaults.store_mind(&key, &value).map_err(|e| {
            AudioIntelligenceError::MemoryStorage(format!("Failed to store in Mind Vault: {}", e))
        })?;

        Ok(analysis)
    }

    /// Get current ring buffer contents (L1 instant cache)
    pub async fn get_ring_buffer(&self) -> Vec<AudioChunk> {
        self.ring_buffer.lock().await.clone()
    }

    /// Check if currently listening
    pub fn is_listening(&self) -> bool {
        self.is_listening.load(Ordering::Relaxed)
    }

    /// Check if currently recording
    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::Relaxed)
    }
}
