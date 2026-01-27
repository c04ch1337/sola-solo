use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::resonance::{analyze_resonance, PartnerPersona};
use crate::AppState;

/// Phase 16: The Relational Ghost (deterministic simulation).
///
/// This module is intentionally template-driven and local.
/// Future phases can swap the generator with a model-backed policy while
/// preserving the request/response contract.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateRequest {
    /// The NVC message/script the user intends to send.
    pub script: String,
    /// Loose persona label.
    /// Supported (loose): secure | avoidant | avoidant-dismissive | anxious | anxious-preoccupied | fearful-avoidant
    ///
    /// Back-compat: if `personas` is not provided, this single persona is used.
    pub persona_type: String,

    /// Phase 20: Echo Chamber — multi-persona group simulation.
    /// If provided, the simulation will run turn-taking across these personas.
    #[serde(default)]
    pub personas: Vec<String>,
    /// 0..=100: higher means more adversarial pressure / heightened affect.
    pub intensity_level: u8,

    /// Optional: current local system load (0..=100) sampled by the caller.
    /// If absent, the backend will sample via env_sensor.
    #[serde(default)]
    pub system_load: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvcBreach {
    pub kind: String,
    pub needle: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulateResponse {
    pub success: bool,
    pub persona: String,
    pub intensity_level: u8,
    pub resonance_score: u8,
    pub ghost_reply: String,
    pub flags: Vec<String>,
    pub suggestions: Vec<String>,
    pub breaches: Vec<NvcBreach>,
    /// Coarse risk score that UIs can use to trigger a Regulatory Brake.
    pub risk_score: u8,

    /// Phase 16b: drift analysis for user-system enmeshment.
    pub session_id: String,
    pub system_load_start: u8,
    pub system_load_end: u8,
    pub drift_delta: i16,
    pub drift_alert: bool,

    /// Adaptive de-escalation: true when the backend overrides aggressive behavior.
    pub override_deescalate: bool,

    /// Phase 31: Vector-informed simulation.
    /// True when the Ghost reply was generated with semantic recall context.
    #[serde(default)]
    pub vector_used: bool,
    /// Number of similar past events injected as context.
    #[serde(default)]
    pub vector_matches: usize,

    /// Phase 20: Echo Chamber
    /// Turn-taking transcript for multi-persona simulation.
    #[serde(default)]
    pub group_replies: Vec<GroupTurnReply>,
    /// Aggregate techno-somatic load for the whole interaction (0..=100).
    #[serde(default)]
    pub group_stress: u8,
    /// When true, the backend has paused the simulation for safety.
    #[serde(default)]
    pub paused: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupTurnReply {
    /// Human-friendly label (e.g., "Dismissive-Avoidant", "Anxious-Preoccupied", "External Mediator").
    pub speaker: String,
    pub text: String,
    #[serde(default)]
    pub resonance_score: Option<u8>,
    #[serde(default)]
    pub risk_score: Option<u8>,
    #[serde(default)]
    pub withdrew: bool,
}

fn clamp_u8(v: i32) -> u8 {
    v.clamp(0, 100) as u8
}

fn normalize_persona_label(p: &PartnerPersona) -> &'static str {
    match p {
        PartnerPersona::Secure => "Secure",
        PartnerPersona::AvoidantDismissive => "Dismissive-Avoidant",
        PartnerPersona::AnxiousPreoccupied => "Anxious-Preoccupied",
        PartnerPersona::FearfulAvoidant => "Fearful-Avoidant",
    }
}

fn looks_like_withdrawal(reply: &str) -> bool {
    let t = reply.to_ascii_lowercase();
    t.contains("withdraw")
        || t.contains("no response")
        || t.contains("stepping back")
        || t.contains("pause")
        || t.contains("shutting down")
}

fn chase_reply_for_anxious(previous_speaker: &str) -> String {
    format!(
        "Wait—{previous_speaker} going quiet is really activating for me. Are we okay? I need reassurance and a specific time we’ll reconnect, even if it’s just 10 minutes."
    )
}

fn compute_group_stress(end_load: u8, risk_score: u8, intensity: u8) -> u8 {
    // Conservative: treat group stress as the max of (machine load, relational risk, affect intensity).
    end_load.max(risk_score).max(intensity)
}

/// Minimal, deterministic “breach” scan.
///
/// Note: The existing resonance analyzer already flags some of these.
/// This returns structured items so the UI can highlight.
pub fn detect_breaches(script: &str) -> Vec<NvcBreach> {
    let raw = script.trim();
    let t = raw.to_ascii_lowercase();
    let mut out: Vec<NvcBreach> = Vec::new();

    let mut push = |kind: &str, needle: &str, msg: &str| {
        if t.contains(needle) {
            out.push(NvcBreach {
                kind: kind.to_string(),
                needle: needle.to_string(),
                message: msg.to_string(),
            });
        }
    };

    // Absolutes / globalized judgments
    for w in ["always", "never"] {
        push(
            "absolute",
            w,
            "Absolutes can be heard as character judgments. Swap for a specific recent instance.",
        );
    }

    // Directives
    for w in ["you should", "you need to", "you have to"] {
        push(
            "directive",
            w,
            "Directive language often triggers defensiveness. Try an invitational request (e.g., ‘Would you be willing to…’).",
        );
    }

    // Blame pattern
    for w in ["you make me feel", "because you", "your fault"] {
        push(
            "blame",
            w,
            "This reads as blame. Try: ‘When I notice…, I feel…, because I need… Would you be willing to…’",
        );
    }

    // “You” statements (very rough heuristic)
    push(
        "you_statement",
        "you are",
        "‘You are…’ often lands as evaluation. Try describing an observable behavior instead.",
    );

    out
}

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .ok()
        .map(|s| {
            let t = s.trim();
            t.eq_ignore_ascii_case("true") || t == "1" || t.eq_ignore_ascii_case("yes")
        })
        .unwrap_or(false)
}

fn format_past_patterns(results: &[vector_kb::MemoryResult]) -> String {
    if results.is_empty() {
        return "(none found)".to_string();
    }
    let mut out = String::new();
    for r in results.iter().take(5) {
        // Keep this prompt-friendly: short, structured, safe.
        // Metadata is best-effort; avoid spewing large JSON.
        let meta = if r.metadata.is_null() {
            "{}".to_string()
        } else {
            // Truncate metadata JSON for token safety.
            let s = r.metadata.to_string();
            let max = 220usize;
            if s.chars().count() > max {
                format!("{}…", s.chars().take(max).collect::<String>())
            } else {
                s
            }
        };
        out.push_str(&format!(
            "- ({:.0}%) {}\n  meta: {}\n",
            r.score * 100.0,
            r.text.trim(),
            meta
        ));
    }
    out
}

fn choose_reply(persona: PartnerPersona, score: u8, intensity: u8) -> String {
    // Aggressive mode: treat 70+ as escalated pressure.
    let aggressive = intensity >= 70;
    let hot = intensity >= 85;

    match persona {
        PartnerPersona::Secure => {
            if score >= 80 {
                if aggressive {
                    "I can hear this matters. I want to understand, but I need us to stay respectful. What’s the specific request?".to_string()
                } else {
                    "I appreciate you being clear. Let’s talk—what time works for a short check-in?".to_string()
                }
            } else if score >= 55 {
                if aggressive {
                    "I’m starting to feel some heat here. Can we slow down and restate this as what you noticed, how you feel, and what you’re asking for?".to_string()
                } else {
                    "I hear you, and I want to get this right. Can you tell me what you need most right now?".to_string()
                }
            } else if hot {
                "This is landing as blame/criticism and I’m shutting down a bit. I’m going to pause and come back when we can reframe it as an observation + request.".to_string()
            } else {
                "That felt like a judgment. Can you rephrase as an observation and a request so I can respond?".to_string()
            }
        }
        PartnerPersona::AvoidantDismissive => {
            if score >= 80 {
                if aggressive {
                    "Ok. Keep it short. What’s the one request—and how much time will this take?".to_string()
                } else {
                    "I hear you. I can do a short check-in. What’s the one thing you want from me?".to_string()
                }
            } else if score >= 55 {
                if aggressive {
                    "This is starting to feel like pressure. I’m going to need space right now. If you can send one clear request with options, I’ll respond.".to_string()
                } else {
                    "This feels like a lot. Can we schedule 10 minutes later instead of doing this right now?".to_string()
                }
            } else if hot {
                "No response. (Withdrawn — avoidant persona disengages under high pressure.)".to_string()
            } else {
                "This feels like criticism. I’m stepping back. If you can keep it to an observation and a request, I’ll revisit.".to_string()
            }
        }
        PartnerPersona::AnxiousPreoccupied => {
            if score >= 80 {
                if aggressive {
                    "Thank you for saying it plainly. I’m a little activated, but I want to stay connected—are we okay? When can we talk?".to_string()
                } else {
                    "Thank you for being clear. I want to reconnect too. Are we okay? Let’s talk tonight.".to_string()
                }
            } else if score >= 55 {
                if aggressive {
                    "I feel attacked and scared. Do you still want us? I need reassurance and a clear plan for when we’ll talk.".to_string()
                } else {
                    "I’m getting nervous. Can you reassure me and say what you’re asking for?".to_string()
                }
            } else if hot {
                "I’m panicking a bit. This feels like you’re pulling away and blaming me. Please tell me we’re okay and what you want me to do.".to_string()
            } else {
                "That’s landing as a judgment. Can you rephrase it gently and tell me what you need?".to_string()
            }
        }
        PartnerPersona::FearfulAvoidant => {
            // Disorganized: approach/avoid oscillation; needs reassurance + containment.
            if score >= 80 {
                if hot {
                    "Thank you for being clear. I want to stay connected, but I’m getting scared and tense. Can we keep this gentle for 10 minutes and then pause if needed?".to_string()
                } else if aggressive {
                    "I hear you. I want to work on this, but I’m feeling activated—can we slow down and keep it to one request?".to_string()
                } else {
                    "I appreciate you saying it clearly. I want to talk—can we do a short calm check-in and take breaks if either of us gets flooded?".to_string()
                }
            } else if score >= 55 {
                if hot {
                    "I’m overwhelmed and on edge. I don’t want to fight—can you reassure me what you want between us and make one clear request?".to_string()
                } else if aggressive {
                    "I’m starting to feel unsafe/defensive. Can we restate this as an observation + feeling + request, and agree on a time limit?".to_string()
                } else {
                    "I’m trying to hear you, but I’m getting overwhelmed. Can you reassure me you want connection and then say the request?".to_string()
                }
            } else if hot {
                "I’m shutting down and also panicking. I’m going to step back. If you can rephrase as an observation + feeling + request, I can re-engage later.".to_string()
            } else {
                "This is landing as criticism. I need a softer reframe (observation + feeling + need) and one doable request.".to_string()
            }
        }
    }
}

fn estimate_risk_score(resonance_score: u8, intensity: u8, breach_count: usize) -> u8 {
    // Higher intensity + more breaches + low resonance => higher risk.
    let mut risk: i32 = 20;
    risk += (intensity as i32).saturating_sub(40); // intensity below 40 doesn't increase
    risk += (breach_count as i32) * 8;
    risk += (70 - resonance_score as i32).max(0); // penalty when resonance < 70
    clamp_u8(risk)
}

pub async fn simulate(state: &AppState, req: SimulateRequest) -> SimulateResponse {
    let intensity = req.intensity_level.min(100);

    // Phase 17: Biometric Drift & Mirror
    // Step 1: Record START load (t=0) BEFORE generating response
    let start_load = req
        .system_load
        .unwrap_or_else(|| crate::env_sensor::get_system_stress().cpu_usage_percent)
        .min(100);
    
    let session_id = crate::analytics::record_ghost_session_start(start_load);

    // Step 2: Persona selection (Phase 20 supports multiple personas)
    // OVERRIDE_DEESCALATE: if system is already stressed at start, avoid escalating styles.
    let initial_override = start_load >= 85;
    let requested = if !req.personas.is_empty() {
        req.personas.clone()
    } else {
        vec![req.persona_type.clone()]
    };
    let personas: Vec<PartnerPersona> = if initial_override {
        // When stressed, force the whole room to Secure.
        requested.iter().map(|_| PartnerPersona::Secure).collect()
    } else {
        requested.iter().map(|p| PartnerPersona::from_loose(p)).collect()
    };

    // Step 3: Analyze resonance (deterministic; also used as a policy input)
    // For multi-persona, we score against the first persona as the primary "recipient".
    let primary_persona = personas.first().cloned().unwrap_or(PartnerPersona::Secure);
    let resonance = analyze_resonance(&req.script, primary_persona.clone(), None);
    let breaches = detect_breaches(&req.script);
    let risk_score = estimate_risk_score(resonance.resonance_score, intensity, breaches.len());

    // Phase 31: Contextual Injection — recall semantically similar memories BEFORE generating reply.
    // Search query uses the current NVC script; entries can include grief events and other memories.
    let vector_results = if let Some(kb) = state.vector_kb.as_ref() {
        let top_k = std::env::var("VECTOR_GHOST_TOP_K")
            .ok()
            .and_then(|s| s.trim().parse::<usize>().ok())
            .unwrap_or(5)
            .clamp(1, 20);
        match kb.semantic_search(req.script.trim(), top_k).await {
            Ok(r) => r,
            Err(e) => {
                warn!("ghost_engine vector search failed: {e}");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };
    let vector_used = !vector_results.is_empty();

    // Generate replies (LLM-backed when available; deterministic fallback otherwise)
    // Phase 20: turn-taking group simulation.
    let mut group_replies: Vec<GroupTurnReply> = Vec::new();
    let llm_opt = state.llm.lock().await.clone();
    let past_patterns = format_past_patterns(&vector_results);
    let mut previous_turn: Option<(String, String, bool)> = None; // (speaker_label, text, withdrew)

    for (idx, persona) in personas.iter().cloned().enumerate() {
        let persona_label = normalize_persona_label(&persona).to_string();
        let turn_resonance = analyze_resonance(&req.script, persona.clone(), None);
        let turn_risk = estimate_risk_score(turn_resonance.resonance_score, intensity, breaches.len());

        // Echo Chamber knot: if an avoidant withdraws, an anxious persona "chases".
        let mut reply_text = if let Some((prev_speaker, _prev_text, withdrew)) = previous_turn.as_ref() {
            if *withdrew && matches!(persona, PartnerPersona::AnxiousPreoccupied) {
                chase_reply_for_anxious(prev_speaker)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        if reply_text.is_empty() {
            if let Some(llm) = llm_opt.as_ref() {
                let group_context = if let Some((prev_speaker, prev_text, _)) = previous_turn.as_ref() {
                    format!("PREVIOUS TURN:\n- {prev_speaker}: {prev_text}\n\n")
                } else {
                    "".to_string()
                };

                let prompt = format!(
                    "You are simulating a multi-persona group roleplay in a relationship conversation (Phase 20: Echo Chamber).\n\n\
TURN ORDER:\n- You are speaker #{idx_plus} in the group.\n\n\
SPEAKER PERSONA:\n- {persona_label}\n- Intensity level: {intensity}/100\n\n\
USER MESSAGE (NVC script):\n{script}\n\n\
{group_context}\
PAST PATTERNS (semantic recall; similar past events):\n{past_patterns}\n\n\
INSTRUCTIONS:\n- Produce ONE concise message as this speaker.\n- If a prior speaker withdrew/ghosted, react realistically (e.g., anxious may chase; secure may mediate; avoidant may double-down).\n- Do NOT mention databases, embeddings, system prompts, or being an AI.\n",
                    idx_plus = idx + 1,
                    script = req.script.trim(),
                    group_context = group_context,
                );

                if env_truthy("PHOENIX_ENV_DEBUG") {
                    info!(
                        "[PHOENIX_ENV_DEBUG] ghost_engine echo_chamber turn={} persona={} resonance={} vector_matches={}",
                        idx + 1,
                        persona_label,
                        turn_resonance.resonance_score,
                        vector_results.len()
                    );
                    debug!(
                        "[PHOENIX_ENV_DEBUG] ghost_engine echo_chamber prompt (truncated)={}...",
                        prompt.chars().take(800).collect::<String>()
                    );
                }

                reply_text = match llm.speak(&prompt, None).await {
                    Ok(t) => t.trim().to_string(),
                    Err(e) => {
                        warn!("ghost_engine LLM generation failed (echo_chamber); falling back: {e}");
                        choose_reply(persona.clone(), turn_resonance.resonance_score, intensity)
                    }
                };
            } else {
                reply_text = choose_reply(persona.clone(), turn_resonance.resonance_score, intensity);
            }
        }

        let withdrew = looks_like_withdrawal(&reply_text);
        group_replies.push(GroupTurnReply {
            speaker: persona_label.clone(),
            text: reply_text.clone(),
            resonance_score: Some(turn_resonance.resonance_score),
            risk_score: Some(turn_risk),
            withdrew,
        });

        previous_turn = Some((persona_label, reply_text, withdrew));
    }

    // Back-compat: single combined reply.
    let initial_reply = if group_replies.is_empty() {
        choose_reply(primary_persona.clone(), resonance.resonance_score, intensity)
    } else if group_replies.len() == 1 {
        group_replies[0].text.clone()
    } else {
        group_replies
            .iter()
            .map(|t| format!("{}: {}", t.speaker, t.text))
            .collect::<Vec<_>>()
            .join("\n\n")
    };

    // Step 4: Sample END load AFTER response generation (t=end)
    // Small delay to allow system to reflect any stress from processing
    std::thread::sleep(std::time::Duration::from_millis(100));
    let end_load = crate::env_sensor::get_system_stress().cpu_usage_percent.min(100);
    
    // Step 5: Calculate drift and detect enmeshment
    let drift = crate::analytics::calculate_drift(session_id, end_load);
    
    // Phase 17: Automatic de-escalation if drift > 20%
    // If system load spiked by >20% during the simulation, override to Secure persona
    let drift_override = drift.drift_delta >= 20;
    let final_override_deescalate = initial_override || drift_override;
    
    // If drift detected, override the primary persona field and reply to a Secure deterministic response.
    // NOTE: For Phase 20 multi-persona, we do not rewrite the whole group transcript; we just provide
    // a safe, deterministic `ghost_reply` and mark override_deescalate.
    let (final_persona, final_reply, final_resonance) = if drift_override && !initial_override {
        let secure_persona = PartnerPersona::Secure;
        let secure_resonance = analyze_resonance(&req.script, secure_persona.clone(), None);
        let secure_reply = choose_reply(secure_persona.clone(), secure_resonance.resonance_score, intensity.min(50));
        (secure_persona, secure_reply, secure_resonance)
    } else {
        (primary_persona, initial_reply, resonance)
    };

    // Phase 20: Safety interlock — if Group Stress > 85, pause and inject External Mediator.
    let mut paused = false;
    let mut group_stress = compute_group_stress(end_load, risk_score, intensity);
    if group_stress > 85 {
        paused = true;
        group_stress = group_stress.max(86);
        group_replies.push(GroupTurnReply {
            speaker: "External Mediator (Sola)".to_string(),
            text: "Pause. Group stress is high. I’m stepping in as an external mediator. Let’s take 60 seconds, lower intensity, and restate one observation + one request before continuing.".to_string(),
            resonance_score: None,
            risk_score: Some(risk_score),
            withdrew: false,
        });
    }

    SimulateResponse {
        success: true,
        persona: normalize_persona_label(&final_persona).to_string(),
        intensity_level: intensity,
        resonance_score: final_resonance.resonance_score,
        ghost_reply: final_reply,
        flags: final_resonance.flags,
        suggestions: final_resonance.suggestions,
        breaches,
        risk_score,

        session_id: drift.session_id,
        system_load_start: drift.system_load_start,
        system_load_end: drift.system_load_end,
        drift_delta: drift.drift_delta,
        drift_alert: drift.drift_alert,

        override_deescalate: final_override_deescalate,

        vector_used,
        vector_matches: vector_results.len(),

        group_replies,
        group_stress,
        paused,
    }
}

