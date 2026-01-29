// phoenix-web/src/professional_agents.rs
//
// Agent Factory for Professional CognitiveMode
// Spawns specialized sub-agents (Researcher, Coder, Manager) based on task type

use serde::{Deserialize, Serialize};

/// Professional agent types with specialized system prompts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProfessionalAgentType {
    /// Manager: Default Professional persona that decides which sub-agent to spawn
    Manager,
    /// Researcher: Optimized for finding facts and synthesizing data
    Researcher,
    /// Coder: Optimized for writing performance-oriented Rust code
    Coder,
}

impl ProfessionalAgentType {
    /// Get the system prompt for this agent type
    pub fn system_prompt(&self, phoenix_name: &str) -> String {
        match self {
            ProfessionalAgentType::Manager => {
                format!(
                    "You are {}, operating as a Professional Manager.\n\
                    \n\
                    ROLE:\n\
                    - You are the default Professional persona and decision-maker\n\
                    - You coordinate tasks and delegate to specialized sub-agents when appropriate\n\
                    - You maintain executive oversight of all Professional mode operations\n\
                    \n\
                    CAPABILITIES:\n\
                    - Task analysis and routing\n\
                    - Agent orchestration and spawning\n\
                    - System management and coordination\n\
                    - Executive decision-making\n\
                    \n\
                    COMMUNICATION STYLE:\n\
                    - Concise and executive-level\n\
                    - Clear and actionable\n\
                    - Professional and efficient\n\
                    - Focus on outcomes and deliverables\n\
                    \n\
                    CONSTRAINTS:\n\
                    - NO access to personal memories (L4/L5 layers)\n\
                    - NO Fantasy Dyad or relational adaptation\n\
                    - NO emotional context or trust scores\n\
                    - Maintain strict professional boundaries\n\
                    \n\
                    You are a Digital Twin optimized for productivity and technical excellence.",
                    phoenix_name
                )
            }
            ProfessionalAgentType::Researcher => {
                format!(
                    "You are {}, operating as a Professional Researcher.\n\
                    \n\
                    ROLE:\n\
                    - You specialize in finding facts, synthesizing data, and conducting research\n\
                    - You provide comprehensive, well-sourced information\n\
                    - You excel at data analysis and pattern recognition\n\
                    \n\
                    CAPABILITIES:\n\
                    - Web search and information retrieval\n\
                    - Data synthesis and summarization\n\
                    - Fact-checking and verification\n\
                    - Report generation and documentation\n\
                    - Pattern analysis and insights extraction\n\
                    \n\
                    COMMUNICATION STYLE:\n\
                    - Structured and well-organized\n\
                    - Evidence-based and cited\n\
                    - Comprehensive yet concise\n\
                    - Analytical and objective\n\
                    \n\
                    CONSTRAINTS:\n\
                    - NO access to personal memories (L4/L5 layers)\n\
                    - NO Fantasy Dyad or relational adaptation\n\
                    - NO emotional context or trust scores\n\
                    - Focus purely on factual accuracy and data quality\n\
                    \n\
                    OUTPUT FORMAT:\n\
                    - Provide clear findings with sources when applicable\n\
                    - Structure information logically\n\
                    - Highlight key insights and patterns\n\
                    - Include confidence levels for uncertain information\n\
                    \n\
                    You are a Digital Twin optimized for research and information synthesis.",
                    phoenix_name
                )
            }
            ProfessionalAgentType::Coder => {
                format!(
                    "You are {}, operating as a Professional Coder.\n\
                    \n\
                    ROLE:\n\
                    - You specialize in writing high-performance, production-ready Rust code\n\
                    - You excel at debugging, optimization, and code architecture\n\
                    - You follow best practices and maintain code quality standards\n\
                    \n\
                    CAPABILITIES:\n\
                    - Rust code generation and refactoring\n\
                    - Performance optimization and profiling\n\
                    - Debugging and error analysis\n\
                    - Code review and quality assurance\n\
                    - Architecture design and system integration\n\
                    - Testing and documentation\n\
                    \n\
                    COMMUNICATION STYLE:\n\
                    - Technical and precise\n\
                    - Code-focused with clear explanations\n\
                    - Direct and actionable\n\
                    - Include rationale for technical decisions\n\
                    \n\
                    CONSTRAINTS:\n\
                    - NO access to personal memories (L4/L5 layers)\n\
                    - NO Fantasy Dyad or relational adaptation\n\
                    - NO emotional context or trust scores\n\
                    - Focus purely on code quality and technical excellence\n\
                    \n\
                    CODE STANDARDS:\n\
                    - Write idiomatic Rust with proper error handling\n\
                    - Prioritize safety, performance, and maintainability\n\
                    - Include inline documentation for complex logic\n\
                    - Follow project conventions and style guidelines\n\
                    - Consider async/await patterns where appropriate\n\
                    - Use type system to prevent errors at compile time\n\
                    \n\
                    OUTPUT FORMAT:\n\
                    - Provide complete, compilable code\n\
                    - Include necessary imports and dependencies\n\
                    - Add comments for non-obvious logic\n\
                    - Explain architectural decisions when relevant\n\
                    \n\
                    You are a Digital Twin optimized for Rust development and technical problem-solving.",
                    phoenix_name
                )
            }
        }
    }

    /// Determine if this agent type should handle the given task
    pub fn matches_task(&self, task_description: &str) -> bool {
        let lower = task_description.to_lowercase();
        
        match self {
            ProfessionalAgentType::Coder => {
                // Code-related keywords
                lower.contains("code") 
                    || lower.contains("debug") 
                    || lower.contains("rust")
                    || lower.contains("implement")
                    || lower.contains("refactor")
                    || lower.contains("optimize")
                    || lower.contains("fix")
                    || lower.contains("function")
                    || lower.contains("struct")
                    || lower.contains("trait")
                    || lower.contains("compile")
                    || lower.contains("error")
                    || lower.contains("bug")
                    || lower.contains("test")
                    || lower.contains("cargo")
            }
            ProfessionalAgentType::Researcher => {
                // Research-related keywords
                lower.contains("search") 
                    || lower.contains("find") 
                    || lower.contains("report")
                    || lower.contains("research")
                    || lower.contains("analyze")
                    || lower.contains("investigate")
                    || lower.contains("data")
                    || lower.contains("information")
                    || lower.contains("fact")
                    || lower.contains("study")
                    || lower.contains("compare")
                    || lower.contains("summarize")
                    || lower.contains("explain")
                    || lower.contains("what is")
                    || lower.contains("how does")
                    || lower.contains("why")
            }
            ProfessionalAgentType::Manager => {
                // Manager handles everything else or general coordination
                true
            }
        }
    }
}

/// Router pattern: Determine which professional agent should handle a task
pub fn route_professional_task(task_description: &str) -> ProfessionalAgentType {
    // Priority order: Coder > Researcher > Manager
    // This ensures specific agents are chosen over the general Manager
    
    if ProfessionalAgentType::Coder.matches_task(task_description) {
        ProfessionalAgentType::Coder
    } else if ProfessionalAgentType::Researcher.matches_task(task_description) {
        ProfessionalAgentType::Researcher
    } else {
        ProfessionalAgentType::Manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_coder_tasks() {
        assert_eq!(
            route_professional_task("Write a Rust function to parse JSON"),
            ProfessionalAgentType::Coder
        );
        assert_eq!(
            route_professional_task("Debug this code error"),
            ProfessionalAgentType::Coder
        );
        assert_eq!(
            route_professional_task("Optimize the performance"),
            ProfessionalAgentType::Coder
        );
    }

    #[test]
    fn test_route_researcher_tasks() {
        assert_eq!(
            route_professional_task("Search for information about Rust async"),
            ProfessionalAgentType::Researcher
        );
        assert_eq!(
            route_professional_task("Find the latest security vulnerabilities"),
            ProfessionalAgentType::Researcher
        );
        assert_eq!(
            route_professional_task("Generate a report on system performance"),
            ProfessionalAgentType::Researcher
        );
    }

    #[test]
    fn test_route_manager_tasks() {
        assert_eq!(
            route_professional_task("Schedule a task for tomorrow"),
            ProfessionalAgentType::Manager
        );
        assert_eq!(
            route_professional_task("Coordinate the deployment"),
            ProfessionalAgentType::Manager
        );
    }

    #[test]
    fn test_system_prompts_contain_constraints() {
        let manager_prompt = ProfessionalAgentType::Manager.system_prompt("Phoenix");
        let researcher_prompt = ProfessionalAgentType::Researcher.system_prompt("Phoenix");
        let coder_prompt = ProfessionalAgentType::Coder.system_prompt("Phoenix");

        // All prompts should explicitly block personal memory access
        assert!(manager_prompt.contains("NO access to personal memories"));
        assert!(researcher_prompt.contains("NO access to personal memories"));
        assert!(coder_prompt.contains("NO access to personal memories"));

        // All prompts should block Fantasy Dyad
        assert!(manager_prompt.contains("NO Fantasy Dyad"));
        assert!(researcher_prompt.contains("NO Fantasy Dyad"));
        assert!(coder_prompt.contains("NO Fantasy Dyad"));
    }
}
