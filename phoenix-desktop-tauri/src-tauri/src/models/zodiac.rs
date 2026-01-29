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

/// Coarse-grained interaction classification used to apply zodiac preference multipliers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreferredInput {
    DeepSecrets,
    HighVolume,
    Consistency,
    Intensity,
    EmotionalDepth,
    TimeLogic,
    Balanced,
}

/// Special bonus condition for a given zodiac (beyond base preference).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BonusTrigger {
    PiiSharing,
    VariedTopics,
    DailyCheckins,
    AdventurousPrompts,
    FeelingKeywords,
    ProfessionalCrosstalk,
    None,
}

/// Signals extracted from a user's message/event for trust math.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TrustSignals {
    pub input_type: PreferredInput,
    pub pii_sharing: bool,
    pub varied_topics: bool,
    pub daily_checkin: bool,
    pub adventurous_prompt: bool,
    pub feeling_keywords: bool,
    pub professional_crosstalk: bool,
}

impl TrustSignals {
    pub fn matches_bonus(&self, trigger: BonusTrigger) -> bool {
        match trigger {
            BonusTrigger::PiiSharing => self.pii_sharing,
            BonusTrigger::VariedTopics => self.varied_topics,
            BonusTrigger::DailyCheckins => self.daily_checkin,
            BonusTrigger::AdventurousPrompts => self.adventurous_prompt,
            BonusTrigger::FeelingKeywords => self.feeling_keywords,
            BonusTrigger::ProfessionalCrosstalk => self.professional_crosstalk,
            BonusTrigger::None => false,
        }
    }
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

    /// Which interaction type this sign learns from fastest.
    #[serde(default = "PreferredInput::default_balanced")]
    pub preferred_input: PreferredInput,

    /// Applied when `signals.input_type == preferred_input`.
    #[serde(default = "ZodiacProfile::default_preferred_input_multiplier")]
    pub preferred_input_multiplier: f64,

    /// Optional special bonus condition.
    #[serde(default = "BonusTrigger::default_none")]
    pub bonus_trigger: BonusTrigger,

    /// Applied when `signals.matches_bonus(bonus_trigger)`.
    #[serde(default = "ZodiacProfile::default_bonus_multiplier")]
    pub bonus_multiplier: f64,

    /// "Cold shoulder" decay applied per 48h of inactivity *after* the first 48h.
    ///
    /// Example: a value of 0.06 means losing 0.06 trust per 48h window.
    #[serde(default = "ZodiacProfile::default_decay_48h")]
    pub decay_48h: f64,

    pub phase_thresholds: PhaseThresholds,
    pub visual_unlock_gates: Vec<u8>,
}

impl PreferredInput {
    fn default_balanced() -> Self {
        Self::Balanced
    }
}

impl BonusTrigger {
    fn default_none() -> Self {
        Self::None
    }
}

impl ZodiacProfile {
    fn default_preferred_input_multiplier() -> f64 {
        1.0
    }

    fn default_bonus_multiplier() -> f64 {
        1.0
    }

    fn default_decay_48h() -> f64 {
        0.03
    }

    /// Computes a trust delta for a single interaction.
    ///
    /// - `base_gain` is the interaction signal (e.g., 0.01 .. 0.10).
    /// - applies `trust_slope` as a multiplier.
    /// - adds volatility jitter proportional to `|base_gain|`.
    pub fn calculate_trust_gain(&self, base_gain: f64) -> f64 {
        self.calculate_trust_gain_with_signals(base_gain, None)
    }

    /// Implements:
    ///
    /// T_new = T_old + (Δ_interaction × σ_zodiac × ω_type)
    ///
    /// where:
    /// - Δ_interaction is the raw score of the user's message/event
    /// - σ_zodiac is this profile's `trust_slope`
    /// - ω_type is derived from preference/bonus signals
    ///
    /// Volatility is applied as jitter on the computed delta.
    pub fn update_trust(
        &self,
        trust_old: f64,
        delta_interaction: f64,
        signals: TrustSignals,
        last_interaction_ms: Option<u64>,
        now_ms: u64,
    ) -> f64 {
        let trust_old = trust_old.clamp(0.0, 1.0);

        // Cold shoulder: if no interaction for 48h, apply sign-specific decay.
        let trust_after_decay = self.apply_cold_shoulder_decay(trust_old, last_interaction_ms, now_ms);

        let mut omega_type = 1.0;
        if signals.input_type == self.preferred_input {
            omega_type *= self.preferred_input_multiplier.max(0.0);
        }
        if signals.matches_bonus(self.bonus_trigger) {
            omega_type *= self.bonus_multiplier.max(0.0);
        }

        // Base formula
        let raw_delta = delta_interaction * self.trust_slope * omega_type;

        // Volatility: random jitter proportional to |raw_delta|
        let jitter = (fastrand::f64() * 2.0 - 1.0) * self.volatility * raw_delta.abs();
        let delta = (raw_delta + jitter).clamp(-1.0, 1.0);

        (trust_after_decay + delta).clamp(0.0, 1.0)
    }

    /// Helper: compute trust gain without needing full signal extraction.
    pub fn calculate_trust_gain_with_signals(&self, base_gain: f64, signals: Option<TrustSignals>) -> f64 {
        let signals = signals.unwrap_or(TrustSignals {
            input_type: PreferredInput::Balanced,
            pii_sharing: false,
            varied_topics: false,
            daily_checkin: false,
            adventurous_prompt: false,
            feeling_keywords: false,
            professional_crosstalk: false,
        });
        // Return delta for convenience (treat trust_old=0, no decay).
        self.update_trust(0.0, base_gain, signals, None, 0) - 0.0
    }

    fn apply_cold_shoulder_decay(&self, trust_old: f64, last_interaction_ms: Option<u64>, now_ms: u64) -> f64 {
        const THRESH_MS: u64 = 48 * 60 * 60 * 1000;
        let Some(last_ms) = last_interaction_ms else {
            return trust_old;
        };
        if now_ms <= last_ms {
            return trust_old;
        }

        let idle_ms = now_ms - last_ms;
        if idle_ms < THRESH_MS {
            return trust_old;
        }

        // Apply decay once per 48h window after threshold (including the first exceeded window).
        let windows = (idle_ms / THRESH_MS) as f64;
        let decay = self.decay_48h.max(0.0) * windows;
        (trust_old - decay).clamp(0.0, 1.0)
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

