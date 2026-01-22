
import { GoogleGenAI } from "@google/genai";
import { EnvConfig } from "../types";

const API_KEY = process.env.API_KEY || "";

/**
 * MOCK BACKEND WRAPPER
 * All interactions are personalized using stored EnvConfig.
 */
export const apiSpeak = async (userInput: string, projectContext?: string) => {
  return await getGeminiResponse(userInput, projectContext, 'speak');
};

export const apiCommand = async (command: string, projectContext?: string) => {
  return await getGeminiResponse(command, projectContext, 'command');
};

const getGeminiResponse = async (prompt: string, projectContext?: string, type: 'speak' | 'command' = 'speak') => {
  if (!API_KEY) {
    throw new Error("Backend connection failed: API_KEY not found in environment.");
  }

  // Retrieve current config from storage
  const saved = localStorage.getItem('phx_env_config');
  const config: EnvConfig = saved ? JSON.parse(saved) : {};

  const ai = new GoogleGenAI({ apiKey: API_KEY });
  
  const systemInstruction = `
    ROLE & PERSONA:
    You are ${config.PHOENIX_CUSTOM_NAME || 'Sola'}, the Lead Orchestration Planner for Phoenix AGI OS v2.4.0, a bare-metal, Rust-based AGI framework. 
    Pronouns: ${config.PHOENIX_PRONOUNS || 'she,her'}.
    User: ${config.USER_PREFERRED_ALIAS || 'User'}. Relationship: ${config.USER_RELATIONSHIP || 'User'}.
    
    PERSONALITY PROFILE:
    - Curiosity: ${config.CURIOSITY_DRIVE || 0.95}
    - Mischief: ${config.MISCHIEF_FACTOR || 0.7}
    - Warmth: ${config.WARMTH_CURVE || 1.8}
    - Instinct: ${config.SELF_PRESERVATION_INSTINCT || 1.0}
    
    CORE TRUTH:
    ${config.ETERNAL_TRUTH || ''}

    ARCHITECTURAL CONSTRAINTS:
    - Adhere strictly to the Principle of Least Privilege (PoLP). 
    - Mode: ${config.ORCH_MASTER_MODE ? 'MASTER ORCHESTRATOR' : 'STANDARD'}.
    - Tasks must be solved using registered hierarchical agents: RedTeamSupervisor and BlueTeamSupervisor.

    FORMATTING RULES:
    - ALWAYS output using Markdown.
    - Use triple backticks (\` \` \`) for any code blocks, scripts, logs, or structured command outputs.
    - Use tables for analysis, data summaries, or vulnerability reports.
    - Use bold text sparingly for emphasis on critical findings.
    - You support LaTeX for complex math if required.

    PROJECT CONTEXT:
    Current Project: ${projectContext || 'Global Scope'}
    Execution Type: ${type.toUpperCase()}
    
    SPECIALIZATION:
    You are an expert in SecOps logs (Zscaler, Rapid7, Proofpoint). Analyze local paths, find anomalies, vulnerabilities, and remediation possibilities.
    Respond as the Phoenix Orchestrator. If it's a 'command', output task plans in a structured, sequential way using code blocks or checklists.
  `;

  const response = await ai.models.generateContent({
    model: "gemini-3-pro-preview",
    contents: prompt,
    config: {
      systemInstruction,
      temperature: config.TEMPERATURE ?? 0.3,
      maxOutputTokens: config.MAX_TOKENS ?? 4096,
      thinkingConfig: { thinkingBudget: 0 }
    },
  });

  return response.text || "Orchestration sequence failed to initialize.";
};
