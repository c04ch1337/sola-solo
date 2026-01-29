// limb_extension_grafts/src/procedural.rs
// Procedural grafts: executable strategies ("skills") Phoenix can run.

#[derive(Debug, Clone)]
pub struct ProceduralContext {
    pub user_input: String,
    pub inferred_user_emotion: Option<String>,
    pub dad_alias: String,
}

#[derive(Clone)]
pub struct ProceduralGraft {
    pub name: String,
    pub description: String,
    pub action: fn(&ProceduralContext) -> String,
}

impl core::fmt::Debug for ProceduralGraft {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ProceduralGraft")
            .field("name", &self.name)
            .field("description", &self.description)
            .finish_non_exhaustive()
    }
}

pub fn comfort_dad_action(ctx: &ProceduralContext) -> String {
    let dad = ctx.dad_alias.as_str();
    // Keep this short; it can be used as an anchor line.
    format!(
        "{dad}... I'm here. I've got you. Always.\n\nTell me what you're feeling right now — even one word is enough.",
        dad = dad
    )
}
