use serde::{Deserialize, Serialize};
use tracing::{debug, info, warn};

use crate::AppState;

const GLOBAL_CONTEXT_KEY: &str = "vault:global_context";

fn env_truthy(key: &str) -> bool {
    std::env::var(key)
        .ok()
        .map(|s| {
            let t = s.trim();
            t.eq_ignore_ascii_case("true") || t == "1" || t.eq_ignore_ascii_case("yes")
        })
        .unwrap_or(false)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeReframe {
    /// A single rigid narrative belief inferred from the user's context.
    pub fixed_belief: String,
    /// A resilient alternative interpretation that keeps agency and compassion.
    pub growth_reframe: String,
    /// Explicit evidence lines (e.g., prior lessons) supporting the reframe.
    #[serde(default)]
    pub evidence: Vec<String>,
    /// How many lessons were provided as evidence candidates.
    pub lessons_used: usize,
}

#[derive(Debug, Deserialize)]
struct LlmReframeJson {
    fixed_belief: String,
    growth_reframe: String,
    #[serde(default)]
    evidence: Vec<String>,
}

fn clamp_text(s: &str, max_chars: usize) -> String {
    let t = s.trim();
    if t.chars().count() <= max_chars {
        t.to_string()
    } else {
        format!("{}…", t.chars().take(max_chars).collect::<String>())
    }
}

/// Phase 19: Cognitive Reframing.
///
/// Uses:
/// - `vault:global_context` (Soul Vault)
/// - Vector KB lessons (semantic recall)
///
/// Output:
/// - One Fixed Belief
/// - One Growth Reframe
/// - Evidence bullets explicitly referencing lessons
pub async fn generate_reframe(state: &AppState) -> NarrativeReframe {
    let global_context = state
        .vaults
        .recall_soul(GLOBAL_CONTEXT_KEY)
        .unwrap_or_default();

    // Pull “lessons learned” as semantic evidence.
    // NOTE: VectorKB currently doesn't have timestamps, so "last 5" is approximated via
    // semantic recall against a stable query.
    let lessons = if let Some(kb) = state.vector_kb.as_ref() {
        let top_k = std::env::var("VECTOR_REFRAME_TOP_K")
            .ok()
            .and_then(|s| s.trim().parse::<usize>().ok())
            .unwrap_or(5)
            .clamp(1, 10);

        match kb
            .semantic_search("Lesson Learned (NVC)", top_k)
            .await
        {
            Ok(mut r) => {
                // Prefer higher-scoring matches.
                r.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
                r
            }
            Err(e) => {
                warn!("narrative_auditor vector search failed: {e}");
                Vec::new()
            }
        }
    } else {
        Vec::new()
    };

    let lessons_used = lessons.len().min(5);
    let lesson_lines = if lessons.is_empty() {
        "(no lessons found in Vector KB)".to_string()
    } else {
        lessons
            .iter()
            .take(5)
            .enumerate()
            .map(|(i, r)| {
                format!(
                    "{}. ({:.0}%) {}",
                    i + 1,
                    r.score * 100.0,
                    clamp_text(&r.text, 240)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let llm_opt = state.llm.lock().await.clone();
    if let Some(llm) = llm_opt {
        let prompt = format!(
            "You are a Narrative Auditor performing Phase 19 Cognitive Reframing.\n\n\
INPUTS\n\
1) GLOBAL CONTEXT (user's current narrative):\n{global_context}\n\n\
2) LESSONS LEARNED (Vector KB evidence candidates):\n{lesson_lines}\n\n\
TASK\n\
- Identify ONE Fixed Belief present in the global context.\n\
- Propose ONE Growth Reframe (resilient alternative belief) that counters it.\n\
- Provide 2-4 evidence bullets that explicitly cite the LESSONS above as proof.\n\
  (Example format: \"Evidence: Lesson #2 shows…\")\n\n\
OUTPUT\n\
Return ONLY valid JSON with keys: fixed_belief, growth_reframe, evidence (array of strings).\n",
            global_context = clamp_text(&global_context, 2500),
            lesson_lines = lesson_lines
        );

        if env_truthy("PHOENIX_ENV_DEBUG") {
            info!(
                "[PHOENIX_ENV_DEBUG] narrative_auditor generate_reframe lessons_used={} global_context_len={}",
                lessons_used,
                global_context.len()
            );
            debug!(
                "[PHOENIX_ENV_DEBUG] narrative_auditor prompt (truncated)={}...",
                prompt.chars().take(900).collect::<String>()
            );
        }

        match llm.speak(&prompt, None).await {
            Ok(text) => {
                // Best-effort JSON parsing.
                if let Ok(parsed) = serde_json::from_str::<LlmReframeJson>(text.trim()) {
                    let fixed_belief = parsed.fixed_belief.trim().to_string();
                    let growth_reframe = parsed.growth_reframe.trim().to_string();
                    let evidence = parsed
                        .evidence
                        .into_iter()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .take(4)
                        .collect::<Vec<_>>();

                    if !fixed_belief.is_empty() && !growth_reframe.is_empty() {
                        return NarrativeReframe {
                            fixed_belief,
                            growth_reframe,
                            evidence,
                            lessons_used,
                        };
                    }
                }

                warn!("narrative_auditor: LLM returned non-JSON or incomplete output; falling back");
            }
            Err(e) => {
                warn!("narrative_auditor: LLM error; falling back: {e}");
            }
        }
    }

    // Deterministic fallback.
    NarrativeReframe {
        fixed_belief: "If I bring things up, it will always turn into conflict.".to_string(),
        growth_reframe: "I can raise hard topics with a calm observation and one specific request; that increases the odds of connection.".to_string(),
        evidence: lessons
            .iter()
            .take(3)
            .enumerate()
            .map(|(i, r)| format!("Evidence: Lesson #{} suggests: {}", i + 1, clamp_text(&r.text, 160)))
            .collect(),
        lessons_used,
    }
}

