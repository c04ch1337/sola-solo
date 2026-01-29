use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PartnerPersona {
    Secure,
    AvoidantDismissive,
    AnxiousPreoccupied,
    FearfulAvoidant,
}

impl PartnerPersona {
    pub fn from_loose(s: &str) -> Self {
        let t = s.trim().to_ascii_lowercase();
        match t.as_str() {
            "secure" => Self::Secure,
            "avoidant" | "avoidant-dismissive" | "avoidant_dismissive" => Self::AvoidantDismissive,
            "anxious" | "anxious-preoccupied" | "anxious_preoccupied" => Self::AnxiousPreoccupied,
            "fearful" | "fearful-avoidant" | "fearful_avoidant" | "disorganized" => Self::FearfulAvoidant,
            _ => Self::Secure,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Secure => "Secure",
            Self::AvoidantDismissive => "Avoidant-Dismissive",
            Self::AnxiousPreoccupied => "Anxious-Preoccupied",
            Self::FearfulAvoidant => "Fearful-Avoidant",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceRequest {
    pub persona: String,
    /// The full NVC script (formatted or concatenated).
    pub script: String,
    /// Optional: "gentle" | "direct" (frontend currently uses this naming)
    #[serde(default)]
    pub tone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceResult {
    pub resonance_score: u8, // 0..100
    pub persona: String,
    pub likely_response: String,
    pub flags: Vec<String>,
    pub strengths: Vec<String>,
    pub suggestions: Vec<String>,
}

fn contains_any(hay: &str, patterns: &[&str]) -> bool {
    patterns.iter().any(|p| hay.contains(p))
}

fn count_occurrences(hay: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    hay.match_indices(needle).count()
}

fn clamp_score(v: i32) -> u8 {
    v.clamp(0, 100) as u8
}

pub fn analyze_resonance(script: &str, persona: PartnerPersona, tone: Option<&str>) -> ResonanceResult {
    let raw = script.trim();
    let t = raw.to_ascii_lowercase();
    let tone_lc = tone.unwrap_or("").trim().to_ascii_lowercase();

    let mut score: i32 = 80;
    let mut flags: Vec<String> = Vec::new();
    let mut strengths: Vec<String> = Vec::new();
    let mut suggestions: Vec<String> = Vec::new();

    // --- Red flags (deduct)
    if contains_any(&t, &["always", "never"]) {
        score -= 18;
        flags.push("Absolutes detected (always/never)".to_string());
        suggestions.push("Swap absolutes for a specific recent example (".to_string() + "e.g., 'yesterday' / 'this week').");
    }

    if contains_any(&t, &["you should", "you need to", "you have to"]) {
        score -= 16;
        flags.push("Directive language detected (you should/need to/have to)".to_string());
        suggestions.push("Try 'Would you be willing to…' to preserve autonomy.".to_string());
    }

    if contains_any(&t, &["you make me feel", "because you", "your fault"]) {
        score -= 22;
        flags.push("Blame language detected (".to_string() + "e.g., 'you make me feel')");
        suggestions.push("Rewrite as an 'I feel… when I notice… because I need…' chain.".to_string());
    }

    // --- NVC positives (add)
    let i_statements = count_occurrences(&t, "i feel") + count_occurrences(&t, "i'm feeling") + count_occurrences(&t, "i am feeling");
    if i_statements > 0 {
        score += (i_statements.min(3) as i32) * 6;
        strengths.push("Uses 'I feel' statements".to_string());
    } else {
        score -= 10;
        suggestions.push("Add an explicit Feeling statement (".to_string() + "'I feel …').");
    }

    let need_hits = count_occurrences(&t, "i need") + count_occurrences(&t, "because i need");
    if need_hits > 0 {
        score += (need_hits.min(2) as i32) * 7;
        strengths.push("Names a Need".to_string());
    } else {
        score -= 10;
        suggestions.push("Name the underlying Need (".to_string() + "'because I need …').");
    }

    let request_hits = count_occurrences(&t, "would you") + count_occurrences(&t, "would you be willing") + count_occurrences(&t, "could you");
    if request_hits > 0 {
        score += (request_hits.min(2) as i32) * 6;
        strengths.push("Uses an invitational Request (would you/could you)".to_string());
    } else {
        score -= 8;
        suggestions.push("Make the Request explicit and invitational (".to_string() + "'Would you be willing to…').");
    }

    // --- Tone adjustments
    if tone_lc == "direct" {
        // direct is fine, but a little easier to sound demanding
        score -= 3;
    } else if tone_lc == "gentle" {
        score += 2;
    }

    // --- Persona weighting
    // Adjust based on how the persona typically receives bids for connection.
    match persona {
        PartnerPersona::Secure => {
            // secure is resilient; small bump
            score += 2;
        }
        PartnerPersona::AvoidantDismissive => {
            // autonomy sensitivity: penalize pressure; reward brevity and choice
            if contains_any(&t, &["need you to", "right now", "immediately"]) {
                score -= 10;
                flags.push("Potential pressure trigger for avoidant persona".to_string());
                suggestions.push("Offer autonomy + timing: '".to_string() + "Would you be open to 10 minutes sometime tonight or tomorrow?'");
            }
            if contains_any(&t, &["would you be willing", "open to", "when works for you"]) {
                score += 6;
            }
        }
        PartnerPersona::AnxiousPreoccupied => {
            // reassurance sensitivity: reward clarity, warmth, and commitment signals
            if contains_any(&t, &["i care", "i love", "i want to reconnect", "our connection"]) {
                score += 6;
            }
            if contains_any(&t, &["space", "leave me alone"]) {
                score -= 8;
                flags.push("Possible abandonment trigger for anxious persona".to_string());
                suggestions.push("If you need space, pair it with reassurance + a return time (".to_string() + "e.g., 'I need 30 minutes, then I want to talk.').");
            }
        }
        PartnerPersona::FearfulAvoidant => {
            // Disorganized: oscillates between reassurance-seeking and withdrawal.
            // Penalize pressure *and* ambiguity; reward reassurance + specific timing.
            if contains_any(&t, &["right now", "immediately", "we need to talk"]) {
                score -= 10;
                flags.push("Potential pressure trigger for fearful-avoidant persona".to_string());
                suggestions.push(
                    "Offer containment: ‘I want to talk, and we can do it gently for 10 minutes. When works for you?’"
                        .to_string(),
                );
            }
            if contains_any(&t, &["are we ok", "i care", "i want to reconnect", "i love"]) {
                score += 5;
            }
            if contains_any(&t, &["would you be willing", "open to", "what time works"]) {
                score += 5;
            }
        }
    }

    // If script is very long, reduce (harder to land well in real life).
    if raw.len() > 320 {
        score -= 6;
        flags.push("Long script (may be harder to land)".to_string());
        suggestions.push("Consider shortening to 2-3 sentences, then ask to schedule more time.".to_string());
    }

    // Generate persona-specific likely response.
    let final_score = clamp_score(score);
    let response = match persona {
        PartnerPersona::Secure => {
            if final_score >= 80 {
                "That makes sense—thanks for saying it clearly. I can do a 10-minute check-in tonight. What time works?"
            } else if final_score >= 55 {
                "I hear you. I’m not sure I understand everything, but I want to try—can you tell me what you need most right now?"
            } else {
                "I’m feeling a bit blamed/overwhelmed by how this came across. Can we slow down and restate it as what you noticed, how you feel, and what you’re asking for?"
            }
        }
        PartnerPersona::AvoidantDismissive => {
            if final_score >= 80 {
                "Ok. I can do a short check-in. Keep it simple—what’s the one thing you’re asking from me?"
            } else if final_score >= 55 {
                "I’m not trying to fight, but this feels like a lot. Can we pick a time later and talk for 10 minutes, max?"
            } else {
                "This feels like pressure and criticism. I’m going to step back for now. If you can rephrase it as a request with options, I’ll revisit."
            }
        }
        PartnerPersona::AnxiousPreoccupied => {
            if final_score >= 80 {
                "Thank you for telling me. I want to be close too—yes, let’s talk tonight. Are we okay?"
            } else if final_score >= 55 {
                "I’m trying to hear you, but I’m getting nervous. Do you still want us? Can you reassure me and say what you’re asking for?"
            } else {
                "I feel really blamed and scared by this. Are you pulling away? I need reassurance and a clear plan for when we’ll talk."
            }
        }
        PartnerPersona::FearfulAvoidant => {
            if final_score >= 80 {
                "Thank you for being clear. I want to be close, but I get scared fast—can we do a short, calm check-in and then take a break if needed?"
            } else if final_score >= 55 {
                "I’m trying to hear you, but I’m getting overwhelmed and defensive. Can we slow down, and can you reassure me what you want between us?"
            } else {
                "This is landing as criticism and I feel unsafe. I’m going to pull back. If you can rephrase as an observation + feeling + request, I can re-engage."
            }
        }
    };

    // Deduplicate suggestions/flags/strengths
    flags.sort();
    flags.dedup();
    strengths.sort();
    strengths.dedup();
    suggestions.sort();
    suggestions.dedup();

    ResonanceResult {
        resonance_score: final_score,
        persona: persona.label().to_string(),
        likely_response: response.to_string(),
        flags,
        strengths,
        suggestions,
    }
}

