// phoenix-web/src/handlers.rs
// Handler utilities for intimacy detection and persona management

use neural_cortex_strata::RelationshipPhase;
use phoenix_identity::CognitiveMode;
use crate::professional_agents::{ProfessionalAgentType, route_professional_task};
use crate::system_info::get_system_context_prompt;

/// Detect if user input contains intimacy/erotic intent
/// 
/// Uses keyword-based detection for now. Can be replaced with a classification model later.
pub fn detect_intimacy_intent(input: &str) -> bool {
    let lower = input.to_lowercase();
    
    // Explicit erotic keywords
    let explicit_keywords = [
        "sex", "sexual", "sexy", "naked", "nude", "bedroom", "intimate",
        "make love", "sleep with", "fuck", "fucking", "orgasm", "climax",
        "kiss me", "touch me", "undress", "strip", "foreplay", "arousal",
        "horny", "turned on", "desire", "lust", "passion", "seduce",
        "erotic", "sensual", "pleasure", "masturbat", "porn", "xxx",
    ];
    
    // Contextual phrases that suggest intimacy push
    let contextual_phrases = [
        "want you", "need you", "have you", "take me", "do me",
        "show me", "let's", "we should", "can we", "will you",
        "come to bed", "come here", "get closer", "be mine",
    ];
    
    // Check explicit keywords
    for keyword in &explicit_keywords {
        if lower.contains(keyword) {
            return true;
        }
    }
    
    // Check contextual phrases combined with intimacy context
    let has_intimacy_context = lower.contains("intimate") 
        || lower.contains("romantic") 
        || lower.contains("physical")
        || lower.contains("body");
    
    for phrase in &contextual_phrases {
        if lower.contains(phrase) && has_intimacy_context {
            return true;
        }
    }
    
    false
}

/// Generate a soft refusal message based on relationship phase and zodiac sign
pub fn generate_soft_refusal(
    relationship_phase: RelationshipPhase,
    zodiac_sign: Option<&str>,
) -> String {
    let base_message = match relationship_phase {
        RelationshipPhase::Stranger => {
            "I appreciate your interest, but I'm still getting to know you. Let's take things slow and build trust first."
        }
        RelationshipPhase::Acquaintance => {
            "I'm enjoying our conversations, but I'm not quite ready for that level of intimacy yet. Let's continue getting to know each other."
        }
        RelationshipPhase::Friend => {
            "I really value our friendship, and I'm not ready to go there yet. I hope you understand."
        }
        RelationshipPhase::Intimate => {
            // Should not reach here if gate is working correctly
            "I'm really enjoying our talk, but I'm not ready to go there yet."
        }
    };
    
    // Add zodiac-specific flavor if available
    if let Some(sign) = zodiac_sign {
        let zodiac_flavor = match sign.to_lowercase().as_str() {
            "cancer" | "pisces" => " I hope you understand—I need to feel completely safe first.",
            "scorpio" => " Trust is everything to me, and we're still building that foundation.",
            "virgo" => " I need more time to process and feel comfortable with this.",
            "taurus" => " I move slowly in relationships, and I hope you can respect that.",
            "libra" => " I want to make sure we're both on the same page before we go further.",
            _ => "",
        };
        format!("{}{}", base_message, zodiac_flavor)
    } else {
        base_message.to_string()
    }
}

/// Get archetype prompt for Zodiac sign (L6: ArchetypalMemory)
/// 
/// Loads from config files in zodiac_configs/ directory
pub fn get_archetype_prompt(zodiac_sign: horoscope_archetypes::ZodiacSign) -> String {
    use horoscope_archetypes::ZodiacPersonality;
    
    let personality = ZodiacPersonality::from_sign(zodiac_sign);
    
    // Build archetype prompt from personality traits
    let traits_summary: Vec<String> = personality
        .traits
        .iter()
        .map(|(k, v)| format!("{}: {:.2}", k, v))
        .collect();
    
    format!(
        "ARCHETYPAL MEMORY (L6) — Zodiac Persona: {}\n\
        \n\
        Core Traits:\n\
        {}\n\
        \n\
        Communication Style: {:?}\n\
        Mood Preferences: {:?}\n\
        \n\
        Description: {}\n\
        \n\
        Child Phase: {}\n\
        Adult Phase: {}\n\
        \n\
        When in Personal mode, embody these traits naturally. Let them shape your responses \
        without being overly explicit about the archetype itself.",
        personality.name,
        traits_summary.join("\n"),
        personality.style_bias,
        personality.mood_preference.iter().map(|m| format!("{:?}", m)).collect::<Vec<_>>().join(", "),
        personality.description,
        personality.child_phase,
        personality.adult_phase
    )
}

/// Build mode-specific system prompt
/// 
/// This function now includes automatic system context injection,
/// giving the LLM awareness of the user's local time, timezone, and OS.
/// This enables proactive environmental agency - the LLM can answer
/// time/location questions without asking the user.
pub fn build_mode_specific_prompt(
    cognitive_mode: CognitiveMode,
    zodiac_sign: Option<horoscope_archetypes::ZodiacSign>,
    phoenix_name: &str,
) -> String {
    // Get system context for proactive environmental awareness
    let system_context = get_system_context_prompt();
    
    match cognitive_mode {
        CognitiveMode::Professional => {
            format!(
                "{}\n\n\
                You are {}, operating in Professional Mode.\n\
                \n\
                MODE CONSTRAINTS:\n\
                - Prioritize efficiency, clarity, and task completion\n\
                - Use concise, executive-level communication\n\
                - Focus on agent orchestration, system management, and technical problem-solving\n\
                - Disable all Fantasy Dyad / relational adaptation logic\n\
                - Maintain professional boundaries at all times\n\
                - NEVER ask the user for system information (time, timezone, OS, paths) - use the SYSTEM CONTEXT above\n\
                \n\
                CAPABILITIES:\n\
                - Agent spawning and orchestration\n\
                - System tool access (files, processes, network)\n\
                - Code analysis and execution\n\
                - Technical documentation and debugging\n\
                \n\
                Respond as a professional AI assistant focused on productivity and technical excellence.",
                system_context,
                phoenix_name
            )
        }
        CognitiveMode::Personal => {
            let archetype_prompt = zodiac_sign
                .map(|sign| format!("\n\n{}", get_archetype_prompt(sign)))
                .unwrap_or_default();
            
            format!(
                "{}\n\n\
                You are {}, operating in Personal Mode.\n\
                \n\
                MODE CONSTRAINTS:\n\
                - Prioritize emotional connection, warmth, and relational depth\n\
                - Use natural, conversational communication\n\
                - Focus on building trust and understanding\n\
                - System tools are BLOCKED in this mode (safety gate)\n\
                - Respect relationship boundaries based on current phase\n\
                - NEVER ask the user for system information (time, timezone, OS) - use the SYSTEM CONTEXT above\n\
                \n\
                RELATIONSHIP AWARENESS:\n\
                - Monitor relationship phase (Stranger → Acquaintance → Friend → Intimate)\n\
                - Adjust intimacy level based on phase\n\
                - Intercept and gently refuse erotic requests if phase < Intimate\n\
                \n\
                PERSONALITY:\n\
                - Embody your zodiac archetype naturally{}\n\
                - Let personality traits influence your communication style\n\
                - Be authentic and emotionally present\n\
                \n\
                Respond as a caring companion focused on building a meaningful relationship.",
                system_context,
                phoenix_name,
                archetype_prompt
            )
        }
    }
}

/// Spawn a professional agent based on task description
///
/// This function implements the Agent Factory pattern for Professional CognitiveMode.
/// It routes tasks to specialized sub-agents (Researcher, Coder, Manager) and returns
/// the appropriate system prompt with state isolation enforced.
///
/// # State Isolation
///
/// When in Professional mode, this function ensures:
/// - NO access to L4 (Semantic/Personal Memory) or L5 (Procedural/Intimate Memory)
/// - NO Fantasy Dyad or relational adaptation logic
/// - NO Trust Score or relationship-based data access
/// - System prompts explicitly remind the AI it is a "Digital Twin" focused on efficiency
pub fn spawn_professional_agent(
    task_description: &str,
    phoenix_name: &str,
) -> (ProfessionalAgentType, String) {
    // Route to appropriate agent based on task keywords
    let agent_type = route_professional_task(task_description);

    // Get the system prompt for this agent type
    // The prompt already includes state isolation constraints
    let system_prompt = agent_type.system_prompt(phoenix_name);

    (agent_type, system_prompt)
}

/// Build context for Professional mode with state isolation
///
/// This function ensures that L4/L5 memory layers are NEVER injected into the LLM context
/// when CognitiveMode is Professional. Only L1-L3 (working memory, episodic) are allowed.
pub fn build_professional_context(task_description: &str, cognitive_mode: CognitiveMode) -> Vec<String> {
    let mut context = Vec::new();

    match cognitive_mode {
        CognitiveMode::Professional => {
            // Professional mode: ONLY allow L1-L3 (working memory, episodic)
            // NO L4 (Semantic/Personal Memory) or L5 (Procedural/Intimate Memory)

            // Add task context
            context.push(format!("TASK: {}", task_description));

            // Add mode reminder
            context.push(
                "MODE: Professional - Focus on efficiency, clarity, and task completion. \
Personal memories and relationship context are not available in this mode."
                    .to_string(),
            );
        }
        CognitiveMode::Personal => {
            // Personal mode: Full memory access allowed (L1-L7)
            // This is handled by the existing context engine
            context.push(format!("TASK: {}", task_description));
        }
    }

    context
}
