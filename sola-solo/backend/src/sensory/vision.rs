//! Camera capture worker using OpenCV.
//!
//! This module provides non-blocking camera frame capture using a dedicated
//! thread and `tokio::sync::watch` channels for broadcasting frames.
//!
//! ## Face Recognition Pipeline
//!
//! When the `sensory` feature is enabled, this module also provides:
//! - Face detection using YuNet
//! - Face embedding extraction using SFace
//! - 128-dimensional embeddings (padded to 512 for Qdrant compatibility)

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[cfg(feature = "sensory")]
use std::thread;

#[cfg(feature = "sensory")]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(feature = "sensory")]
use tokio::sync::watch;

/// A captured camera frame with JPEG-encoded data.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CameraFrame {
    /// JPEG-encoded image data.
    pub jpeg_data: Vec<u8>,
    /// Timestamp in milliseconds since UNIX epoch.
    pub timestamp_ms: u64,
    /// Frame width in pixels.
    pub width: u32,
    /// Frame height in pixels.
    pub height: u32,
}

/// Status of the camera worker.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CameraStatus {
    /// Whether the camera capture loop is running.
    pub running: bool,
    /// Camera device index (e.g., 0 for default webcam).
    pub camera_index: i32,
    /// Approximate frames per second.
    pub fps: f32,
    /// Last error message, if any.
    pub last_error: Option<String>,
}

/// Detected face with bounding box and landmarks.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DetectedFace {
    /// Bounding box: (x, y, width, height)
    pub bbox: (f32, f32, f32, f32),
    /// Confidence score (0.0 to 1.0)
    pub confidence: f32,
    /// 5-point facial landmarks (eyes, nose, mouth corners)
    pub landmarks: Vec<(f32, f32)>,
}

/// Face embedding result.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FaceEmbedding {
    /// 512-dimensional embedding vector (128-dim from SFace, padded)
    pub embedding: Vec<f32>,
    /// Confidence of the face detection
    pub detection_confidence: f32,
    /// Timestamp when embedding was computed
    pub timestamp_ms: u64,
}

/// Liveness detection result.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LivenessResult {
    /// Whether liveness was detected (blink or head movement)
    pub is_live: bool,
    /// Number of blinks detected during capture
    pub blinks_detected: u32,
    /// Eye aspect ratio variance (indicates eye movement)
    pub ear_variance: f32,
    /// Head pose variance (indicates head movement)
    pub head_pose_variance: f32,
    /// Confidence score for liveness (0.0 to 1.0)
    pub confidence: f32,
    /// Human-readable message
    pub message: String,
}

/// Security error types for enrollment failures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityError {
    /// Liveness detection failed - possible spoofing attempt
    LivenessFailed { reason: String },
    /// No face detected during enrollment
    NoFaceDetected,
    /// Face detection confidence too low
    LowConfidence { confidence: f32 },
    /// Camera not available
    CameraUnavailable,
}

/// Camera capture worker.
///
/// Uses a dedicated thread to capture frames from OpenCV's VideoCapture
/// and broadcasts them via a watch channel.
pub struct CameraWorker {
    running: Arc<AtomicBool>,
    status: CameraStatus,
    #[cfg(feature = "sensory")]
    frame_rx: Option<watch::Receiver<CameraFrame>>,
    #[cfg(feature = "sensory")]
    _capture_thread: Option<thread::JoinHandle<()>>,
}

impl CameraWorker {
    /// Create a new camera worker (not yet started).
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            status: CameraStatus::default(),
            #[cfg(feature = "sensory")]
            frame_rx: None,
            #[cfg(feature = "sensory")]
            _capture_thread: None,
        }
    }

    /// Start capturing from the specified camera index.
    ///
    /// Returns an error if the camera cannot be opened.
    #[cfg(feature = "sensory")]
    pub fn start(&mut self, camera_index: i32) -> anyhow::Result<()> {
        use opencv::prelude::*;
        use opencv::videoio::{VideoCapture, CAP_ANY};

        if self.running.load(Ordering::SeqCst) {
            return Ok(()); // Already running
        }

        // Test camera access before spawning thread
        let mut test_cap = VideoCapture::new(camera_index, CAP_ANY)?;
        if !test_cap.is_opened()? {
            anyhow::bail!("Failed to open camera {}", camera_index);
        }
        drop(test_cap);

        let (frame_tx, frame_rx) = watch::channel(CameraFrame::default());
        self.frame_rx = Some(frame_rx);

        let running = self.running.clone();
        running.store(true, Ordering::SeqCst);

        let handle = thread::spawn(move || {
            capture_loop(camera_index, running, frame_tx);
        });

        self._capture_thread = Some(handle);
        self.status.running = true;
        self.status.camera_index = camera_index;
        self.status.last_error = None;

        Ok(())
    }

    /// Start capturing (no-op when sensory feature is disabled).
    #[cfg(not(feature = "sensory"))]
    pub fn start(&mut self, camera_index: i32) -> anyhow::Result<()> {
        self.status.camera_index = camera_index;
        self.status.last_error = Some("sensory feature not enabled".to_string());
        Ok(())
    }

    /// Stop the camera capture loop.
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        self.status.running = false;
    }

    /// Get the current camera status.
    pub fn status(&self) -> CameraStatus {
        self.status.clone()
    }

    /// Get the latest captured frame.
    #[cfg(feature = "sensory")]
    pub fn latest_frame(&self) -> CameraFrame {
        self.frame_rx
            .as_ref()
            .map(|rx| rx.borrow().clone())
            .unwrap_or_default()
    }

    /// Get the latest captured frame (empty when sensory feature is disabled).
    #[cfg(not(feature = "sensory"))]
    pub fn latest_frame(&self) -> CameraFrame {
        CameraFrame::default()
    }
}

impl Default for CameraWorker {
    fn default() -> Self {
        Self::new()
    }
}

/// Internal capture loop running in a dedicated thread.
#[cfg(feature = "sensory")]
fn capture_loop(
    camera_index: i32,
    running: Arc<AtomicBool>,
    frame_tx: watch::Sender<CameraFrame>,
) {
    use image::codecs::jpeg::JpegEncoder;
    use image::{ImageBuffer, Rgb};
    use opencv::core::{Mat, Vector};
    use opencv::imgproc;
    use opencv::prelude::*;
    use opencv::videoio::{VideoCapture, CAP_ANY};

    let mut cap = match VideoCapture::new(camera_index, CAP_ANY) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[vision] Failed to open camera: {}", e);
            return;
        }
    };

    let mut frame_mat = Mat::default();
    let mut rgb_mat = Mat::default();

    while running.load(Ordering::SeqCst) {
        // Read frame (blocking)
        match cap.read(&mut frame_mat) {
            Ok(true) => {}
            Ok(false) => {
                // No frame available, sleep briefly
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }
            Err(e) => {
                eprintln!("[vision] Frame read error: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(100));
                continue;
            }
        }

        // Convert BGR to RGB
        if let Err(e) = imgproc::cvt_color(&frame_mat, &mut rgb_mat, imgproc::COLOR_BGR2RGB, 0) {
            eprintln!("[vision] Color conversion error: {}", e);
            continue;
        }

        let width = rgb_mat.cols() as u32;
        let height = rgb_mat.rows() as u32;

        // Extract raw pixel data
        let data: Vector<u8> = match rgb_mat.data_bytes() {
            Ok(d) => Vector::from_slice(d),
            Err(e) => {
                eprintln!("[vision] Data extraction error: {}", e);
                continue;
            }
        };

        // Create image buffer and encode to JPEG
        let img_buf: ImageBuffer<Rgb<u8>, Vec<u8>> =
            match ImageBuffer::from_raw(width, height, data.to_vec()) {
                Some(buf) => buf,
                None => {
                    eprintln!("[vision] Failed to create image buffer");
                    continue;
                }
            };

        let mut jpeg_data = Vec::new();
        {
            let mut encoder = JpegEncoder::new_with_quality(&mut jpeg_data, 80);
            if let Err(e) = encoder.encode_image(&img_buf) {
                eprintln!("[vision] JPEG encoding error: {}", e);
                continue;
            }
        }

        let timestamp_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);

        let frame = CameraFrame {
            jpeg_data,
            timestamp_ms,
            width,
            height,
        };

        // Broadcast frame (ignore if no receivers)
        let _ = frame_tx.send(frame);

        // Target ~30 FPS
        std::thread::sleep(std::time::Duration::from_millis(33));
    }

    eprintln!("[vision] Capture loop stopped");
}

// ============================================================================
// Face Detection and Recognition (sensory feature)
// ============================================================================

/// Path to the YuNet face detection model.
pub const YUNET_MODEL_PATH: &str = "sola-solo/backend/models/face_detection_yunet_2023mar.onnx";

/// Path to the SFace face recognition model.
pub const SFACE_MODEL_PATH: &str = "sola-solo/backend/models/face_recognition_sface_2021dec.onnx";

/// Face recognition pipeline using OpenCV DNN.
///
/// This struct holds the loaded models and provides methods for
/// face detection and embedding extraction.
#[cfg(feature = "sensory")]
pub struct FaceRecognizer {
    detector: opencv::objdetect::FaceDetectorYN,
    recognizer: opencv::objdetect::FaceRecognizerSF,
}

#[cfg(feature = "sensory")]
impl FaceRecognizer {
    /// Create a new face recognizer by loading the models.
    ///
    /// Returns an error if the model files are not found.
    pub fn new() -> anyhow::Result<Self> {
        use opencv::objdetect::{FaceDetectorYN, FaceRecognizerSF};

        // Check if model files exist
        if !std::path::Path::new(YUNET_MODEL_PATH).exists() {
            anyhow::bail!(
                "YuNet model not found at {}. See models/README.md for download instructions.",
                YUNET_MODEL_PATH
            );
        }
        if !std::path::Path::new(SFACE_MODEL_PATH).exists() {
            anyhow::bail!(
                "SFace model not found at {}. See models/README.md for download instructions.",
                SFACE_MODEL_PATH
            );
        }

        // Load face detector (YuNet)
        // Parameters: model, config, input_size, score_threshold, nms_threshold, top_k, backend, target
        let detector = FaceDetectorYN::create(
            YUNET_MODEL_PATH,
            "",
            opencv::core::Size::new(320, 320),
            0.9,  // score threshold
            0.3,  // NMS threshold
            5000, // top_k
            0,    // backend (default)
            0,    // target (default)
        )?;

        // Load face recognizer (SFace)
        let recognizer = FaceRecognizerSF::create(
            SFACE_MODEL_PATH,
            "",
            0, // backend (default)
            0, // target (default)
        )?;

        Ok(Self {
            detector,
            recognizer,
        })
    }

    /// Detect faces in an image.
    ///
    /// Returns a list of detected faces with bounding boxes and landmarks.
    pub fn detect_faces(&mut self, image: &opencv::core::Mat) -> anyhow::Result<Vec<DetectedFace>> {
        use opencv::core::{Mat, Size};
        use opencv::prelude::*;

        // Resize input to detector's expected size
        let input_size = Size::new(image.cols(), image.rows());
        self.detector.set_input_size(input_size)?;

        // Run detection
        let mut faces = Mat::default();
        self.detector.detect(image, &mut faces)?;

        let mut results = Vec::new();
        let rows = faces.rows();

        for i in 0..rows {
            // Each row contains: x, y, w, h, x1, y1, x2, y2, x3, y3, x4, y4, x5, y5, score
            // (bounding box + 5 landmarks + confidence)
            let x = *faces.at_2d::<f32>(i, 0)?;
            let y = *faces.at_2d::<f32>(i, 1)?;
            let w = *faces.at_2d::<f32>(i, 2)?;
            let h = *faces.at_2d::<f32>(i, 3)?;
            let confidence = *faces.at_2d::<f32>(i, 14)?;

            let mut landmarks = Vec::new();
            for j in 0..5 {
                let lx = *faces.at_2d::<f32>(i, 4 + j * 2)?;
                let ly = *faces.at_2d::<f32>(i, 5 + j * 2)?;
                landmarks.push((lx, ly));
            }

            results.push(DetectedFace {
                bbox: (x, y, w, h),
                confidence,
                landmarks,
            });
        }

        Ok(results)
    }

    /// Extract face embedding from a detected face.
    ///
    /// The face parameter should be the row from detect_faces output.
    /// Returns a 512-dimensional embedding (128-dim from SFace, padded).
    pub fn extract_embedding(
        &mut self,
        image: &opencv::core::Mat,
        face_row: &opencv::core::Mat,
    ) -> anyhow::Result<Vec<f32>> {
        use opencv::core::Mat;
        use opencv::prelude::*;

        // Align face using the recognizer
        let mut aligned_face = Mat::default();
        self.recognizer.align_crop(image, face_row, &mut aligned_face)?;

        // Extract features
        let mut features = Mat::default();
        self.recognizer.feature(&aligned_face, &mut features)?;

        // Convert to Vec<f32>
        let mut embedding = Vec::with_capacity(512);
        let cols = features.cols();
        for i in 0..cols {
            embedding.push(*features.at_2d::<f32>(0, i)?);
        }

        // Pad to 512 dimensions for Qdrant compatibility
        while embedding.len() < 512 {
            embedding.push(0.0);
        }

        Ok(embedding)
    }

    /// Compare two face embeddings and return similarity score.
    ///
    /// Uses cosine similarity. Returns a value between 0.0 and 1.0.
    pub fn compare_embeddings(&self, emb1: &[f32], emb2: &[f32]) -> f32 {
        if emb1.len() != emb2.len() || emb1.is_empty() {
            return 0.0;
        }

        let dot: f32 = emb1.iter().zip(emb2.iter()).map(|(a, b)| a * b).sum();
        let norm1: f32 = emb1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm2: f32 = emb2.iter().map(|x| x * x).sum::<f32>().sqrt();

        if norm1 == 0.0 || norm2 == 0.0 {
            return 0.0;
        }

        // Cosine similarity normalized to 0-1 range
        (dot / (norm1 * norm2) + 1.0) / 2.0
    }
}

/// Extract face embedding from a JPEG-encoded frame.
///
/// This is a convenience function that decodes the JPEG, detects faces,
/// and extracts the embedding of the largest face.
#[cfg(feature = "sensory")]
pub fn extract_embedding_from_jpeg(jpeg_data: &[u8]) -> anyhow::Result<Option<FaceEmbedding>> {
    use opencv::core::{Mat, Vector};
    use opencv::imgcodecs::{imdecode, IMREAD_COLOR};
    use std::time::{SystemTime, UNIX_EPOCH};

    if jpeg_data.is_empty() {
        return Ok(None);
    }

    // Decode JPEG to Mat
    let data = Vector::from_slice(jpeg_data);
    let image = imdecode(&data, IMREAD_COLOR)?;

    if image.empty() {
        return Ok(None);
    }

    // Create recognizer
    let mut recognizer = FaceRecognizer::new()?;

    // Detect faces
    let faces = recognizer.detect_faces(&image)?;

    if faces.is_empty() {
        return Ok(None);
    }

    // Find largest face (by area)
    let largest = faces
        .iter()
        .max_by(|a, b| {
            let area_a = a.bbox.2 * a.bbox.3;
            let area_b = b.bbox.2 * b.bbox.3;
            area_a.partial_cmp(&area_b).unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap();

    // Re-detect to get the face row for alignment
    let input_size = opencv::core::Size::new(image.cols(), image.rows());
    recognizer.detector.set_input_size(input_size)?;
    let mut faces_mat = Mat::default();
    recognizer.detector.detect(&image, &mut faces_mat)?;

    if faces_mat.rows() == 0 {
        return Ok(None);
    }

    // Get the first face row (assuming it's the largest)
    let face_row = faces_mat.row(0)?;

    // Extract embedding
    let embedding = recognizer.extract_embedding(&image, &face_row)?;

    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);

    Ok(Some(FaceEmbedding {
        embedding,
        detection_confidence: largest.confidence,
        timestamp_ms,
    }))
}

/// Extract face embedding (stub when sensory feature is disabled).
#[cfg(not(feature = "sensory"))]
pub fn extract_embedding_from_jpeg(_jpeg_data: &[u8]) -> anyhow::Result<Option<FaceEmbedding>> {
    Ok(None)
}

// ============================================================================
// Liveness Detection (Anti-Spoofing)
// ============================================================================

/// Eye Aspect Ratio (EAR) calculation for blink detection.
///
/// EAR = (|p2-p6| + |p3-p5|) / (2 * |p1-p4|)
/// Where p1-p6 are the 6 eye landmarks.
#[cfg(feature = "sensory")]
fn calculate_ear(landmarks: &[(f32, f32)]) -> f32 {
    if landmarks.len() < 5 {
        return 0.0;
    }
    
    // Using the 5-point facial landmarks from YuNet:
    // 0: left eye, 1: right eye, 2: nose tip, 3: left mouth corner, 4: right mouth corner
    // For simplified EAR, we use the distance between eyes and nose as a proxy
    let left_eye = landmarks[0];
    let right_eye = landmarks[1];
    let nose = landmarks[2];
    
    // Calculate vertical distance (eye to nose) and horizontal distance (eye to eye)
    let vertical_left = ((left_eye.0 - nose.0).powi(2) + (left_eye.1 - nose.1).powi(2)).sqrt();
    let vertical_right = ((right_eye.0 - nose.0).powi(2) + (right_eye.1 - nose.1).powi(2)).sqrt();
    let horizontal = ((left_eye.0 - right_eye.0).powi(2) + (left_eye.1 - right_eye.1).powi(2)).sqrt();
    
    if horizontal == 0.0 {
        return 0.0;
    }
    
    // Simplified EAR proxy
    (vertical_left + vertical_right) / (2.0 * horizontal)
}

/// Liveness detection configuration.
///
/// The strictness parameter (0.0 to 1.0) scales the detection thresholds:
/// - 0.0 = Very lenient (good for low-light environments)
/// - 0.5 = Default (balanced)
/// - 1.0 = Very strict (high-security environments)
#[derive(Debug, Clone)]
pub struct LivenessConfig {
    /// Strictness level from 0.0 (lenient) to 1.0 (strict)
    pub strictness: f32,
}

impl Default for LivenessConfig {
    fn default() -> Self {
        Self {
            strictness: std::env::var("LIVENESS_STRICTNESS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.5),
        }
    }
}

impl LivenessConfig {
    /// Create a new config with the specified strictness.
    pub fn with_strictness(strictness: f32) -> Self {
        Self {
            strictness: strictness.clamp(0.0, 1.0),
        }
    }

    /// Get the EAR variance threshold based on strictness.
    ///
    /// Higher strictness = higher threshold = harder to pass
    pub fn ear_variance_threshold(&self) -> f32 {
        // Base threshold: 0.0005 (lenient) to 0.002 (strict)
        let base = 0.0005;
        let range = 0.0015;
        base + (range * self.strictness)
    }

    /// Get the head movement threshold based on strictness.
    ///
    /// Higher strictness = higher threshold = harder to pass
    pub fn head_movement_threshold(&self) -> f32 {
        // Base threshold: 1.0 (lenient) to 4.0 (strict)
        let base = 1.0;
        let range = 3.0;
        base + (range * self.strictness)
    }

    /// Get the blink detection threshold based on strictness.
    ///
    /// Higher strictness = higher threshold = harder to detect blinks
    pub fn blink_threshold(&self) -> f32 {
        // Base threshold: 0.01 (lenient) to 0.03 (strict)
        let base = 0.01;
        let range = 0.02;
        base + (range * self.strictness)
    }
}

/// Detect liveness from a sequence of frames using default configuration.
///
/// This function analyzes multiple frames to detect:
/// 1. Blink detection via Eye Aspect Ratio (EAR) changes
/// 2. Head movement via landmark position variance
///
/// Returns a LivenessResult indicating whether the subject is live.
#[cfg(feature = "sensory")]
pub fn detect_liveness(frames: &[Vec<u8>]) -> LivenessResult {
    detect_liveness_with_config(frames, &LivenessConfig::default())
}

/// Detect liveness from a sequence of frames with custom configuration.
///
/// This function analyzes multiple frames to detect:
/// 1. Blink detection via Eye Aspect Ratio (EAR) changes
/// 2. Head movement via landmark position variance
///
/// The strictness parameter scales the detection thresholds:
/// - Lower strictness (0.0-0.3): Good for low-light environments
/// - Medium strictness (0.4-0.6): Balanced (default)
/// - Higher strictness (0.7-1.0): High-security environments
///
/// Returns a LivenessResult indicating whether the subject is live.
#[cfg(feature = "sensory")]
pub fn detect_liveness_with_config(frames: &[Vec<u8>], config: &LivenessConfig) -> LivenessResult {
    use opencv::core::{Mat, Vector};
    use opencv::imgcodecs::{imdecode, IMREAD_COLOR};
    
    if frames.len() < 3 {
        return LivenessResult {
            is_live: false,
            blinks_detected: 0,
            ear_variance: 0.0,
            head_pose_variance: 0.0,
            confidence: 0.0,
            message: "Insufficient frames for liveness detection (need at least 3)".to_string(),
        };
    }
    
    let mut recognizer = match FaceRecognizer::new() {
        Ok(r) => r,
        Err(e) => {
            return LivenessResult {
                is_live: false,
                blinks_detected: 0,
                ear_variance: 0.0,
                head_pose_variance: 0.0,
                confidence: 0.0,
                message: format!("Failed to initialize face recognizer: {}", e),
            };
        }
    };
    
    let mut ear_values: Vec<f32> = Vec::new();
    let mut landmark_positions: Vec<Vec<(f32, f32)>> = Vec::new();
    let mut faces_detected = 0;
    
    for jpeg_data in frames {
        if jpeg_data.is_empty() {
            continue;
        }
        
        let data = Vector::from_slice(jpeg_data);
        let image = match imdecode(&data, IMREAD_COLOR) {
            Ok(img) => img,
            Err(_) => continue,
        };
        
        if image.empty() {
            continue;
        }
        
        let faces = match recognizer.detect_faces(&image) {
            Ok(f) => f,
            Err(_) => continue,
        };
        
        if faces.is_empty() {
            continue;
        }
        
        faces_detected += 1;
        let face = &faces[0]; // Use the first (largest) face
        
        // Calculate EAR for this frame
        let ear = calculate_ear(&face.landmarks);
        ear_values.push(ear);
        
        // Store landmark positions for head movement analysis
        landmark_positions.push(face.landmarks.clone());
    }
    
    if faces_detected < 3 {
        return LivenessResult {
            is_live: false,
            blinks_detected: 0,
            ear_variance: 0.0,
            head_pose_variance: 0.0,
            confidence: 0.0,
            message: format!("Only {} faces detected, need at least 3 for liveness", faces_detected),
        };
    }
    
    // Calculate EAR variance (indicates eye movement/blinking)
    let ear_mean: f32 = ear_values.iter().sum::<f32>() / ear_values.len() as f32;
    let ear_variance: f32 = ear_values.iter()
        .map(|x| (x - ear_mean).powi(2))
        .sum::<f32>() / ear_values.len() as f32;
    
    // Detect blinks: EAR drops significantly then recovers
    // Use config-based threshold for strictness scaling
    let ear_threshold = config.blink_threshold();
    let mut blinks_detected = 0u32;
    let mut in_blink = false;
    
    for i in 1..ear_values.len() {
        let ear_diff = ear_values[i - 1] - ear_values[i];
        if ear_diff > ear_threshold && !in_blink {
            in_blink = true;
        } else if ear_diff < -ear_threshold && in_blink {
            blinks_detected += 1;
            in_blink = false;
        }
    }
    
    // Calculate head pose variance (indicates head movement)
    let head_pose_variance = if landmark_positions.len() >= 2 {
        let mut total_variance = 0.0f32;
        for i in 1..landmark_positions.len() {
            let prev = &landmark_positions[i - 1];
            let curr = &landmark_positions[i];
            
            if prev.len() >= 3 && curr.len() >= 3 {
                // Calculate movement of nose tip (landmark 2)
                let dx = curr[2].0 - prev[2].0;
                let dy = curr[2].1 - prev[2].1;
                total_variance += (dx.powi(2) + dy.powi(2)).sqrt();
            }
        }
        total_variance / (landmark_positions.len() - 1) as f32
    } else {
        0.0
    };
    
    // Determine liveness based on multiple factors
    // A live person should show some eye movement (EAR variance) or head movement
    // Thresholds are scaled by the strictness configuration
    let ear_variance_threshold = config.ear_variance_threshold();
    let head_movement_threshold = config.head_movement_threshold();
    
    let has_eye_movement = ear_variance > ear_variance_threshold;
    let has_head_movement = head_pose_variance > head_movement_threshold;
    let has_blinks = blinks_detected > 0;
    
    let is_live = has_blinks || (has_eye_movement && has_head_movement);
    
    // Calculate confidence score
    let confidence = if is_live {
        let blink_score = (blinks_detected as f32 * 0.3).min(0.3);
        let ear_score = (ear_variance / ear_variance_threshold).min(1.0) * 0.35;
        let head_score = (head_pose_variance / head_movement_threshold).min(1.0) * 0.35;
        (blink_score + ear_score + head_score).min(1.0)
    } else {
        0.0
    };
    
    let message = if is_live {
        format!(
            "Liveness confirmed: {} blink(s), EAR variance: {:.4}, head movement: {:.2}px",
            blinks_detected, ear_variance, head_pose_variance
        )
    } else {
        format!(
            "Liveness check failed: {} blink(s), EAR variance: {:.4} (need > {:.4}), head movement: {:.2}px (need > {:.1}px)",
            blinks_detected, ear_variance, ear_variance_threshold, head_pose_variance, head_movement_threshold
        )
    };
    
    LivenessResult {
        is_live,
        blinks_detected,
        ear_variance,
        head_pose_variance,
        confidence,
        message,
    }
}

/// Detect liveness (stub when sensory feature is disabled).
#[cfg(not(feature = "sensory"))]
pub fn detect_liveness(_frames: &[Vec<u8>]) -> LivenessResult {
    LivenessResult {
        is_live: false,
        blinks_detected: 0,
        ear_variance: 0.0,
        head_pose_variance: 0.0,
        confidence: 0.0,
        message: "Sensory feature not enabled".to_string(),
    }
}

/// Perform enrollment with liveness detection.
///
/// This function captures multiple frames, verifies liveness, and then
/// extracts the face embedding. Returns an error if liveness check fails.
#[cfg(feature = "sensory")]
pub fn enroll_with_liveness(
    frames: &[Vec<u8>],
) -> Result<(FaceEmbedding, LivenessResult), SecurityError> {
    // First, check liveness
    let liveness = detect_liveness(frames);
    
    if !liveness.is_live {
        return Err(SecurityError::LivenessFailed {
            reason: liveness.message.clone(),
        });
    }
    
    // Find the best frame (highest face detection confidence)
    let mut best_embedding: Option<FaceEmbedding> = None;
    let mut best_confidence = 0.0f32;
    
    for jpeg_data in frames {
        if let Ok(Some(emb)) = extract_embedding_from_jpeg(jpeg_data) {
            if emb.detection_confidence > best_confidence {
                best_confidence = emb.detection_confidence;
                best_embedding = Some(emb);
            }
        }
    }
    
    match best_embedding {
        Some(emb) => Ok((emb, liveness)),
        None => Err(SecurityError::NoFaceDetected),
    }
}

/// Perform enrollment with liveness detection (stub when sensory feature is disabled).
#[cfg(not(feature = "sensory"))]
pub fn enroll_with_liveness(
    _frames: &[Vec<u8>],
) -> Result<(FaceEmbedding, LivenessResult), SecurityError> {
    Err(SecurityError::CameraUnavailable)
}
