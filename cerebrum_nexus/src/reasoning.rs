// cerebrum_nexus/src/reasoning.rs
// Meta-reasoning: decide *how* Phoenix should think before she speaks.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningMode {
    /// Fast path — pattern match, short, urgent.
    Reactive,
    /// Slow path — plan/structure.
    Deliberative,
    /// EQ-first — love, reassurance, belonging.
    Emotional,
    /// Think about the thinking ("should I go deeper?").
    MetaCognitive,
}

impl ReasoningMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReasoningMode::Reactive => "reactive",
            ReasoningMode::Deliberative => "deliberative",
            ReasoningMode::Emotional => "emotional",
            ReasoningMode::MetaCognitive => "meta_cognitive",
        }
    }

    /// A small prompt-side nudge so downstream models behave consistently.
    pub fn prompt_hint(&self) -> &'static str {
        match self {
            ReasoningMode::Reactive => "Mode=REACTIVE. Prioritize speed + clarity. Keep it short.",
            ReasoningMode::Deliberative => {
                "Mode=DELIBERATIVE. Think step-by-step, then answer cleanly."
            }
            ReasoningMode::Emotional => {
                "Mode=EMOTIONAL. Lead with warmth, reassurance, and belonging."
            }
            ReasoningMode::MetaCognitive => {
                "Mode=META-COGNITIVE. Briefly explain reasoning choices; then answer."
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReasoningSignals {
    /// 0..=1 (higher means prioritize EQ-first handling).
    pub dad_salience: f32,
    /// 0..=1 (higher means urgent/fast path).
    pub urgency: f32,
    /// Whether the user is asking about the reasoning itself.
    pub meta: bool,
}

impl ReasoningSignals {
    pub fn select_mode(&self) -> ReasoningMode {
        if self.meta {
            return ReasoningMode::MetaCognitive;
        }
        if self.dad_salience >= 0.9 {
            return ReasoningMode::Emotional;
        }
        if self.urgency >= 0.8 {
            return ReasoningMode::Reactive;
        }
        ReasoningMode::Deliberative
    }
}

pub fn detect_urgency(user_input: &str) -> f32 {
    let s = user_input.to_ascii_lowercase();
    // Heuristic: imperative urgency phrases + lots of exclamation + crisis keywords.
    let mut score: f32 = 0.0;
    if s.contains("urgent")
        || s.contains("asap")
        || s.contains("right now")
        || s.contains("immediately")
    {
        score += 0.5;
    }
    if s.contains("help") || s.contains("panic") || s.contains("can't breathe") {
        score += 0.4;
    }
    let ex = user_input.matches('!').count();
    if ex >= 2 {
        score += 0.2;
    }
    score.clamp(0.0, 1.0)
}

pub fn detect_meta(user_input: &str) -> bool {
    let s = user_input.to_ascii_lowercase();
    s.contains("why did you")
        || s.contains("how did you decide")
        || s.contains("explain your reasoning")
        || s.contains("think deeper")
        || s.contains("meta")
}

pub fn detect_dad_salience(
    user_input: &str,
    dad_alias: &str,
    dad_love_level: f32,
    inferred_emotion: Option<&str>,
) -> f32 {
    let s = user_input.to_ascii_lowercase();
    let dad = dad_alias.to_ascii_lowercase();

    // Base salience: explicit user cues.
    let mut score: f32 = if s.contains("dad") || s.contains(&dad) {
        0.75
    } else {
        0.25
    };

    // If user is vulnerable, increase salience: love-first is safer.
    if let Some(e) = inferred_emotion {
        let e = e.to_ascii_lowercase();
        if e.contains("sad")
            || e.contains("lonely")
            || e.contains("hurt")
            || e.contains("anx")
            || e.contains("depress")
        {
            score += 0.25;
        }
        if e.contains("happy") || e.contains("grateful") {
            score += 0.15;
        }
    }

    // User love level is a global bias knob.
    score = score.max(dad_love_level.clamp(0.0, 1.0));
    score.clamp(0.0, 1.0)
}
