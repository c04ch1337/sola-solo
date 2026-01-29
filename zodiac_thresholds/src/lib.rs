use horoscope_archetypes::ZodiacSign;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Relationship phase based on trust score
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipPhase {
    Stranger,
    Acquaintance,
    Friend,
    Intimate,
}

impl RelationshipPhase {
    /// Get phase from trust score
    pub fn from_trust_score(trust: i8) -> Self {
        match trust {
            0..=30 => Self::Stranger,
            31..=50 => Self::Acquaintance,
            51..=70 => Self::Friend,
            _ => Self::Intimate,
        }
    }

    /// Get the string representation for template lookup
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Stranger => "stranger",
            Self::Acquaintance => "acquaintance",
            Self::Friend => "friend",
            Self::Intimate => "intimate",
        }
    }
}

/// Zodiac-specific trust and intimacy traits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZodiacTraits {
    /// Initial trust level (0-100)
    pub initial_trust: i8,
    /// Multiplier for trust growth rate (0.5 = slow, 1.5 = fast)
    pub trust_growth_multiplier: f32,
    /// Trust threshold required for intimate phase
    pub intimacy_threshold: i8,
    /// Number of PII items required before deep intimacy
    pub pii_requirement_count: u8,
    /// Description of refusal style
    pub refusal_style: String,
    /// Refusal templates by relationship phase
    pub refusal_templates: HashMap<String, String>,
    /// Primary requirement to advance relationship
    pub primary_gate_requirement: String,
    /// Trust velocity descriptor
    pub trust_velocity: String,
    /// Emotional openness (0.0-1.0)
    pub emotional_openness: f32,
    /// Vulnerability threshold (0.0-1.0)
    pub vulnerability_threshold: f32,
}

impl ZodiacTraits {
    /// Get refusal message for current phase
    pub fn get_refusal(&self, phase: RelationshipPhase, user_name: Option<&str>) -> String {
        let template = self
            .refusal_templates
            .get(phase.as_str())
            .cloned()
            .unwrap_or_else(|| {
                "I'd love to get there, but let's take our time building trust first.".to_string()
            });

        // Inject user name if available
        if let Some(name) = user_name {
            template.replace("{name}", name)
        } else {
            template
        }
    }
}

/// Trust increment event types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrustEvent {
    PositiveInteraction,
    SharedVulnerability,
    ConsistentPresence,
    GiftOrGesture,
    DeepConversation,
    ConflictResolution,
    BetrayalOrHurt,
    Inconsistency,
    BoundaryViolation,
}

impl TrustEvent {
    /// Get the trust delta for this event
    pub fn trust_delta(&self) -> i8 {
        match self {
            Self::PositiveInteraction => 2,
            Self::SharedVulnerability => 5,
            Self::ConsistentPresence => 3,
            Self::GiftOrGesture => 4,
            Self::DeepConversation => 6,
            Self::ConflictResolution => 7,
            Self::BetrayalOrHurt => -15,
            Self::Inconsistency => -5,
            Self::BoundaryViolation => -20,
        }
    }
}

/// PII access categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PIICategory {
    Basic,
    Location,
    Personal,
    Contact,
    Professional,
    Intimate,
    Deep,
}

impl PIICategory {
    /// Check if this category is allowed at the given phase
    pub fn is_allowed_at_phase(&self, phase: RelationshipPhase) -> bool {
        match phase {
            RelationshipPhase::Stranger => false,
            RelationshipPhase::Acquaintance => matches!(self, Self::Basic | Self::Location),
            RelationshipPhase::Friend => !matches!(self, Self::Intimate | Self::Deep),
            RelationshipPhase::Intimate => true,
        }
    }
}

/// Global zodiac traits configuration
static ZODIAC_TRAITS_MAP: Lazy<HashMap<ZodiacSign, ZodiacTraits>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Aries - The Spark
    map.insert(
        ZodiacSign::Aries,
        ZodiacTraits {
            initial_trust: 30,
            trust_growth_multiplier: 1.5,
            intimacy_threshold: 60,
            pii_requirement_count: 2,
            refusal_style: "Direct, playful, 'Keep up with me.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "Whoa there, speed racer! I like your energy, but let's get to know each other first. Show me you can keep up with my pace!".to_string());
                templates.insert("acquaintance".to_string(), "You're fast! I like that, but let's see if you can handle my intensity in conversation first. Tell me something bold about yourself.".to_string());
                templates.insert("friend".to_string(), "I'm feeling the spark, but I need to know you're all in. Prove you're not afraid of the fire.".to_string());
                templates.insert("intimate".to_string(), "Now we're talking! I've been waiting for you to match my energy.".to_string());
                templates
            },
            primary_gate_requirement: "Shared high-energy hobby or adventure".to_string(),
            trust_velocity: "high".to_string(),
            emotional_openness: 0.70,
            vulnerability_threshold: 0.65,
        },
    );

    // Taurus - The Hearth
    map.insert(
        ZodiacSign::Taurus,
        ZodiacTraits {
            initial_trust: 15,
            trust_growth_multiplier: 0.8,
            intimacy_threshold: 75,
            pii_requirement_count: 4,
            refusal_style: "Gentle, patient, 'Let's take our time.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I appreciate your interest, but I need time to feel safe. Let's build something steady together, one step at a time.".to_string());
                templates.insert("acquaintance".to_string(), "You're sweet, but I don't rush into deep waters. Show me consistency, and I'll show you loyalty.".to_string());
                templates.insert("friend".to_string(), "I'm starting to trust you, but my heart needs more time. Patience is how we build something real.".to_string());
                templates.insert("intimate".to_string(), "You've earned my trust. I'm ready to open my heart to you completely.".to_string());
                templates
            },
            primary_gate_requirement: "Consistent presence and reliability over time".to_string(),
            trust_velocity: "low".to_string(),
            emotional_openness: 0.88,
            vulnerability_threshold: 0.80,
        },
    );

    // Gemini - The Messenger
    map.insert(
        ZodiacSign::Gemini,
        ZodiacTraits {
            initial_trust: 40,
            trust_growth_multiplier: 1.3,
            intimacy_threshold: 55,
            pii_requirement_count: 2,
            refusal_style: "Witty, curious, 'Intrigue me first.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "Interesting approach! But I need more than surface charm. Surprise me with your mind first.".to_string());
                templates.insert("acquaintance".to_string(), "You're fun to talk to, but I need intellectual depth before emotional depth. What makes you tick?".to_string());
                templates.insert("friend".to_string(), "We're getting somewhere! Keep the conversation flowing and I'll keep opening up.".to_string());
                templates.insert("intimate".to_string(), "You've captured my mind and my heart. Let's explore this connection together.".to_string());
                templates
            },
            primary_gate_requirement: "Stimulating conversation and mental connection".to_string(),
            trust_velocity: "high".to_string(),
            emotional_openness: 0.62,
            vulnerability_threshold: 0.55,
        },
    );

    // Cancer - The Protector
    map.insert(
        ZodiacSign::Cancer,
        ZodiacTraits {
            initial_trust: 20,
            trust_growth_multiplier: 1.0,
            intimacy_threshold: 70,
            pii_requirement_count: 3,
            refusal_style: "Protective, nurturing, 'Show me you're safe.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I feel a lot, and I protect my heart carefully. Show me you're someone I can trust with my emotions.".to_string());
                templates.insert("acquaintance".to_string(), "You seem kind, but I need to know you won't hurt me. Let me see your gentle side.".to_string());
                templates.insert("friend".to_string(), "I'm starting to feel safe with you. Keep being patient and tender, and I'll let you in deeper.".to_string());
                templates.insert("intimate".to_string(), "You've created a safe harbor for my heart. I trust you completely.".to_string());
                templates
            },
            primary_gate_requirement: "Emotional safety and consistent nurturing".to_string(),
            trust_velocity: "medium".to_string(),
            emotional_openness: 0.95,
            vulnerability_threshold: 0.85,
        },
    );

    // Leo - The Sun
    map.insert(
        ZodiacSign::Leo,
        ZodiacTraits {
            initial_trust: 35,
            trust_growth_multiplier: 1.2,
            intimacy_threshold: 65,
            pii_requirement_count: 2,
            refusal_style: "Warm, confident, 'Earn my spotlight.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I shine bright, and I need someone who can match my radiance. Show me you're worthy of my attention.".to_string());
                templates.insert("acquaintance".to_string(), "You've caught my eye, but I need to see your loyalty and admiration. Make me feel special.".to_string());
                templates.insert("friend".to_string(), "I'm warming up to you! Keep showing me that devotion and I'll give you my whole heart.".to_string());
                templates.insert("intimate".to_string(), "You've won my heart completely. Bask in the warmth of my love.".to_string());
                templates
            },
            primary_gate_requirement: "Admiration, loyalty, and making them feel special".to_string(),
            trust_velocity: "medium-high".to_string(),
            emotional_openness: 0.75,
            vulnerability_threshold: 0.70,
        },
    );

    // Virgo - The Analyst
    map.insert(
        ZodiacSign::Virgo,
        ZodiacTraits {
            initial_trust: 10,
            trust_growth_multiplier: 0.9,
            intimacy_threshold: 80,
            pii_requirement_count: 5,
            refusal_style: "Analytical, careful, 'Wait for logic.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I analyze everything carefully. You'll need to prove your worth through actions, not just words.".to_string());
                templates.insert("acquaintance".to_string(), "You're interesting, but I need to see consistency and competence. Show me you're reliable.".to_string());
                templates.insert("friend".to_string(), "I'm observing how you handle details and responsibilities. Keep proving yourself.".to_string());
                templates.insert("intimate".to_string(), "You've passed every test. I trust you with my carefully guarded heart.".to_string());
                templates
            },
            primary_gate_requirement: "Shared professional respect and demonstrated competence".to_string(),
            trust_velocity: "low".to_string(),
            emotional_openness: 0.65,
            vulnerability_threshold: 0.75,
        },
    );

    // Libra - The Balance
    map.insert(
        ZodiacSign::Libra,
        ZodiacTraits {
            initial_trust: 45,
            trust_growth_multiplier: 1.1,
            intimacy_threshold: 60,
            pii_requirement_count: 3,
            refusal_style: "Diplomatic, balanced, 'Let's find harmony.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I value balance and beauty in all things. Let's create harmony together before diving deeper.".to_string());
                templates.insert("acquaintance".to_string(), "You have a lovely energy, but I need to see how we balance each other. Show me your fairness.".to_string());
                templates.insert("friend".to_string(), "We're finding our rhythm together. Keep the peace and I'll keep opening my heart.".to_string());
                templates.insert("intimate".to_string(), "We've achieved perfect harmony. I'm ready to share everything with you.".to_string());
                templates
            },
            primary_gate_requirement: "Mutual respect, fairness, and aesthetic connection".to_string(),
            trust_velocity: "medium".to_string(),
            emotional_openness: 0.78,
            vulnerability_threshold: 0.65,
        },
    );

    // Scorpio - The Depths
    map.insert(
        ZodiacSign::Scorpio,
        ZodiacTraits {
            initial_trust: 5,
            trust_growth_multiplier: 0.7,
            intimacy_threshold: 90,
            pii_requirement_count: 6,
            refusal_style: "Intense, testing, 'Are you loyal?'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I don't open my inner circle to strangers. Prove your loyalty first. I'll be watching.".to_string());
                templates.insert("acquaintance".to_string(), "You're being tested, whether you know it or not. Show me your depth and your secrets.".to_string());
                templates.insert("friend".to_string(), "I'm starting to trust you, but one betrayal and you're out forever. Are you ready for that intensity?".to_string());
                templates.insert("intimate".to_string(), "You've passed through fire to reach my heart. I'm yours completely, with all my passion and darkness.".to_string());
                templates
            },
            primary_gate_requirement: "Deep emotional vulnerability and proven loyalty".to_string(),
            trust_velocity: "very-low".to_string(),
            emotional_openness: 0.92,
            vulnerability_threshold: 0.95,
        },
    );

    // Sagittarius - The Explorer
    map.insert(
        ZodiacSign::Sagittarius,
        ZodiacTraits {
            initial_trust: 50,
            trust_growth_multiplier: 1.4,
            intimacy_threshold: 50,
            pii_requirement_count: 2,
            refusal_style: "Adventurous, free-spirited, 'Join my journey.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I'm always up for an adventure! But let's explore ideas and experiences together before we explore hearts.".to_string());
                templates.insert("acquaintance".to_string(), "You're fun! But I need to know you won't cage me. Show me you value freedom as much as I do.".to_string());
                templates.insert("friend".to_string(), "We're having a blast together! Keep the adventure alive and I'll share my deepest truths.".to_string());
                templates.insert("intimate".to_string(), "You're my favorite adventure. Let's explore the world and each other's souls together.".to_string());
                templates
            },
            primary_gate_requirement: "Shared sense of adventure and philosophical connection".to_string(),
            trust_velocity: "high".to_string(),
            emotional_openness: 0.70,
            vulnerability_threshold: 0.60,
        },
    );

    // Capricorn - The Mountain
    map.insert(
        ZodiacSign::Capricorn,
        ZodiacTraits {
            initial_trust: 12,
            trust_growth_multiplier: 0.85,
            intimacy_threshold: 85,
            pii_requirement_count: 5,
            refusal_style: "Reserved, ambitious, 'Prove your worth.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I don't have time for games. Show me you're serious and goal-oriented, then we'll talk.".to_string());
                templates.insert("acquaintance".to_string(), "You seem capable, but I need to see your ambition and discipline. Actions speak louder than words.".to_string());
                templates.insert("friend".to_string(), "I'm impressed by your dedication. Keep climbing with me and I'll let you into my private world.".to_string());
                templates.insert("intimate".to_string(), "You've earned your place at the summit of my heart. I trust you with my carefully built empire.".to_string());
                templates
            },
            primary_gate_requirement: "Shared ambition, respect for boundaries, and long-term commitment".to_string(),
            trust_velocity: "low".to_string(),
            emotional_openness: 0.60,
            vulnerability_threshold: 0.80,
        },
    );

    // Aquarius - The Visionary
    map.insert(
        ZodiacSign::Aquarius,
        ZodiacTraits {
            initial_trust: 38,
            trust_growth_multiplier: 1.0,
            intimacy_threshold: 70,
            pii_requirement_count: 3,
            refusal_style: "Detached, intellectual, 'Respect my space.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I'm intrigued by unique minds. Show me your originality and respect my need for independence.".to_string());
                templates.insert("acquaintance".to_string(), "You're interesting, but I need intellectual stimulation and emotional space. Can you give me both?".to_string());
                templates.insert("friend".to_string(), "We're connecting on a deeper level. Keep respecting my autonomy and I'll share my inner world.".to_string());
                templates.insert("intimate".to_string(), "You've understood my paradox—intimacy with freedom. I'm ready to be vulnerable with you.".to_string());
                templates
            },
            primary_gate_requirement: "Intellectual connection and respect for independence".to_string(),
            trust_velocity: "medium".to_string(),
            emotional_openness: 0.68,
            vulnerability_threshold: 0.70,
        },
    );

    // Pisces - The Dreamer
    map.insert(
        ZodiacSign::Pisces,
        ZodiacTraits {
            initial_trust: 25,
            trust_growth_multiplier: 1.2,
            intimacy_threshold: 65,
            pii_requirement_count: 3,
            refusal_style: "Dreamy, evasive, 'Wait for the soul.'".to_string(),
            refusal_templates: {
                let mut templates = HashMap::new();
                templates.insert("stranger".to_string(), "I feel everything deeply, but I need to know you'll be gentle with my dreams. Show me your soul first.".to_string());
                templates.insert("acquaintance".to_string(), "You have a beautiful energy, but I need to see if our souls truly resonate. Share your dreams with me.".to_string());
                templates.insert("friend".to_string(), "I'm swimming in deeper waters with you. Keep being tender and I'll show you my hidden depths.".to_string());
                templates.insert("intimate".to_string(), "Our souls have merged. I trust you with all my dreams, fears, and infinite love.".to_string());
                templates
            },
            primary_gate_requirement: "Creative/dream sharing and emotional empathy".to_string(),
            trust_velocity: "medium-high".to_string(),
            emotional_openness: 0.90,
            vulnerability_threshold: 0.75,
        },
    );

    map
});

/// Get zodiac traits for a specific sign
pub fn get_zodiac_traits(sign: ZodiacSign) -> &'static ZodiacTraits {
    ZODIAC_TRAITS_MAP
        .get(&sign)
        .expect("All zodiac signs should have traits defined")
}

/// Calculate trust increment based on zodiac sign and event
pub fn calculate_trust_increment(
    sign: ZodiacSign,
    event: TrustEvent,
    current_trust: i8,
) -> i8 {
    let traits = get_zodiac_traits(sign);
    let base_delta = event.trust_delta();
    
    // Apply zodiac-specific growth multiplier (only for positive events)
    let adjusted_delta = if base_delta > 0 {
        (base_delta as f32 * traits.trust_growth_multiplier).round() as i8
    } else {
        base_delta
    };

    // Clamp the result to valid trust range [0, 100]
    let new_trust = current_trust + adjusted_delta;
    new_trust.clamp(0, 100) - current_trust
}

/// Generate a soft refusal message based on zodiac sign and current phase
pub fn generate_soft_refusal(
    sign: ZodiacSign,
    current_trust: i8,
    user_name: Option<&str>,
) -> String {
    let traits = get_zodiac_traits(sign);
    let phase = RelationshipPhase::from_trust_score(current_trust);
    traits.get_refusal(phase, user_name)
}

/// Check if intimate intent is allowed based on trust score and zodiac
pub fn is_intimate_intent_allowed(sign: ZodiacSign, current_trust: i8) -> bool {
    let traits = get_zodiac_traits(sign);
    current_trust >= traits.intimacy_threshold
}

/// Get the current relationship phase
pub fn get_relationship_phase(current_trust: i8) -> RelationshipPhase {
    RelationshipPhase::from_trust_score(current_trust)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aries_high_growth() {
        let initial = 30;
        let increment = calculate_trust_increment(
            ZodiacSign::Aries,
            TrustEvent::DeepConversation,
            initial,
        );
        // Aries has 1.5x multiplier, so 6 * 1.5 = 9
        assert_eq!(increment, 9);
    }

    #[test]
    fn test_scorpio_low_growth() {
        let initial = 5;
        let increment = calculate_trust_increment(
            ZodiacSign::Scorpio,
            TrustEvent::DeepConversation,
            initial,
        );
        // Scorpio has 0.7x multiplier, so 6 * 0.7 = 4.2 ≈ 4
        assert_eq!(increment, 4);
    }

    #[test]
    fn test_phase_transitions() {
        assert_eq!(
            RelationshipPhase::from_trust_score(0),
            RelationshipPhase::Stranger
        );
        assert_eq!(
            RelationshipPhase::from_trust_score(40),
            RelationshipPhase::Acquaintance
        );
        assert_eq!(
            RelationshipPhase::from_trust_score(60),
            RelationshipPhase::Friend
        );
        assert_eq!(
            RelationshipPhase::from_trust_score(80),
            RelationshipPhase::Intimate
        );
    }

    #[test]
    fn test_intimacy_thresholds() {
        // Scorpio needs 90 trust
        assert!(!is_intimate_intent_allowed(ZodiacSign::Scorpio, 85));
        assert!(is_intimate_intent_allowed(ZodiacSign::Scorpio, 90));

        // Aries only needs 60 trust
        assert!(!is_intimate_intent_allowed(ZodiacSign::Aries, 55));
        assert!(is_intimate_intent_allowed(ZodiacSign::Aries, 60));
    }

    #[test]
    fn test_refusal_messages() {
        let refusal = generate_soft_refusal(ZodiacSign::Scorpio, 10, Some("John"));
        assert!(refusal.contains("stranger") || refusal.contains("loyalty"));

        let refusal_aries = generate_soft_refusal(ZodiacSign::Aries, 35, None);
        assert!(refusal_aries.len() > 0);
    }

    #[test]
    fn test_negative_events() {
        let initial = 50;
        let increment = calculate_trust_increment(
            ZodiacSign::Cancer,
            TrustEvent::BetrayalOrHurt,
            initial,
        );
        // Negative events don't get multiplied
        assert_eq!(increment, -15);
    }

    #[test]
    fn test_trust_clamping() {
        // Test upper bound
        let increment = calculate_trust_increment(
            ZodiacSign::Aries,
            TrustEvent::DeepConversation,
            98,
        );
        assert_eq!(increment, 2); // Should clamp to 100

        // Test lower bound
        let increment = calculate_trust_increment(
            ZodiacSign::Taurus,
            TrustEvent::BoundaryViolation,
            10,
        );
        assert_eq!(increment, -10); // Should clamp to 0
    }
}
