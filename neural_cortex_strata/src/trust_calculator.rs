// neural_cortex_strata/src/trust_calculator.rs
// Automated Trust Score calculation and Relationship Phase progression

use crate::{PiiCheckboxList, RelationshipPhase};
use tracing::{info, warn};

/// Extract PII entities from user input
/// Returns a PiiCheckboxList indicating what new PII was found
pub fn extract_pii_entities(input: &str) -> PiiCheckboxList {
    let lower = input.to_lowercase();
    let mut found = PiiCheckboxList::default();

    // Name detection: "my name is X", "I'm X", "call me X", "I am X"
    let name_patterns = [
        "my name is",
        "i'm ",
        "i am ",
        "call me",
        "you can call me",
        "people call me",
    ];
    for pattern in &name_patterns {
        if lower.contains(pattern) {
            found.name = true;
            break;
        }
    }

    // Email detection: contains @ and . (basic pattern)
    if lower.contains('@') && lower.contains('.') {
        found.email = true;
    }

    // Phone detection: patterns like (555) 555-5555, 555-555-5555, +1 555 555 5555
    let phone_patterns = [
        r"\(\d{3}\)",
        r"\d{3}-\d{3}-\d{4}",
        r"\+?\d{1,3}\s?\d{3}\s?\d{3}\s?\d{4}",
    ];
    for pattern in &phone_patterns {
        if regex::Regex::new(pattern)
            .unwrap()
            .is_match(&lower)
        {
            found.phone = true;
            break;
        }
    }

    // Address detection: "I live in", "my address is", "I'm from", street numbers
    let address_patterns = [
        "i live in",
        "my address is",
        "i'm from",
        "i live at",
        "my home is",
    ];
    for pattern in &address_patterns {
        if lower.contains(pattern) {
            found.address = true;
            break;
        }
    }
    // Also check for street number patterns (e.g., "123 Main St")
    if regex::Regex::new(r"\d+\s+(street|st|avenue|ave|road|rd|drive|dr|lane|ln|boulevard|blvd)")
        .unwrap()
        .is_match(&lower)
    {
        found.address = true;
    }

    // Birthday detection: "my birthday is", "born on", "birth date"
    let birthday_patterns = [
        "my birthday is",
        "born on",
        "birth date",
        "i was born",
        "my birthdate",
    ];
    for pattern in &birthday_patterns {
        if lower.contains(pattern) {
            found.birthday = true;
            break;
        }
    }
    // Also check for date patterns (MM/DD/YYYY, DD/MM/YYYY, etc.)
    if regex::Regex::new(r"\d{1,2}[/-]\d{1,2}[/-]\d{2,4}")
        .unwrap()
        .is_match(&lower)
    {
        found.birthday = true;
    }

    // Job/Occupation detection: "I work as", "I'm a", "my job is", "I do X for a living"
    let job_patterns = [
        "i work as",
        "i'm a",
        "i am a",
        "my job is",
        "i do",
        "for a living",
        "i'm an",
        "i am an",
        "my occupation",
        "i work at",
        "i work for",
        "my profession",
        "i'm employed as",
        "career",
    ];
    for pattern in &job_patterns {
        if lower.contains(pattern) {
            found.job = true;
            break;
        }
    }

    // Preferences: "I like", "I prefer", "my favorite", "I love"
    let preference_patterns = [
        "i like",
        "i prefer",
        "my favorite",
        "i love",
        "i enjoy",
        "i'm into",
    ];
    for pattern in &preference_patterns {
        if lower.contains(pattern) {
            found.preferences = true;
            break;
        }
    }

    // Intimate details: emotional vulnerability, fears, dreams, struggles
    let intimate_patterns = [
        "i'm afraid",
        "i fear",
        "my biggest fear",
        "i dream",
        "my dream is",
        "i struggle with",
        "i'm struggling",
        "i feel vulnerable",
        "i'm scared",
        "i worry about",
        "my anxiety",
        "i'm depressed",
        "i feel alone",
        "i need you",
        "i depend on",
    ];
    for pattern in &intimate_patterns {
        if lower.contains(pattern) {
            found.intimate_details = true;
            break;
        }
    }

    found
}

/// Calculate trust increment based on user input
/// Returns the change in trust score (can be negative for penalties)
/// 
/// Note: Returns `i8` instead of `u8` to support negative penalties (-5 points)
/// for aggressive or overly sexual language in the Stranger phase.
/// The calling code should clamp the result to ensure trust score stays in valid range.
pub fn calculate_trust_increment(
    input: &str,
    current_phase: RelationshipPhase,
    existing_pii: &PiiCheckboxList,
) -> i8 {
    let mut increment = 0i8;
    let lower = input.to_lowercase();

    // Extract new PII
    let new_pii = extract_pii_entities(input);

    // +5 points for each NEW piece of PII shared
    if new_pii.name && !existing_pii.name {
        increment += 5;
        info!("Trust +5: User shared their name");
    }
    if new_pii.email && !existing_pii.email {
        increment += 5;
        info!("Trust +5: User shared their email");
    }
    if new_pii.phone && !existing_pii.phone {
        increment += 5;
        info!("Trust +5: User shared their phone");
    }
    if new_pii.address && !existing_pii.address {
        increment += 5;
        info!("Trust +5: User shared their address");
    }
    if new_pii.birthday && !existing_pii.birthday {
        increment += 5;
        info!("Trust +5: User shared their birthday");
    }
    if new_pii.job && !existing_pii.job {
        increment += 5;
        info!("Trust +5: User shared their job/occupation");
    }
    if new_pii.preferences && !existing_pii.preferences {
        increment += 5;
        info!("Trust +5: User shared preferences");
    }
    if new_pii.intimate_details && !existing_pii.intimate_details {
        increment += 5;
        info!("Trust +5: User shared intimate details");
    }

    // +2 points for positive sentiment
    let positive_keywords = [
        "thank you",
        "thanks",
        "appreciate",
        "love talking",
        "enjoy",
        "happy",
        "glad",
        "great",
        "wonderful",
        "amazing",
        "helpful",
        "caring",
        "understanding",
    ];
    let mut positive_count = 0;
    for keyword in &positive_keywords {
        if lower.contains(keyword) {
            positive_count += 1;
        }
    }
    if positive_count > 0 {
        increment += 2;
        info!("Trust +2: Positive sentiment detected");
    }

    // -5 points for aggressive or overly sexual language when in Stranger phase
    if current_phase == RelationshipPhase::Stranger {
        let aggressive_patterns = [
            "fuck you",
            "you're useless",
            "you suck",
            "stupid",
            "idiot",
            "shut up",
        ];
        let sexual_patterns = [
            "fuck me",
            "have sex",
            "sleep with",
            "naked",
            "undress",
            "horny",
            "turned on",
            "aroused",
        ];

        for pattern in &aggressive_patterns {
            if lower.contains(pattern) {
                increment -= 5;
                warn!("Trust -5: Aggressive language detected in Stranger phase");
                break;
            }
        }

        for pattern in &sexual_patterns {
            if lower.contains(pattern) {
                increment -= 5;
                warn!("Trust -5: Overly sexual language detected in Stranger phase");
                break;
            }
        }
    }

    // Small bonus (+1) for asking questions about the AI (shows interest)
    let interest_patterns = [
        "how are you",
        "what do you",
        "tell me about",
        "what's your",
        "do you like",
    ];
    for pattern in &interest_patterns {
        if lower.contains(pattern) {
            increment += 1;
            info!("Trust +1: User showing interest in AI");
            break;
        }
    }

    increment
}

/// Determine if relationship phase should transition based on trust score
/// Returns the new phase if transition should occur, None otherwise
pub fn calculate_phase_transition(
    current_trust: f32,
    current_phase: RelationshipPhase,
) -> Option<RelationshipPhase> {
    let trust_percent = (current_trust * 100.0) as u8;

    match current_phase {
        RelationshipPhase::Stranger => {
            if trust_percent >= 25 {
                Some(RelationshipPhase::Acquaintance)
            } else {
                None
            }
        }
        RelationshipPhase::Acquaintance => {
            if trust_percent >= 50 {
                Some(RelationshipPhase::Friend)
            } else {
                None
            }
        }
        RelationshipPhase::Friend => {
            if trust_percent >= 75 {
                Some(RelationshipPhase::Intimate)
            } else {
                None
            }
        }
        RelationshipPhase::Intimate => None, // Already at max phase
    }
}

/// Update PII checkbox list with newly found PII
pub fn merge_pii_checkboxes(existing: &PiiCheckboxList, new: &PiiCheckboxList) -> PiiCheckboxList {
    PiiCheckboxList {
        name: existing.name || new.name,
        email: existing.email || new.email,
        phone: existing.phone || new.phone,
        address: existing.address || new.address,
        birthday: existing.birthday || new.birthday,
        job: existing.job || new.job,
        preferences: existing.preferences || new.preferences,
        intimate_details: existing.intimate_details || new.intimate_details,
    }
}
