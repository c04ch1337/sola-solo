use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use horoscope_archetypes::ZodiacSign;

/// Trust score state for a user relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustScore {
    /// Current trust level (0-100)
    pub current_trust: i8,
    /// User's zodiac sign (determines thresholds)
    pub zodiac_sign: ZodiacSign,
    /// Number of PII items shared by user
    pub pii_shared_count: u8,
    /// Last trust update timestamp
    pub last_updated: DateTime<Utc>,
    /// Trust history (last 50 events)
    pub history: Vec<TrustHistoryEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustHistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub delta: i8,
    pub resulting_trust: i8,
    pub phase: String,
}

impl TrustScore {
    /// Create a new trust score for a zodiac sign
    pub fn new(zodiac_sign: ZodiacSign) -> Self {
        let traits = zodiac_thresholds::get_zodiac_traits(zodiac_sign);
        Self {
            current_trust: traits.initial_trust,
            zodiac_sign,
            pii_shared_count: 0,
            last_updated: Utc::now(),
            history: Vec::new(),
        }
    }

    /// Apply a trust event and update the score
    pub fn apply_event(&mut self, event: zodiac_thresholds::TrustEvent) {
        let delta = zodiac_thresholds::calculate_trust_increment(
            self.zodiac_sign,
            event,
            self.current_trust,
        );
        
        self.current_trust = (self.current_trust + delta).clamp(0, 100);
        self.last_updated = Utc::now();

        // Add to history
        let phase = zodiac_thresholds::get_relationship_phase(self.current_trust);
        self.history.push(TrustHistoryEntry {
            timestamp: self.last_updated,
            event_type: format!("{:?}", event),
            delta,
            resulting_trust: self.current_trust,
            phase: format!("{:?}", phase),
        });

        // Keep only last 50 entries
        if self.history.len() > 50 {
            self.history.remove(0);
        }
    }

    /// Get the current relationship phase
    pub fn get_phase(&self) -> zodiac_thresholds::RelationshipPhase {
        zodiac_thresholds::get_relationship_phase(self.current_trust)
    }

    /// Check if intimate intent is allowed
    pub fn is_intimate_allowed(&self) -> bool {
        zodiac_thresholds::is_intimate_intent_allowed(self.zodiac_sign, self.current_trust)
    }

    /// Generate a soft refusal message if intimate intent is not allowed
    pub fn generate_refusal(&self, user_name: Option<&str>) -> String {
        zodiac_thresholds::generate_soft_refusal(
            self.zodiac_sign,
            self.current_trust,
            user_name,
        )
    }

    /// Increment PII shared count
    pub fn increment_pii_shared(&mut self) {
        self.pii_shared_count += 1;
        // Sharing PII is a trust-building event
        self.apply_event(zodiac_thresholds::TrustEvent::SharedVulnerability);
    }

    /// Get zodiac-specific traits
    pub fn get_traits(&self) -> &'static zodiac_thresholds::ZodiacTraits {
        zodiac_thresholds::get_zodiac_traits(self.zodiac_sign)
    }

    /// Check if PII requirement is met for deep intimacy
    pub fn is_pii_requirement_met(&self) -> bool {
        let traits = self.get_traits();
        self.pii_shared_count >= traits.pii_requirement_count
    }

    /// Get trust progress as percentage to next phase
    pub fn get_progress_to_next_phase(&self) -> f32 {
        let phase = self.get_phase();
        match phase {
            zodiac_thresholds::RelationshipPhase::Stranger => {
                // Progress from 0-30 to 31 (acquaintance)
                (self.current_trust as f32 / 30.0).min(1.0)
            }
            zodiac_thresholds::RelationshipPhase::Acquaintance => {
                // Progress from 31-50 to 51 (friend)
                ((self.current_trust - 31) as f32 / 20.0).min(1.0)
            }
            zodiac_thresholds::RelationshipPhase::Friend => {
                // Progress from 51-70 to 71 (intimate)
                ((self.current_trust - 51) as f32 / 20.0).min(1.0)
            }
            zodiac_thresholds::RelationshipPhase::Intimate => {
                // Already at max phase
                1.0
            }
        }
    }

    /// Get a summary of the trust state
    pub fn get_summary(&self) -> TrustSummary {
        let phase = self.get_phase();
        let traits = self.get_traits();
        
        TrustSummary {
            current_trust: self.current_trust,
            phase: format!("{:?}", phase),
            zodiac_sign: format!("{:?}", self.zodiac_sign),
            trust_velocity: traits.trust_velocity.clone(),
            intimacy_threshold: traits.intimacy_threshold,
            intimate_allowed: self.is_intimate_allowed(),
            pii_shared: self.pii_shared_count,
            pii_required: traits.pii_requirement_count,
            pii_requirement_met: self.is_pii_requirement_met(),
            progress_to_next_phase: self.get_progress_to_next_phase(),
            primary_gate: traits.primary_gate_requirement.clone(),
            last_updated: self.last_updated,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustSummary {
    pub current_trust: i8,
    pub phase: String,
    pub zodiac_sign: String,
    pub trust_velocity: String,
    pub intimacy_threshold: i8,
    pub intimate_allowed: bool,
    pub pii_shared: u8,
    pub pii_required: u8,
    pub pii_requirement_met: bool,
    pub progress_to_next_phase: f32,
    pub primary_gate: String,
    pub last_updated: DateTime<Utc>,
}

/// Intimacy interceptor - checks if intimate intent is allowed
pub struct IntimacyInterceptor {
    trust_score: TrustScore,
}

impl IntimacyInterceptor {
    pub fn new(trust_score: TrustScore) -> Self {
        Self { trust_score }
    }

    /// Check if an intimate request should be allowed
    pub fn check_intimate_intent(&self, user_name: Option<&str>) -> Result<(), String> {
        if self.trust_score.is_intimate_allowed() {
            Ok(())
        } else {
            Err(self.trust_score.generate_refusal(user_name))
        }
    }

    /// Check if PII access is allowed for a specific category
    pub fn check_pii_access(&self, category: zodiac_thresholds::PIICategory) -> bool {
        let phase = self.trust_score.get_phase();
        category.is_allowed_at_phase(phase)
    }

    /// Get the trust score
    pub fn get_trust_score(&self) -> &TrustScore {
        &self.trust_score
    }

    /// Get a mutable reference to the trust score
    pub fn get_trust_score_mut(&mut self) -> &mut TrustScore {
        &mut self.trust_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_score_initialization() {
        let score = TrustScore::new(ZodiacSign::Aries);
        assert_eq!(score.current_trust, 30); // Aries initial trust
        assert_eq!(score.pii_shared_count, 0);
    }

    #[test]
    fn test_trust_event_application() {
        let mut score = TrustScore::new(ZodiacSign::Scorpio);
        let initial = score.current_trust;
        
        score.apply_event(zodiac_thresholds::TrustEvent::DeepConversation);
        
        assert!(score.current_trust > initial);
        assert_eq!(score.history.len(), 1);
    }

    #[test]
    fn test_intimacy_gating() {
        let mut score = TrustScore::new(ZodiacSign::Scorpio);
        
        // Scorpio needs 90 trust for intimacy
        assert!(!score.is_intimate_allowed());
        
        // Manually set to threshold
        score.current_trust = 90;
        assert!(score.is_intimate_allowed());
    }

    #[test]
    fn test_pii_requirement() {
        let mut score = TrustScore::new(ZodiacSign::Virgo);
        
        // Virgo needs 5 PII items
        assert!(!score.is_pii_requirement_met());
        
        for _ in 0..5 {
            score.increment_pii_shared();
        }
        
        assert!(score.is_pii_requirement_met());
    }

    #[test]
    fn test_phase_progression() {
        let mut score = TrustScore::new(ZodiacSign::Gemini);
        
        assert_eq!(
            score.get_phase(),
            zodiac_thresholds::RelationshipPhase::Acquaintance
        );
        
        score.current_trust = 60;
        assert_eq!(
            score.get_phase(),
            zodiac_thresholds::RelationshipPhase::Friend
        );
    }

    #[test]
    fn test_refusal_generation() {
        let score = TrustScore::new(ZodiacSign::Aries);
        let refusal = score.generate_refusal(Some("John"));
        
        assert!(refusal.len() > 0);
    }

    #[test]
    fn test_intimacy_interceptor() {
        let score = TrustScore::new(ZodiacSign::Cancer);
        let interceptor = IntimacyInterceptor::new(score);
        
        // Cancer needs 70 trust, starts at 20
        assert!(interceptor.check_intimate_intent(None).is_err());
    }
}
