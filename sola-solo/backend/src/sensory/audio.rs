//! Audio capture and voice activity detection using CPAL and Silero VAD.
//!
//! This module provides real-time audio capture from the default input device
//! and runs voice activity detection to identify speech segments.

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "sensory")]
use std::sync::Mutex;

#[cfg(feature = "sensory")]
use tokio::sync::watch;

/// Voice activity detection result.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VoiceActivity {
    /// Whether speech was detected in the latest chunk.
    pub speech_detected: bool,
    /// Confidence score (0.0 to 1.0).
    pub confidence: f32,
    /// Timestamp in milliseconds since UNIX epoch.
    pub timestamp_ms: u64,
}

/// Status of the audio pipeline.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioStatus {
    /// Whether the audio capture is running.
    pub running: bool,
    /// Sample rate in Hz.
    pub sample_rate: u32,
    /// Number of audio channels.
    pub channels: u16,
    /// Last error message, if any.
    pub last_error: Option<String>,
    /// Total number of speech events detected.
    pub speech_events: u64,
}

/// Audio capture and VAD pipeline.
///
/// Uses CPAL for audio input and Silero VAD for voice activity detection.
pub struct AudioPipeline {
    running: Arc<AtomicBool>,
    speech_events: Arc<AtomicU64>,
    status: AudioStatus,
    #[cfg(feature = "sensory")]
    vad_rx: Option<watch::Receiver<VoiceActivity>>,
    #[cfg(feature = "sensory")]
    _stream: Option<cpal::Stream>,
    /// Shared buffer for capturing audio samples (16kHz mono)
    #[cfg(feature = "sensory")]
    capture_buffer: Arc<Mutex<Vec<f32>>>,
    /// Flag to enable/disable capture buffer recording
    #[cfg(feature = "sensory")]
    capture_enabled: Arc<AtomicBool>,
}

impl AudioPipeline {
    /// Create a new audio pipeline (not yet started).
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            speech_events: Arc::new(AtomicU64::new(0)),
            status: AudioStatus::default(),
            #[cfg(feature = "sensory")]
            vad_rx: None,
            #[cfg(feature = "sensory")]
            _stream: None,
            #[cfg(feature = "sensory")]
            capture_buffer: Arc::new(Mutex::new(Vec::with_capacity(16000 * 10))), // 10 seconds
            #[cfg(feature = "sensory")]
            capture_enabled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Start the audio capture and VAD pipeline.
    #[cfg(feature = "sensory")]
    pub fn start(&mut self) -> anyhow::Result<()> {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        use voice_activity_detector::{VoiceActivityDetector, VoiceActivityDetectorBuilder};

        if self.running.load(Ordering::SeqCst) {
            return Ok(()); // Already running
        }

        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;

        let config = device.default_input_config()?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels();

        // Silero VAD expects 16kHz mono audio
        // We'll resample if needed
        let target_sample_rate = 16000;

        // Create VAD detector
        let vad = VoiceActivityDetectorBuilder::default()
            .sample_rate(target_sample_rate)
            .chunk_size(512usize) // ~32ms at 16kHz
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to create VAD: {:?}", e))?;

        let vad = Arc::new(Mutex::new(vad));

        let (vad_tx, vad_rx) = watch::channel(VoiceActivity::default());
        self.vad_rx = Some(vad_rx);

        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        let speech_events = self.speech_events.clone();

        // Audio buffer for resampling and chunking
        let audio_buffer: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::with_capacity(4096)));
        let resample_ratio = target_sample_rate as f32 / sample_rate as f32;

        let vad_clone = vad.clone();
        let buffer_clone = audio_buffer.clone();
        let capture_buffer_clone = self.capture_buffer.clone();
        let capture_enabled_clone = self.capture_enabled.clone();

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if !running.load(Ordering::SeqCst) {
                    return;
                }

                // Convert to mono if stereo
                let mono: Vec<f32> = if channels > 1 {
                    data.chunks(channels as usize)
                        .map(|chunk| chunk.iter().sum::<f32>() / channels as f32)
                        .collect()
                } else {
                    data.to_vec()
                };

                // Simple linear resampling (good enough for VAD)
                let resampled: Vec<f32> = if (resample_ratio - 1.0).abs() > 0.01 {
                    let output_len = (mono.len() as f32 * resample_ratio) as usize;
                    (0..output_len)
                        .map(|i| {
                            let src_idx = (i as f32 / resample_ratio) as usize;
                            mono.get(src_idx).copied().unwrap_or(0.0)
                        })
                        .collect()
                } else {
                    mono
                };

                // Add to buffer
                let mut buffer = buffer_clone.lock().unwrap();
                buffer.extend(resampled.iter().copied());

                // Also add to capture buffer if enabled
                if capture_enabled_clone.load(Ordering::SeqCst) {
                    if let Ok(mut capture_buf) = capture_buffer_clone.lock() {
                        capture_buf.extend(resampled.iter().copied());
                    }
                }

                // Process in 512-sample chunks
                while buffer.len() >= 512 {
                    let chunk: Vec<f32> = buffer.drain(..512).collect();

                    // Run VAD
                    let mut vad = vad_clone.lock().unwrap();
                    match vad.predict(chunk.iter().copied()) {
                        Ok(probability) => {
                            let speech_detected = probability > 0.5;

                            if speech_detected {
                                speech_events.fetch_add(1, Ordering::SeqCst);
                            }

                            let timestamp_ms = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .map(|d| d.as_millis() as u64)
                                .unwrap_or(0);

                            let activity = VoiceActivity {
                                speech_detected,
                                confidence: probability,
                                timestamp_ms,
                            };

                            let _ = vad_tx.send(activity);
                        }
                        Err(e) => {
                            eprintln!("[audio] VAD prediction error: {:?}", e);
                        }
                    }
                }
            },
            move |err| {
                eprintln!("[audio] Stream error: {}", err);
            },
            None, // No timeout
        )?;

        stream.play()?;
        self._stream = Some(stream);

        self.status.running = true;
        self.status.sample_rate = sample_rate;
        self.status.channels = channels;
        self.status.last_error = None;

        Ok(())
    }

    /// Start the audio pipeline (no-op when sensory feature is disabled).
    #[cfg(not(feature = "sensory"))]
    pub fn start(&mut self) -> anyhow::Result<()> {
        self.status.last_error = Some("sensory feature not enabled".to_string());
        Ok(())
    }

    /// Stop the audio capture.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        #[cfg(feature = "sensory")]
        {
            self._stream = None;
        }
        self.status.running = false;
    }

    /// Get the current audio pipeline status.
    pub fn status(&self) -> AudioStatus {
        let mut status = self.status.clone();
        status.speech_events = self.speech_events.load(Ordering::SeqCst);
        status
    }

    /// Get the latest VAD result.
    #[cfg(feature = "sensory")]
    pub fn latest_vad(&self) -> VoiceActivity {
        self.vad_rx
            .as_ref()
            .map(|rx| rx.borrow().clone())
            .unwrap_or_default()
    }

    /// Get the latest VAD result (empty when sensory feature is disabled).
    #[cfg(not(feature = "sensory"))]
    pub fn latest_vad(&self) -> VoiceActivity {
        VoiceActivity::default()
    }

    /// Start capturing audio samples to the internal buffer.
    #[cfg(feature = "sensory")]
    pub fn start_capture(&self) {
        // Clear existing buffer
        if let Ok(mut buf) = self.capture_buffer.lock() {
            buf.clear();
        }
        self.capture_enabled.store(true, Ordering::SeqCst);
    }

    /// Start capturing audio samples (no-op when sensory feature is disabled).
    #[cfg(not(feature = "sensory"))]
    pub fn start_capture(&self) {}

    /// Stop capturing audio samples.
    #[cfg(feature = "sensory")]
    pub fn stop_capture(&self) {
        self.capture_enabled.store(false, Ordering::SeqCst);
    }

    /// Stop capturing audio samples (no-op when sensory feature is disabled).
    #[cfg(not(feature = "sensory"))]
    pub fn stop_capture(&self) {}

    /// Get the captured audio samples (16kHz mono).
    #[cfg(feature = "sensory")]
    pub fn get_captured_samples(&self) -> Vec<f32> {
        self.capture_buffer
            .lock()
            .map(|buf| buf.clone())
            .unwrap_or_default()
    }

    /// Get the captured audio samples (empty when sensory feature is disabled).
    #[cfg(not(feature = "sensory"))]
    pub fn get_captured_samples(&self) -> Vec<f32> {
        Vec::new()
    }
}

impl Default for AudioPipeline {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Voiceprint Extraction (Speaker Identification)
// ============================================================================

/// Path to the speaker encoder ONNX model.
pub const SPEAKER_ENCODER_PATH: &str = "sola-solo/backend/models/speaker_encoder_resnet34.onnx";

/// Voiceprint embedding result.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VoiceprintEmbedding {
    /// 512-dimensional embedding vector
    pub embedding: Vec<f32>,
    /// Duration of audio used for extraction (in seconds)
    pub duration_secs: f32,
    /// Timestamp when embedding was computed
    pub timestamp_ms: u64,
    /// Confidence score (0.0 to 1.0) based on audio quality
    pub confidence: f32,
}

/// Mel-spectrogram parameters for speaker recognition.
pub struct MelSpectrogramConfig {
    /// Sample rate (Hz)
    pub sample_rate: u32,
    /// FFT window size
    pub n_fft: usize,
    /// Hop length between frames
    pub hop_length: usize,
    /// Number of Mel filter banks
    pub n_mels: usize,
    /// Minimum frequency (Hz)
    pub f_min: f32,
    /// Maximum frequency (Hz)
    pub f_max: f32,
}

impl Default for MelSpectrogramConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            n_fft: 512,
            hop_length: 160, // 10ms at 16kHz
            n_mels: 80,
            f_min: 20.0,
            f_max: 8000.0,
        }
    }
}

/// Generate a Hanning window for FFT.
#[cfg(feature = "sensory")]
fn hanning_window(size: usize) -> Vec<f32> {
    (0..size)
        .map(|i| {
            0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (size - 1) as f32).cos())
        })
        .collect()
}

/// Generate Mel filter bank matrix.
#[cfg(feature = "sensory")]
fn mel_filter_bank(config: &MelSpectrogramConfig) -> Vec<Vec<f32>> {
    let n_fft_bins = config.n_fft / 2 + 1;

    // Convert Hz to Mel scale
    let hz_to_mel = |hz: f32| 2595.0 * (1.0 + hz / 700.0).log10();
    let mel_to_hz = |mel: f32| 700.0 * (10.0_f32.powf(mel / 2595.0) - 1.0);

    let mel_min = hz_to_mel(config.f_min);
    let mel_max = hz_to_mel(config.f_max);

    // Create equally spaced Mel points
    let mel_points: Vec<f32> = (0..=config.n_mels + 1)
        .map(|i| mel_min + (mel_max - mel_min) * i as f32 / (config.n_mels + 1) as f32)
        .collect();

    // Convert back to Hz and then to FFT bin indices
    let hz_points: Vec<f32> = mel_points.iter().map(|&m| mel_to_hz(m)).collect();
    let bin_points: Vec<usize> = hz_points
        .iter()
        .map(|&hz| ((config.n_fft as f32 + 1.0) * hz / config.sample_rate as f32) as usize)
        .collect();

    // Create triangular filters
    let mut filters = vec![vec![0.0; n_fft_bins]; config.n_mels];

    for m in 0..config.n_mels {
        let left = bin_points[m];
        let center = bin_points[m + 1];
        let right = bin_points[m + 2];

        // Rising slope
        for k in left..center {
            if k < n_fft_bins && center > left {
                filters[m][k] = (k - left) as f32 / (center - left) as f32;
            }
        }

        // Falling slope
        for k in center..right {
            if k < n_fft_bins && right > center {
                filters[m][k] = (right - k) as f32 / (right - center) as f32;
            }
        }
    }

    filters
}

/// Compute Mel-spectrogram from audio samples.
///
/// # Arguments
/// * `samples` - Audio samples at 16kHz mono
/// * `config` - Mel-spectrogram configuration
///
/// # Returns
/// 2D array of shape (n_mels, n_frames) containing log-Mel energies
#[cfg(feature = "sensory")]
pub fn compute_mel_spectrogram(samples: &[f32], config: &MelSpectrogramConfig) -> Vec<Vec<f32>> {
    use rustfft::{num_complex::Complex, FftPlanner};

    let window = hanning_window(config.n_fft);
    let mel_filters = mel_filter_bank(config);

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(config.n_fft);

    let n_frames = (samples.len().saturating_sub(config.n_fft)) / config.hop_length + 1;
    let mut spectrogram = vec![vec![0.0; n_frames]; config.n_mels];

    for frame_idx in 0..n_frames {
        let start = frame_idx * config.hop_length;
        let end = (start + config.n_fft).min(samples.len());

        // Apply window and prepare FFT input
        let mut fft_input: Vec<Complex<f32>> = (0..config.n_fft)
            .map(|i| {
                let sample = if start + i < end {
                    samples[start + i] * window[i]
                } else {
                    0.0
                };
                Complex::new(sample, 0.0)
            })
            .collect();

        // Compute FFT
        fft.process(&mut fft_input);

        // Compute power spectrum (only positive frequencies)
        let power_spectrum: Vec<f32> = fft_input
            .iter()
            .take(config.n_fft / 2 + 1)
            .map(|c| c.norm_sqr())
            .collect();

        // Apply Mel filter bank
        for (mel_idx, filter) in mel_filters.iter().enumerate() {
            let mel_energy: f32 = filter
                .iter()
                .zip(power_spectrum.iter())
                .map(|(f, p)| f * p)
                .sum();

            // Log compression (add small epsilon to avoid log(0))
            spectrogram[mel_idx][frame_idx] = (mel_energy + 1e-10).ln();
        }
    }

    spectrogram
}

/// Speaker encoder using OpenCV DNN.
#[cfg(feature = "sensory")]
pub struct SpeakerEncoder {
    net: opencv::dnn::Net,
}

#[cfg(feature = "sensory")]
impl SpeakerEncoder {
    /// Create a new speaker encoder by loading the ONNX model.
    pub fn new() -> anyhow::Result<Self> {
        use opencv::dnn::read_net_from_onnx;

        if !std::path::Path::new(SPEAKER_ENCODER_PATH).exists() {
            anyhow::bail!(
                "Speaker encoder model not found at {}. See models/README.md for download instructions.",
                SPEAKER_ENCODER_PATH
            );
        }

        let net = read_net_from_onnx(SPEAKER_ENCODER_PATH)?;

        Ok(Self { net })
    }

    /// Extract speaker embedding from Mel-spectrogram.
    ///
    /// # Arguments
    /// * `mel_spec` - Mel-spectrogram of shape (n_mels, n_frames)
    ///
    /// # Returns
    /// 512-dimensional speaker embedding
    pub fn extract_embedding(&mut self, mel_spec: &[Vec<f32>]) -> anyhow::Result<Vec<f32>> {
        use opencv::core::{Mat, Scalar, Size, CV_32F};
        use opencv::dnn::blob_from_image;
        use opencv::prelude::*;

        let n_mels = mel_spec.len();
        let n_frames = mel_spec.first().map(|v| v.len()).unwrap_or(0);

        if n_mels == 0 || n_frames == 0 {
            anyhow::bail!("Empty mel-spectrogram");
        }

        // Create Mat from mel-spectrogram (treat as grayscale image)
        let mut mat = Mat::new_rows_cols_with_default(
            n_mels as i32,
            n_frames as i32,
            CV_32F,
            Scalar::all(0.0),
        )?;

        for (i, row) in mel_spec.iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                *mat.at_2d_mut::<f32>(i as i32, j as i32)? = val;
            }
        }

        // Normalize to [0, 1] range
        let min_val = mel_spec.iter().flatten().cloned().fold(f32::INFINITY, f32::min);
        let max_val = mel_spec.iter().flatten().cloned().fold(f32::NEG_INFINITY, f32::max);
        let range = max_val - min_val;

        if range > 0.0 {
            for i in 0..n_mels {
                for j in 0..n_frames {
                    let val = mat.at_2d::<f32>(i as i32, j as i32)?;
                    *mat.at_2d_mut::<f32>(i as i32, j as i32)? = (*val - min_val) / range;
                }
            }
        }

        // Create blob (resize to expected input size if needed)
        // Most speaker encoders expect fixed-size input (e.g., 80x300)
        let target_frames = 300;
        let blob = blob_from_image(
            &mat,
            1.0,
            Size::new(target_frames, n_mels as i32),
            Scalar::all(0.0),
            false,
            false,
            CV_32F,
        )?;

        // Run inference
        self.net.set_input(&blob, "", 1.0, Scalar::all(0.0))?;
        let output = self.net.forward("")?;

        // Extract embedding from output
        let mut embedding = Vec::with_capacity(512);
        let cols = output.cols().min(512);
        for i in 0..cols {
            embedding.push(*output.at_2d::<f32>(0, i)?);
        }

        // Pad to 512 dimensions if needed
        while embedding.len() < 512 {
            embedding.push(0.0);
        }

        // L2 normalize the embedding
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for val in &mut embedding {
                *val /= norm;
            }
        }

        Ok(embedding)
    }
}

/// Extract voiceprint embedding from audio samples.
///
/// This function:
/// 1. Computes a Mel-spectrogram from the audio
/// 2. Runs the spectrogram through a speaker encoder model
/// 3. Returns a 512-dimensional speaker embedding
///
/// # Arguments
/// * `samples` - Audio samples at 16kHz mono
///
/// # Returns
/// A VoiceprintEmbedding with the speaker embedding
#[cfg(feature = "sensory")]
pub fn extract_voiceprint(samples: &[f32]) -> VoiceprintEmbedding {
    let duration_secs = samples.len() as f32 / 16000.0;

    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    // Require at least 1 second of audio
    if samples.len() < 16000 {
        return VoiceprintEmbedding {
            embedding: vec![0.0; 512],
            duration_secs,
            timestamp_ms,
            confidence: 0.0,
        };
    }

    // Compute Mel-spectrogram
    let config = MelSpectrogramConfig::default();
    let mel_spec = compute_mel_spectrogram(samples, &config);

    // Try to extract embedding using speaker encoder
    let (embedding, confidence) = match SpeakerEncoder::new() {
        Ok(mut encoder) => match encoder.extract_embedding(&mel_spec) {
            Ok(emb) => (emb, 0.9), // High confidence if model succeeds
            Err(e) => {
                eprintln!("[audio] Speaker encoder inference failed: {}", e);
                (vec![0.0; 512], 0.0)
            }
        },
        Err(e) => {
            eprintln!("[audio] Failed to load speaker encoder: {}", e);
            (vec![0.0; 512], 0.0)
        }
    };

    VoiceprintEmbedding {
        embedding,
        duration_secs,
        timestamp_ms,
        confidence,
    }
}

/// Extract voiceprint embedding (stub when sensory feature is disabled).
#[cfg(not(feature = "sensory"))]
pub fn extract_voiceprint(samples: &[f32]) -> VoiceprintEmbedding {
    let duration_secs = samples.len() as f32 / 16000.0;

    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    VoiceprintEmbedding {
        embedding: vec![0.0; 512],
        duration_secs,
        timestamp_ms,
        confidence: 0.0,
    }
}
