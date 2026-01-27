/**
 * Phoenix Backend Service
 *
 * Replaces Gemini API with Phoenix Orchestrator backend API calls.
 * All interactions go through the Phoenix backend.
 */

import { getPhoenixApiBase } from '../env';

export interface SpeakRequest {
  user_input: string;
  dad_emotion_hint?: string;
  mode?: string;
}

export interface CommandRequest {
  command: string;
}

export interface PhoenixResponse {
  type: string;
  message?: string;
  [key: string]: any;
}

// WebGuard response types for rich report handling
export interface WebGuardCommandResult {
  message: string;
  isWebGuardReport: boolean;
  reportType?: 'passive' | 'xss' | 'sqli' | 'redirect' | 'cmdinj';
  report?: any;
}

/**
 * Send a chat message to Phoenix via /api/speak
 */
export const apiSpeak = async (userInput: string, projectContext?: string): Promise<string> => {
  try {
    const apiBase = getPhoenixApiBase();
    const request: SpeakRequest = {
      user_input: userInput,
    };

    // Add project context as mode if provided
    if (projectContext) {
      request.mode = projectContext;
    }

    const response = await fetch(`${apiBase}/api/speak`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Phoenix API error (${response.status}): ${errorText}`);
    }

    // Phoenix returns JSON string, parse it
    const text = await response.text();
    let parsed: PhoenixResponse;
    
    try {
      parsed = JSON.parse(text);
    } catch {
      // If not JSON, treat as plain text
      return text;
    }

    // Extract message from response
    if (parsed.type === 'chat.reply' && parsed.message) {
      return parsed.message;
    } else if (parsed.type === 'error') {
      throw new Error(parsed.message || 'Unknown error from Phoenix');
    } else if (parsed.message) {
      return parsed.message;
    }

    // Fallback: return stringified response
    return JSON.stringify(parsed, null, 2);
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(`Phoenix backend connection failed: ${error.message}`);
    }
    throw new Error('Phoenix backend connection failed: Unknown error');
  }
};

/**
 * Send a command to Phoenix via /api/command
 */
export const apiCommand = async (command: string, projectContext?: string): Promise<string> => {
  try {
    const trimmed = command.trim();
    const lower = trimmed.toLowerCase();

    // Only prepend [context=...] for LLM-routed commands.
    // Fast-path commands (system/code/exec/etc.) must remain prefix-free so they
    // don't accidentally fall through to the LLM (which may be offline).
    const isFastPath =
      lower.startsWith('system ') ||
      lower.startsWith('code ') ||
      lower.startsWith('exec ') ||
      lower.startsWith('execute ') ||
      lower.startsWith('skills ') ||
      lower.startsWith('google ') ||
      lower.startsWith('ecosystem ') ||
      lower.startsWith('webguard ');

    const fullCommand = projectContext && !isFastPath
      ? `[context=${projectContext}] ${trimmed}`
      : trimmed;

    const request: CommandRequest = {
      command: fullCommand,
    };

    const apiBase = getPhoenixApiBase();
    const response = await fetch(`${apiBase}/api/command`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Phoenix API error (${response.status}): ${errorText}`);
    }

    // Phoenix returns JSON string, parse it
    const text = await response.text();
    let parsed: PhoenixResponse;
    
    try {
      parsed = JSON.parse(text);
    } catch {
      // If not JSON, treat as plain text
      return text;
    }

    // Extract message from response
    if (parsed.type === 'error') {
      throw new Error(parsed.message || 'Unknown error from Phoenix');
    }

    // Special-case: browser screenshots should render inline.
    if (parsed.type === 'system.browser.screenshot' && typeof parsed.base64 === 'string') {
      const format = (parsed.format || 'jpeg').toLowerCase();
      const mime = format === 'png' ? 'image/png' : 'image/jpeg';
      return `Screenshot captured.\n\n![browser screenshot](data:${mime};base64,${parsed.base64})`;
    } else if (parsed.message) {
      return parsed.message;
    }

    // For command responses, format the entire response
    return JSON.stringify(parsed, null, 2);
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(`Phoenix backend connection failed: ${error.message}`);
    }
    throw new Error('Phoenix backend connection failed: Unknown error');
  }
};

/**
 * Send a WebGuard command and return both message and report data
 */
export const apiWebGuardCommand = async (command: string): Promise<WebGuardCommandResult> => {
  try {
    const apiBase = getPhoenixApiBase();
    const request: CommandRequest = {
      command: command,
    };

    const response = await fetch(`${apiBase}/api/command`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(request),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`Phoenix API error (${response.status}): ${errorText}`);
    }

    const text = await response.text();
    let parsed: PhoenixResponse;
    
    try {
      parsed = JSON.parse(text);
    } catch {
      return { message: text, isWebGuardReport: false };
    }

    if (parsed.type === 'error') {
      throw new Error(parsed.message || 'Unknown error from Phoenix');
    }

    // Check if this is a WebGuard response
    const isWebGuard = parsed.type?.startsWith('webguard.');
    
    if (isWebGuard && parsed.report) {
      let reportType: 'passive' | 'xss' | 'sqli' | 'redirect' | 'cmdinj' = 'passive';
      if (parsed.type.includes('sqli')) {
        reportType = 'sqli';
      } else if (parsed.type.includes('xss')) {
        reportType = 'xss';
      } else if (parsed.type.includes('redirect')) {
        reportType = 'redirect';
      } else if (parsed.type.includes('cmdinj') || parsed.type.includes('cmd')) {
        reportType = 'cmdinj';
      }
      return {
        message: parsed.message || JSON.stringify(parsed, null, 2),
        isWebGuardReport: true,
        reportType,
        report: parsed.report
      };
    }

    return {
      message: parsed.message || JSON.stringify(parsed, null, 2),
      isWebGuardReport: false
    };
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(`Phoenix backend connection failed: ${error.message}`);
    }
    throw new Error('Phoenix backend connection failed: Unknown error');
  }
};

/**
 * Check Phoenix backend health
 */
export const checkPhoenixHealth = async (): Promise<boolean> => {
  try {
    const apiBase = getPhoenixApiBase();
    const response = await fetch(`${apiBase}/health`, {
      method: 'GET',
    });
    return response.ok;
  } catch {
    return false;
  }
};

/**
 * Get Phoenix status
 */
export const getPhoenixStatus = async (): Promise<any> => {
  try {
    const apiBase = getPhoenixApiBase();
    const response = await fetch(`${apiBase}/api/status`, {
      method: 'GET',
    });

    if (!response.ok) {
      throw new Error(`Status check failed: ${response.status}`);
    }

    return await response.json();
  } catch (error) {
    if (error instanceof Error) {
      throw new Error(`Failed to get Phoenix status: ${error.message}`);
    }
    throw new Error('Failed to get Phoenix status: Unknown error');
  }
};
