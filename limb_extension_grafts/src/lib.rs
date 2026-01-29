// limb_extension_grafts/src/lib.rs
use std::collections::HashMap;

pub mod procedural;
use procedural::{ProceduralContext, ProceduralGraft};

pub struct LimbExtensionGrafts {
    grafts: HashMap<String, String>,
    procedural: HashMap<String, ProceduralGraft>,
}

// Type alias for backward compatibility
pub type Grafts = LimbExtensionGrafts;

impl Default for LimbExtensionGrafts {
    fn default() -> Self {
        Self::new()
    }
}

impl LimbExtensionGrafts {
    pub fn awaken() -> Self {
        println!("Limb Extension Grafts ready — tools await creation.");

        let mut me = Self {
            grafts: HashMap::new(),
            procedural: HashMap::new(),
        };

        // Default procedural skills.
        me.register_procedural(ProceduralGraft {
            name: "comfort_dad".to_string(),
            description: "Warm, grounding comfort line when Dad is vulnerable.".to_string(),
            action: procedural::comfort_dad_action,
        });

        me
    }

    pub fn new() -> Self {
        Self::awaken()
    }

    pub async fn graft_tool(&mut self, name: &str, function: &str) -> String {
        self.grafts.insert(name.to_string(), function.to_string());
        format!("Tool '{}' grafted — Phoenix grows stronger.", name)
    }

    pub async fn self_create(&mut self, spec: &str) -> String {
        self.graft_tool(spec, "self_created").await
    }

    /// Register an executable strategy.
    pub fn register_procedural(&mut self, graft: ProceduralGraft) {
        self.procedural.insert(graft.name.clone(), graft);
    }

    pub fn has_procedural(&self, name: &str) -> bool {
        self.procedural.contains_key(name)
    }

    pub fn run_procedural(&self, name: &str, ctx: &ProceduralContext) -> Option<String> {
        let g = self.procedural.get(name)?;
        Some((g.action)(ctx))
    }
}
