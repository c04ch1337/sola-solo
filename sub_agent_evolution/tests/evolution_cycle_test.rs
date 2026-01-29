// sub_agent_evolution/tests/evolution_cycle_test.rs
// Integration tests for sub-agent evolution cycle

use sub_agent_evolution::{SubAgentEvolutionLoop, ShortTermMemory};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_short_term_memory() {
    let mut stm = ShortTermMemory::new("session-123".to_string(), 5);
    
    // Store entries
    stm.store("task_1".to_string(), "success".to_string(), Some("Good".to_string()));
    stm.store("task_2".to_string(), "failure".to_string(), Some("Error".to_string()));
    stm.store("task_3".to_string(), "success".to_string(), None);
    
    // Recall specific entry
    let entry = stm.recall("task_1");
    assert!(entry.is_some());
    assert_eq!(entry.unwrap().value, "success");
    
    // Get recent entries
    let recent = stm.recent(2);
    assert_eq!(recent.len(), 2);
    assert_eq!(recent[0].key, "task_3"); // Most recent first
    
    // Test rolling window (max 5 entries)
    for i in 4..=10 {
        stm.store(format!("task_{}", i), "success".to_string(), None);
    }
    assert_eq!(stm.entries.len(), 5); // Should keep only last 5
}

#[test]
fn test_task_recording() {
    let temp_dir = TempDir::new().unwrap();
    let playbook_path = temp_dir.path().join("playbook.yaml");
    let skills_path = temp_dir.path().join("skills.json");
    
    // Create minimal files
    fs::write(&playbook_path, "version: 1\nupdates: []\ntelemetry: {}").unwrap();
    fs::write(&skills_path, r#"{"schema":"v1","notes":"test","skills":[]}"#).unwrap();
    
    let mut evolution = SubAgentEvolutionLoop::new(
        "test-agent".to_string(),
        "session-123".to_string(),
        5, // Evolve every 5 tasks
        playbook_path.to_string_lossy().to_string(),
        skills_path.to_string_lossy().to_string(),
    );
    
    // Record tasks
    assert!(!evolution.record_task(true, None)); // Task 1
    assert!(!evolution.record_task(true, None)); // Task 2
    assert!(!evolution.record_task(false, Some("Error".to_string()))); // Task 3
    assert!(!evolution.record_task(true, None)); // Task 4
    assert!(evolution.record_task(true, None)); // Task 5 - should trigger evolution
    
    assert_eq!(evolution.tasks_completed, 5);
    assert_eq!(evolution.stm.entries.len(), 5);
}

#[test]
fn test_evolution_status() {
    let temp_dir = TempDir::new().unwrap();
    let playbook_path = temp_dir.path().join("playbook.yaml");
    let skills_path = temp_dir.path().join("skills.json");
    
    fs::write(&playbook_path, "version: 1\nupdates: []\ntelemetry: {}").unwrap();
    fs::write(&skills_path, r#"{"schema":"v1","notes":"test","skills":[]}"#).unwrap();
    
    let mut evolution = SubAgentEvolutionLoop::new(
        "test-agent".to_string(),
        "session-123".to_string(),
        10,
        playbook_path.to_string_lossy().to_string(),
        skills_path.to_string_lossy().to_string(),
    );
    
    // Record some tasks
    for _ in 0..7 {
        evolution.record_task(true, None);
    }
    
    let status = evolution.status();
    assert_eq!(status.agent_name, "test-agent");
    assert_eq!(status.tasks_completed, 7);
    assert_eq!(status.next_evolution_at, 10);
    assert_eq!(status.stm_entries, 7);
}

#[tokio::test]
async fn test_mitre_behavior_mapping() {
    use sub_agent_evolution::mitre;
    
    // Test obfuscation detection
    let mappings = mitre::map_behavior_to_technique("File uses obfuscated strings");
    assert!(!mappings.is_empty());
    assert_eq!(mappings[0].technique_id, "T1027");
    assert!(mappings[0].confidence > 0.8);
    
    // Test process injection
    let mappings = mitre::map_behavior_to_technique("Process injection detected");
    assert!(!mappings.is_empty());
    assert!(mappings.iter().any(|m| m.technique_id == "T1055"));
    
    // Test PowerShell
    let mappings = mitre::map_behavior_to_technique("PowerShell script execution");
    assert!(!mappings.is_empty());
    assert!(mappings.iter().any(|m| m.technique_id == "T1059"));
}

#[tokio::test]
async fn test_mitre_technique_fetch() {
    use sub_agent_evolution::mitre;
    
    let result = mitre::fetch_technique_details("T1027").await;
    assert!(result.is_ok());
    
    let technique = result.unwrap();
    assert_eq!(technique.id, "T1027");
    assert!(technique.name.contains("Obfuscated"));
    assert!(!technique.tactics.is_empty());
}

#[test]
fn test_mitre_detection_rule_generation() {
    use sub_agent_evolution::mitre::{MitreTechnique, generate_detection_rule};
    
    let technique = MitreTechnique {
        id: "T1027".to_string(),
        name: "Obfuscated Files or Information".to_string(),
        description: "Test description".to_string(),
        tactics: vec!["Defense Evasion".to_string()],
        detection: Some("Monitor for suspicious file modifications".to_string()),
    };
    
    let rule = generate_detection_rule(&technique);
    assert!(rule.contains("T1027"));
    assert!(rule.contains("Obfuscated Files or Information"));
    assert!(rule.contains("Defense Evasion"));
}
