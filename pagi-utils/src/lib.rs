// pagi-utils/src/lib.rs
//
// Centralized utilities for the PAGI Twin ecosystem.
// Provides common functions for environment variable handling, logging, and .env loading.

use std::path::{Path, PathBuf};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Returns the value of an environment variable if it exists and is non-empty.
pub fn env_nonempty(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Returns true if the environment variable is set to a truthy value.
/// Truthy values: "1", "true", "yes", "y", "on" (case-insensitive).
pub fn env_truthy(key: &str) -> bool {
    env_nonempty(key)
        .map(|s| {
            matches!(
                s.to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "y" | "on"
            )
        })
        .unwrap_or(false)
}

/// Attempts to load a .env file from the specified path, overriding existing variables.
pub fn try_load_dotenv_override(path: &Path) -> Result<(), String> {
    dotenvy::from_path_override(path)
        .map(|_| ())
        .map_err(|e| format!("{e}"))
}

/// Load `.env` from a reasonable location (cwd/exe directory + parents).
/// Searches in order:
/// 1. PHOENIX_DOTENV_PATH environment variable
/// 2. Current working directory and its parents
/// 3. Executable directory and its parents
///
/// Returns the path to the loaded .env file, or None if not found.
pub fn load_dotenv_best_effort() -> Option<PathBuf> {
    // 1. Check PHOENIX_DOTENV_PATH
    if let Some(p) = env_nonempty("PHOENIX_DOTENV_PATH") {
        let path = PathBuf::from(p);
        if path.is_file() {
            let _ = dotenvy::from_path_override(&path);
            return Some(path);
        }
    }

    // 2. Build list of base directories to search
    let mut bases: Vec<PathBuf> = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        bases.push(cwd);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            bases.push(dir.to_path_buf());
        }
    }

    // 3. Search each base and its parents
    for base in bases {
        let mut current = Some(base.as_path());
        while let Some(dir) = current {
            let env_path = dir.join(".env");
            if env_path.is_file() {
                let _ = dotenvy::from_path_override(&env_path);
                return Some(env_path);
            }
            current = dir.parent();
        }
    }

    None
}

/// Initialize tracing subscriber with environment filter.
/// Uses RUST_LOG environment variable for filtering (defaults to "info").
pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}

/// Initialize tracing subscriber with a custom default log level.
pub fn init_tracing_with_default(default_level: &str) {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(default_level))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
