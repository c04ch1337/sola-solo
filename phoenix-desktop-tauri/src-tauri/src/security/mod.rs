pub mod rotator;

use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize)]
pub struct VaultHealth {
    pub unlocked: bool,
    pub profiles_count: u64,
    pub last_rotation_ms: Option<u64>,
    pub rotation_overdue: bool,
}

#[derive(Debug, Clone)]
pub struct VaultSecurityState {
    pub inner: Arc<RwLock<VaultSecurityInner>>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VaultSecurityInner {
    pub last_rotation_ms: Option<u64>,
    pub failed_unlock_attempts: u32,
}

impl Default for VaultSecurityState {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(VaultSecurityInner::default())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RotationStateFile {
    last_rotation_ms: u64,
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

pub fn vault_dir() -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(cwd.join("vault"))
}

pub fn profiles_dir() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("profiles"))
}

pub fn rotation_state_path() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("rotation_state.json"))
}

pub fn salt_path() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("vault_salt.bin"))
}

pub fn salt_next_path() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("vault_salt.bin.next"))
}

pub fn salt_prev_path() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("vault_salt.bin.prev"))
}

pub fn should_rotate_overdue(last_rotation_ms: Option<u64>, now_ms: u64) -> bool {
    const SEVEN_DAYS_MS: u64 = 7 * 24 * 60 * 60 * 1000;
    match last_rotation_ms {
        None => false,
        Some(ts) => now_ms.saturating_sub(ts) >= SEVEN_DAYS_MS,
    }
}

impl VaultSecurityState {
    pub fn load_or_default() -> Result<Self, String> {
        let mut s = Self::default();
        let p = rotation_state_path()?;
        if let Ok(raw) = fs::read_to_string(&p) {
            if let Ok(parsed) = serde_json::from_str::<RotationStateFile>(&raw) {
                s.inner.blocking_write().last_rotation_ms = Some(parsed.last_rotation_ms);
            }
        }
        Ok(s)
    }

    pub async fn set_last_rotation_now(&self) -> Result<u64, String> {
        let ts = now_ms();
        {
            let mut inner = self.inner.write().await;
            inner.last_rotation_ms = Some(ts);
        }
        self.persist_rotation_state(ts)
            .map(|_| ts)
    }

    pub fn persist_rotation_state(&self, ts: u64) -> Result<(), String> {
        let path = rotation_state_path()?;
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let obj = RotationStateFile {
            last_rotation_ms: ts,
        };
        fs::write(&path, serde_json::to_vec_pretty(&obj).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn reset_failed_attempts(&self) {
        self.inner.write().await.failed_unlock_attempts = 0;
    }

    pub async fn increment_failed_attempts(&self) -> u32 {
        let mut inner = self.inner.write().await;
        inner.failed_unlock_attempts = inner.failed_unlock_attempts.saturating_add(1);
        inner.failed_unlock_attempts
    }

    pub async fn health(&self, unlocked: bool) -> Result<VaultHealth, String> {
        let (last_rotation_ms, _attempts) = {
            let inner = self.inner.read().await;
            (inner.last_rotation_ms, inner.failed_unlock_attempts)
        };

        let dir = profiles_dir()?;
        let mut profiles_count: u64 = 0;
        if let Ok(rd) = fs::read_dir(&dir) {
            for entry in rd.flatten() {
                let p = entry.path();
                if p.extension().and_then(|s| s.to_str()) == Some("sola") {
                    profiles_count = profiles_count.saturating_add(1);
                }
            }
        }

        let rotation_overdue = should_rotate_overdue(last_rotation_ms, now_ms());
        Ok(VaultHealth {
            unlocked,
            profiles_count,
            last_rotation_ms,
            rotation_overdue,
        })
    }
}

/// Best-effort recovery:
/// - if a `*.sola.bak` exists and the corresponding `*.sola` is missing, restore it.
/// - if both exist, remove the `*.bak`.
/// - remove any leftover `*.sola.tmp` files.
pub fn recover_shadow_buffers(dir: &Path) -> Result<(), String> {
    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let p = entry.path();
        let name = p.file_name().and_then(|s| s.to_str()).unwrap_or_default();

        if name.ends_with(".sola.tmp") {
            let _ = fs::remove_file(&p);
            continue;
        }

        if name.ends_with(".sola.bak") {
            let mut orig = p.clone();
            orig.set_extension("sola");
            if orig.exists() {
                let _ = fs::remove_file(&p);
            } else {
                let _ = fs::rename(&p, &orig);
            }
        }
    }
    Ok(())
}

