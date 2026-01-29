use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

use crate::tools::video_scout::ScoutFilter;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissionStatus {
    Queued,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutMission {
    pub mission_id: String,
    pub title: String,
    pub query: String,
    pub status: MissionStatus,
    pub started_ms: u64,
    pub finished_ms: Option<u64>,
    pub enqueued_count: u32,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoutSettings {
    pub filter: ScoutFilter,
}

impl Default for ScoutSettings {
    fn default() -> Self {
        Self {
            filter: ScoutFilter {
                min_resolution: "1080p".to_string(),
                preferred_sources: Vec::new(),
                relevance_threshold: 0.8,
            },
        }
    }
}

#[derive(Debug, Default)]
struct ScoutMissionInner {
    missions: HashMap<String, ScoutMission>,
    settings: ScoutSettings,
}

#[derive(Debug, Default, Clone)]
pub struct ScoutMissionState {
    pub inner: Arc<RwLock<ScoutMissionInner>>,
}

impl ScoutMissionState {
    pub async fn settings(&self) -> ScoutSettings {
        self.inner.read().await.settings.clone()
    }

    pub async fn update_filter(&self, filter: ScoutFilter) {
        self.inner.write().await.settings.filter = filter;
    }

    pub async fn upsert_mission(&self, mission: ScoutMission) {
        self.inner
            .write()
            .await
            .missions
            .insert(mission.mission_id.clone(), mission);
    }

    pub async fn list_missions(&self) -> Vec<ScoutMission> {
        let mut out = self
            .inner
            .read()
            .await
            .missions
            .values()
            .cloned()
            .collect::<Vec<_>>();
        out.sort_by_key(|m| std::cmp::Reverse(m.started_ms));
        out
    }
}

