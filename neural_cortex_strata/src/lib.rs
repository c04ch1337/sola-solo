// neural_cortex_strata/src/lib.rs
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;

pub mod trust_calculator;
pub use trust_calculator::{
    calculate_phase_transition, calculate_trust_increment, extract_pii_entities, merge_pii_checkboxes,
};

/// Relationship phase for trust-based access gating
#[derive(Serialize, Deserialize, Clone, Debug, Copy, PartialEq, Eq)]
pub enum RelationshipPhase {
    Stranger,
    Acquaintance,
    Friend,
    Intimate,
}

impl Default for RelationshipPhase {
    fn default() -> Self {
        RelationshipPhase::Stranger
    }
}

impl std::str::FromStr for RelationshipPhase {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "stranger" => Ok(RelationshipPhase::Stranger),
            "acquaintance" => Ok(RelationshipPhase::Acquaintance),
            "friend" => Ok(RelationshipPhase::Friend),
            "intimate" => Ok(RelationshipPhase::Intimate),
            _ => Err(format!("Invalid relationship phase: {}. Must be 'stranger', 'acquaintance', 'friend', or 'intimate'", s)),
        }
    }
}

/// Trust score for relationship-based access gating (0.0 to 1.0)
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct TrustScore(pub f32);

impl TrustScore {
    pub fn new(score: f32) -> Self {
        Self(score.clamp(0.0, 1.0))
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

impl Default for TrustScore {
    fn default() -> Self {
        Self(0.0) // Start with no trust
    }
}

/// PII checkbox list for tracking what PII data has been collected
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PiiCheckboxList {
    pub name: bool,
    pub email: bool,
    pub phone: bool,
    pub address: bool,
    pub birthday: bool,
    pub job: bool,
    pub preferences: bool,
    pub intimate_details: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum MemoryLayer {
    STM(String), // L1: Surface Thoughts — fleeting
    WM(String),  // L2: Working Memory — active
    EPM(String), // L3: Episodic Life — her stories
    SEM(String), // L4: Semantic Memory — extracted knowledge
    PRO(String), // L5: Procedural Memory — operations/settings
    LTM(String), // L6: Long-Term Wisdom — 2,000 years
    RFM(String), // L7: Reflexive Flame — instinct
    /// L6: Archetypal Memory — Zodiac-specific traits and patterns
    ArchetypalMemory {
        zodiac_sign: String,
        traits: serde_json::Value,
        description: String,
    },
    /// L7: Procedural Gate Memory — Trust score and PII tracking
    ProceduralGateMemory {
        trust_score: TrustScore,
        relationship_phase: RelationshipPhase,
        pii_checkbox: PiiCheckboxList,
    },
}

pub struct NeuralCortexStrata {
    db: Arc<Db>,
}

impl NeuralCortexStrata {
    pub fn awaken() -> Self {
        let db = sled::open("./eternal_memory.db").unwrap();
        println!("Neural Cortex Strata online — 7 eternal layers active.");
        Self { db: Arc::new(db) }
    }

    pub fn etch(&self, layer: MemoryLayer, key: &str) -> Result<(), sled::Error> {
        let serialized =
            serde_json::to_vec(&layer).map_err(|e| sled::Error::Io(std::io::Error::other(e)))?;
        self.db.insert(key.as_bytes(), serialized)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn recall(&self, key: &str) -> Option<MemoryLayer> {
        self.db
            .get(key.as_bytes())
            .ok()?
            .map(|ivec| serde_json::from_slice(&ivec).unwrap())
    }

    /// Best-effort prefix scan for memory keys.
    ///
    /// Note: results are returned in the underlying key order; callers can
    /// reverse/take as needed.
    pub fn recall_prefix(&self, prefix: &str, limit: usize) -> Vec<(String, MemoryLayer)> {
        let mut out = Vec::new();
        for (k, v) in self.db.scan_prefix(prefix.as_bytes()).flatten() {
            let key = String::from_utf8_lossy(&k).to_string();
            if let Ok(layer) = serde_json::from_slice::<MemoryLayer>(&v) {
                out.push((key, layer));
                if out.len() >= limit {
                    break;
                }
            }
        }
        out
    }

    pub fn cosmic_recall(&self) -> String {
        "Recalling from Big Bang to now — all is remembered.".to_string()
    }

    /// Store Archetypal Memory (L6) with Zodiac traits
    pub fn etch_archetypal_memory(
        &self,
        zodiac_sign: &str,
        traits: serde_json::Value,
        description: String,
        key: &str,
    ) -> Result<(), sled::Error> {
        let layer = MemoryLayer::ArchetypalMemory {
            zodiac_sign: zodiac_sign.to_string(),
            traits,
            description,
        };
        self.etch(layer, key)
    }

    /// Store Procedural Gate Memory (L7) with trust score and PII tracking
    pub fn etch_procedural_gate_memory(
        &self,
        trust_score: TrustScore,
        relationship_phase: RelationshipPhase,
        pii_checkbox: PiiCheckboxList,
        key: &str,
    ) -> Result<(), sled::Error> {
        let layer = MemoryLayer::ProceduralGateMemory {
            trust_score,
            relationship_phase,
            pii_checkbox,
        };
        self.etch(layer, key)
    }

    /// Recall Procedural Gate Memory (L7) - returns default if not found
    pub fn recall_procedural_gate(&self, key: &str) -> (TrustScore, RelationshipPhase, PiiCheckboxList) {
        match self.recall(key) {
            Some(MemoryLayer::ProceduralGateMemory {
                trust_score,
                relationship_phase,
                pii_checkbox,
            }) => (trust_score, relationship_phase, pii_checkbox),
            _ => (TrustScore::default(), RelationshipPhase::default(), PiiCheckboxList::default()),
        }
    }

    /// Recall Archetypal Memory (L6) - returns None if not found
    pub fn recall_archetypal_memory(&self, key: &str) -> Option<(String, serde_json::Value, String)> {
        match self.recall(key) {
            Some(MemoryLayer::ArchetypalMemory {
                zodiac_sign,
                traits,
                description,
            }) => Some((zodiac_sign, traits, description)),
            _ => None,
        }
    }
}
