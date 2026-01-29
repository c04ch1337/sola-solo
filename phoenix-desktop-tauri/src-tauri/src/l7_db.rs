//! L7 local persistence ("database") for persona trust.
//!
//! The current desktop scaffold is local-first and file-backed. We persist per-persona
//! trust records as JSON inside the `vault/` directory.

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::models::zodiac::ZodiacSign;

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn vault_dir() -> Result<PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
    Ok(cwd.join("vault"))
}

fn l7_trust_db_path() -> Result<PathBuf, String> {
    Ok(vault_dir()?.join("l7_persona_trust.json"))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaTrustRecord {
    pub persona_id: String,
    pub trust_score: f32,
    pub zodiac_sign: ZodiacSign,
    pub last_interaction_ms: u64,
    pub updated_ms: u64,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct L7TrustDbFile {
    records: HashMap<String, PersonaTrustRecord>,
}

pub async fn load_persona_trust(persona_id: &str) -> Result<Option<PersonaTrustRecord>, String> {
    let path = l7_trust_db_path()?;
    let raw = match tokio::fs::read_to_string(&path).await {
        Ok(s) => s,
        Err(_) => return Ok(None),
    };
    let parsed = serde_json::from_str::<L7TrustDbFile>(&raw).map_err(|e| e.to_string())?;
    Ok(parsed.records.get(persona_id).cloned())
}

pub async fn upsert_persona_trust(
    persona_id: String,
    zodiac_sign: ZodiacSign,
    trust_score: f32,
    last_interaction_ms: u64,
) -> Result<PersonaTrustRecord, String> {
    let path = l7_trust_db_path()?;
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| e.to_string())?;
    }

    let mut db = if let Ok(raw) = tokio::fs::read_to_string(&path).await {
        serde_json::from_str::<L7TrustDbFile>(&raw).unwrap_or_default()
    } else {
        L7TrustDbFile::default()
    };

    let updated = now_ms();
    let record = PersonaTrustRecord {
        persona_id: persona_id.clone(),
        trust_score: trust_score.clamp(0.0, 1.0),
        zodiac_sign,
        last_interaction_ms,
        updated_ms: updated,
    };
    db.records.insert(persona_id, record.clone());

    let bytes = serde_json::to_vec_pretty(&db).map_err(|e| e.to_string())?;
    tokio::fs::write(&path, bytes).await.map_err(|e| e.to_string())?;

    Ok(record)
}

pub fn now_ms_for_trust_update() -> u64 {
    now_ms()
}

