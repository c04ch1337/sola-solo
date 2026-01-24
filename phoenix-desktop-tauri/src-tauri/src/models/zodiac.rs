use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Canonical set of zodiac signs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ZodiacSign {
    Aries,
    Taurus,
    Gemini,
    Cancer,
    Leo,
    Virgo,
    Libra,
    Scorpio,
    Sagittarius,
    Capricorn,
    Aquarius,
    Pisces,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseThresholds {
    #[serde(rename = "Acquaintance")]
    pub acquaintance: f64,
    #[serde(rename = "Friend")]
    pub friend: f64,
    #[serde(rename = "Intimate")]
    pub intimate: f64,
    #[serde(rename = "ResearchPartner")]
    pub research_partner: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZodiacProfile {
    pub initial_trust: f64,
    pub trust_slope: f64,
    /// Random variance applied to trust gain/loss.
    ///
    /// Expected range: 0.0 ..= 0.5
    pub volatility: f64,
    pub phase_thresholds: PhaseThresholds,
    pub visual_unlock_gates: Vec<u8>,
}

impl ZodiacProfile {
    /// Computes a trust delta for a single interaction.
    ///
    /// - `base_gain` is the interaction signal (e.g., 0.01 .. 0.10).
    /// - applies `trust_slope` as a multiplier.
    /// - adds volatility jitter proportional to `|base_gain|`.
    pub fn calculate_trust_gain(&self, base_gain: f64) -> f64 {
        let scaled = base_gain * self.trust_slope;
        let jitter = (fastrand::f64() * 2.0 - 1.0) * self.volatility * base_gain.abs();
        // clamp to a sane per-interaction delta
        (scaled + jitter).clamp(-1.0, 1.0)
    }
}

/// In-memory registry loaded at startup from `config/zodiac_matrix.json`.
#[derive(Debug, Clone)]
pub struct ZodiacRegistry {
    pub profiles: HashMap<ZodiacSign, ZodiacProfile>,
}

impl ZodiacRegistry {
    pub fn from_embedded_json() -> Result<Self, String> {
        let raw = include_str!("../../config/zodiac_matrix.json");
        let profiles: HashMap<ZodiacSign, ZodiacProfile> =
            serde_json::from_str(raw).map_err(|e| format!("zodiac_matrix.json parse error: {e}"))?;
        Ok(Self { profiles })
    }

    pub fn profile(&self, sign: ZodiacSign) -> Option<&ZodiacProfile> {
        self.profiles.get(&sign)
    }

    /// Current policy: NSFW/high-fidelity image decrypt requires the `Intimate` phase.
    pub fn nsfw_threshold(&self, sign: ZodiacSign) -> f64 {
        self.profile(sign)
            .map(|p| p.phase_thresholds.intimate)
            .unwrap_or(0.85)
    }
}

