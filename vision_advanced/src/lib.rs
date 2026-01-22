//! Advanced vision backend crate.
//!
//! This crate previously contained a native backend.
//! It is now a minimal stub that compiles without native libraries.

use image::{ImageBuffer, Rgb};
use std::fs;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};

/// Local shim so callers can write `tract::Model` (requested API shape).
pub mod tract {
    #[derive(Debug, Clone)]
    pub struct Model;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DetectedEmotion {
    Happy,
    Sad,
    Angry,
    Fearful,
    Disgusted,
    Surprised,
    Neutral,
    Love,
}

#[derive(Debug, Clone)]
pub struct AdvancedVisionResult {
    pub face_rect: Rect,
    pub landmarks: Vec<Point>,
    pub emotion: DetectedEmotion,
    pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct VisionResult {
    pub faces_detected: usize,
    pub primary_emotion: Option<DetectedEmotion>,
    pub results: Vec<AdvancedVisionResult>,
}

#[derive(Debug, Clone)]
pub enum VisionError {
    BackendUnavailable(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

pub struct AdvancedVision {
    // Stubbed: left as Options to preserve the intended shape.
    pub landmark_predictor: Option<tract::Model>,
    pub emotion_model: Option<tract::Model>,
}

impl AdvancedVision {
    pub fn new() -> Result<Self, VisionError> {
        dotenvy::dotenv().ok();

        // Keep large ML assets out of git; download on first run if requested.
        // This is best-effort: failures should not prevent the workspace from compiling/running.
        if auto_download_models_enabled() {
            if let Err(e) = ensure_default_models_present() {
                eprintln!("[vision_advanced] model download/decompress failed: {e}");
            }
        }

        // Keep env reads for compatibility, but do not load native backends.
        let _ = std::env::var("LANDMARK_MODEL_PATH").ok();
        let _ = std::env::var("EMOTION_ONNX_MODEL_PATH").ok();

        Ok(Self {
            landmark_predictor: None,
            emotion_model: None,
        })
    }

    pub async fn process_live_frame(
        &self,
        frame: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    ) -> Result<VisionResult, VisionError> {
        let _ = frame;
        Ok(VisionResult {
            faces_detected: 0,
            primary_emotion: None,
            results: Vec::new(),
        })
    }
}

const DLIB_LANDMARK_BZ2_URL: &str =
    "http://dlib.net/files/shape_predictor_68_face_landmarks.dat.bz2";
const FERPLUS_ONNX_URL: &str = "https://github.com/onnx/models/raw/main/validated/vision/body_analysis/emotion_ferplus/model/emotion-ferplus-8.onnx";

fn auto_download_models_enabled() -> bool {
    env_bool("PHOENIX_AUTO_DOWNLOAD_MODELS").unwrap_or(true)
}

fn models_dir() -> PathBuf {
    std::env::var("PHOENIX_MODELS_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("models"))
}

fn ensure_default_models_present() -> Result<(), Box<dyn std::error::Error>> {
    let dir = models_dir();
    fs::create_dir_all(&dir)?;

    // Dlib landmarks predictor
    let dlib_dat = dir.join("shape_predictor_68_face_landmarks.dat");
    let dlib_bz2 = dir.join("shape_predictor_68_face_landmarks.dat.bz2");
    ensure_dlib_landmark_predictor(&dlib_dat, &dlib_bz2)?;

    // FER+ ONNX model (downloads into models/ferplus.onnx)
    let ferplus = dir.join("ferplus.onnx");
    ensure_file_downloaded(&ferplus, FERPLUS_ONNX_URL)?;

    // Optional: a separate `fer.onnx` (URL must be provided because this repo's original file
    // may have been custom).
    let fer = dir.join("fer.onnx");
    if !fer.exists() {
        if let Some(url) = std::env::var("PHOENIX_FER_ONNX_URL")
            .ok()
            .or_else(|| std::env::var("FER_ONNX_MODEL_URL").ok())
        {
            ensure_file_downloaded(&fer, &url)?;
        } else {
            eprintln!(
                "[vision_advanced] models/fer.onnx missing; set PHOENIX_FER_ONNX_URL to auto-download it"
            );
        }
    }

    Ok(())
}

fn ensure_dlib_landmark_predictor(
    dat_path: &Path,
    bz2_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if dat_path.exists() {
        return Ok(());
    }

    if !bz2_path.exists() {
        ensure_file_downloaded(bz2_path, DLIB_LANDMARK_BZ2_URL)?;
    }

    // Decompress .bz2 -> .dat
    let mut decoder = bzip2::read::BzDecoder::new(File::open(bz2_path)?);
    let out = BufWriter::new(File::create(dat_path)?);
    let mut out = out;
    io::copy(&mut decoder, &mut out)?;
    out.flush()?;

    if env_bool("PHOENIX_DELETE_COMPRESSED_MODELS").unwrap_or(false) {
        let _ = fs::remove_file(bz2_path);
    }

    Ok(())
}

fn ensure_file_downloaded(path: &Path, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        return Ok(());
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    eprintln!(
        "[vision_advanced] downloading {} -> {}",
        url,
        path.display()
    );

    let client = reqwest::blocking::Client::builder()
        .user_agent("phoenix-vision-advanced/0.1")
        .build()?;
    let mut resp = client.get(url).send()?.error_for_status()?;

    // Stream to disk (avoid keeping full file in memory)
    let tmp = path.with_extension("download");
    let mut file = BufWriter::new(File::create(&tmp)?);
    io::copy(&mut resp, &mut file)?;
    file.flush()?;
    fs::rename(tmp, path)?;
    Ok(())
}

fn env_bool(key: &str) -> Option<bool> {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_ascii_lowercase())
        .and_then(|s| match s.as_str() {
            "1" | "true" | "yes" | "y" | "on" => Some(true),
            "0" | "false" | "no" | "n" | "off" => Some(false),
            _ => None,
        })
}
