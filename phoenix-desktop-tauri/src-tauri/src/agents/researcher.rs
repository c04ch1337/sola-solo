use crate::{
    audit,
    sola_state::{OrchestratorMode, SolaState},
};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MemoryInjection {
    pub mode: OrchestratorMode,
    pub target_layer: &'static str,
    pub title: String,
    pub body_markdown: String,
    pub sources: Vec<String>,
}

/// A mode-aware researcher. For now, this is a thin wrapper around the scraping backend.
///
/// Build note: the actual browser automation implementation is behind the `research` feature
/// to avoid forcing Chromium dependencies in minimal builds.
pub struct ResearchSession;

impl ResearchSession {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "research")]
mod impl_research {
    use super::*;
    use headless_chrome::{Browser, LaunchOptionsBuilder};
    use std::time::Duration;

    fn deep_browsing_profile_dir() -> Result<std::path::PathBuf, String> {
        let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
        Ok(cwd.join("vault").join("browser_profiles").join("personal"))
    }

    fn ephemeral_profile_dir() -> Result<std::path::PathBuf, String> {
        let cwd = std::env::current_dir().map_err(|e| e.to_string())?;
        Ok(cwd.join("vault").join("browser_profiles").join("professional_ephemeral"))
    }

    fn build_browser(mode: OrchestratorMode) -> Result<Browser, String> {
        // Mode-aware profile isolation:
        // - Professional: use an ephemeral/isolated profile directory (no personal cookies/history).
        // - Personal: allow "deep browsing" profile (persistent cookies/state).
        let user_data_dir = match mode {
            OrchestratorMode::Professional => Some(ephemeral_profile_dir()?),
            OrchestratorMode::Personal => Some(deep_browsing_profile_dir()?),
        };

        let mut opts = LaunchOptionsBuilder::default();
        opts = opts
            .headless(true)
            .idle_browser_timeout(Duration::from_secs(30));

        if let Some(dir) = user_data_dir {
            // Create the directory so Chrome can lock it.
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            opts = opts.user_data_dir(Some(dir));
        }

        Browser::new(opts.build().map_err(|e| e.to_string())?).map_err(|e| e.to_string())
    }

    pub fn gather_academic_data_impl(query: String, mode: OrchestratorMode) -> Result<MemoryInjection, String> {
        // NOTE: This is a minimal placeholder implementation.
        // Real implementation should navigate, extract titles/abstracts, and cite stable URLs.
        let _browser = build_browser(mode)?;

        Ok(MemoryInjection {
            mode,
            target_layer: "L5",
            title: format!("Academic research: {query}"),
            body_markdown: format!(
                "## Query\n- `{}`\n\n## Summary\n- (placeholder) Collected academic signals via headless_chrome.\n",
                query
            ),
            sources: vec!["https://scholar.google.com".to_string()],
        })
    }

    pub fn gather_companion_insights_impl(target_kink: String, mode: OrchestratorMode) -> Result<MemoryInjection, String> {
        let _browser = build_browser(mode)?;

        Ok(MemoryInjection {
            mode,
            target_layer: "L7",
            title: format!("Companion insights: {target_kink}"),
            body_markdown: format!(
                "## Topic\n- `{}`\n\n## Notes\n- (placeholder) Mode-aware deep browsing enabled in Personal mode only.\n",
                target_kink
            ),
            sources: vec!["(personal browsing profile)".to_string()],
        })
    }
}

#[cfg(not(feature = "research"))]
mod impl_research {
    use super::*;

    pub fn gather_academic_data_impl(_query: String, _mode: OrchestratorMode) -> Result<MemoryInjection, String> {
        Err("Research module not enabled. Rebuild backend with --features research".to_string())
    }

    pub fn gather_companion_insights_impl(_target_kink: String, _mode: OrchestratorMode) -> Result<MemoryInjection, String> {
        Err("Research module not enabled. Rebuild backend with --features research".to_string())
    }
}

impl ResearchSession {
    pub async fn gather_academic_data(state: &SolaState, query: String) -> Result<MemoryInjection, String> {
        let mode = state.inner.read().await.current_mode;
        if mode != OrchestratorMode::Professional {
            audit::append_line(
                "research_audit.log",
                "research_denied action=gather_academic_data reason=wrong_mode",
            )?;
            return Err("Academic research is restricted to Professional mode".to_string());
        }

        let inj = impl_research::gather_academic_data_impl(query, mode)?;
        audit::append_line(
            "research_audit.log",
            &format!("research_ok action=gather_academic_data target_layer={} mode={:?}", inj.target_layer, mode),
        )?;
        Ok(inj)
    }

    pub async fn gather_companion_insights(state: &SolaState, target_kink: String) -> Result<MemoryInjection, String> {
        let mode = state.inner.read().await.current_mode;
        if mode != OrchestratorMode::Personal {
            audit::append_line(
                "research_audit.log",
                "research_denied action=gather_companion_insights reason=wrong_mode",
            )?;
            return Err("Companion insights are restricted to Personal mode".to_string());
        }

        let inj = impl_research::gather_companion_insights_impl(target_kink, mode)?;
        audit::append_line(
            "research_audit.log",
            &format!("research_ok action=gather_companion_insights target_layer={} mode={:?}", inj.target_layer, mode),
        )?;
        Ok(inj)
    }
}

