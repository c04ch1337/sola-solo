use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use zeroize::Zeroizing;

use crate::vault::AES256_KEY_LEN;
use crate::models::zodiac::ZodiacSign;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum OrchestratorMode {
    Professional,
    Personal,
}

#[derive(Default)]
pub struct SolaState {
    pub inner: Arc<RwLock<SolaStateInner>>,
}

#[derive(Debug)]
pub struct SolaStateInner {
    pub current_mode: OrchestratorMode,
    pub active_persona_id: Option<String>,
    pub vault_key: Option<Zeroizing<[u8; AES256_KEY_LEN]>>,

    // L7+ gating inputs (minimal for now; wire to backend memory later)
    pub trust_score: f32,
    pub zodiac_sign: Option<ZodiacSign>,
}

impl Default for SolaStateInner {
    fn default() -> Self {
        Self {
            current_mode: OrchestratorMode::Professional,
            active_persona_id: None,
            vault_key: None,
            trust_score: 0.0,
            zodiac_sign: None,
        }
    }
}

impl SolaState {
    pub async fn is_soul_vault_unlocked(&self) -> bool {
        self.inner.read().await.vault_key.is_some()
    }

    pub async fn set_vault_key(&self, key: zeroize::Zeroizing<[u8; AES256_KEY_LEN]>) {
        self.inner.write().await.vault_key = Some(key);
    }

    pub async fn clear_vault_key(&self) {
        self.inner.write().await.vault_key = None;
    }
}

