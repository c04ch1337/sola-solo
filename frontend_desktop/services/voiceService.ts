/**
 * Voice Service for Phoenix Backend
 * 
 * Handles voice input (STT) and output (TTS) integration with Phoenix backend.
 * Provides recording, transcription, and text-to-speech capabilities.
 */

import { getPhoenixApiBase } from '../env';

export interface VoiceRecordingResponse {
  status: string;
  session_id?: string;
  error?: string;
}

export interface VoiceStopResponse {
  status: string;
  transcript?: string;
  error?: string;
}

export interface VoiceStatusResponse {
  enabled: boolean;
  listening: boolean;
  recording: boolean;
}

export class VoiceService {
  private recordingSessionId: string | null = null;
  private isRecording: boolean = false;
  private voiceOutputEnabled: boolean = false;

  /**
   * Start recording audio for speech-to-text
   * @param purpose Optional purpose for recording (e.g., "dictation", "live")
   * @returns Session ID if successful
   */
  async startRecording(purpose?: string): Promise<string> {
    if (this.isRecording) {
      throw new Error('Already recording');
    }

    try {
      const apiBase = getPhoenixApiBase();
      const response = await fetch(`${apiBase}/api/audio/start-recording`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ purpose: purpose || 'dictation' }),
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || `HTTP ${response.status}`);
      }

      const data: VoiceRecordingResponse = await response.json();
      
      if (data.error) {
        throw new Error(data.error);
      }

      this.recordingSessionId = data.session_id || null;
      this.isRecording = true;
      
      return this.recordingSessionId || '';
    } catch (error) {
      this.isRecording = false;
      this.recordingSessionId = null;
      throw error;
    }
  }

  /**
   * Stop recording and get transcript
   * @returns Transcript text
   */
  async stopRecording(): Promise<string> {
    if (!this.isRecording) {
      throw new Error('Not recording');
    }

    try {
      const apiBase = getPhoenixApiBase();
      const response = await fetch(`${apiBase}/api/audio/stop-recording`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(error.error || `HTTP ${response.status}`);
      }

      const data: VoiceStopResponse = await response.json();
      
      if (data.error) {
        throw new Error(data.error);
      }

      this.isRecording = false;
      this.recordingSessionId = null;

      return data.transcript || '';
    } catch (error) {
      this.isRecording = false;
      this.recordingSessionId = null;
      throw error;
    }
  }

  /**
   * Get current voice status
   */
  async getStatus(): Promise<VoiceStatusResponse> {
    try {
      const apiBase = getPhoenixApiBase();
      const response = await fetch(`${apiBase}/api/audio/status`, {
        method: 'GET',
      });

      if (!response.ok) {
        return { enabled: false, listening: false, recording: false };
      }

      return await response.json();
    } catch (error) {
      return { enabled: false, listening: false, recording: false };
    }
  }

  /**
   * Speak text using TTS
   * @param text Text to speak
   * @param params Optional voice parameters (pitch, rate, etc.)
   */
  async speak(text: string, params?: { pitch?: number; rate?: number; volume?: number }): Promise<void> {
    if (!this.voiceOutputEnabled) {
      console.log('[Voice] Output disabled, skipping TTS');
      return;
    }

    if (!text || text.trim().length === 0) {
      console.log('[Voice] Empty text, skipping TTS');
      return;
    }

    try {
      const apiBase = getPhoenixApiBase();
      const response = await fetch(`${apiBase}/api/audio/speak`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          text: text.trim(),
          pitch: params?.pitch,
          rate: params?.rate,
          volume: params?.volume,
        }),
      });

      if (!response.ok) {
        if (response.status === 404) {
          console.log('[Voice] TTS endpoint not available yet. Backend needs to add /api/audio/speak');
          return;
        }
        const error = await response.json().catch(() => ({ error: `HTTP ${response.status}` }));
        console.error('[Voice] TTS error:', error);
        throw new Error(error.error || `HTTP ${response.status}`);
      }

      // Get audio content type
      const contentType = response.headers.get('content-type') || 'audio/wav';
      
      // Get audio as blob
      const audioBlob = await response.blob();
      
      // Create audio URL and play
      const audioUrl = URL.createObjectURL(audioBlob);
      const audio = new Audio(audioUrl);
      
      // Clean up URL when done
      audio.addEventListener('ended', () => {
        URL.revokeObjectURL(audioUrl);
      });
      
      audio.addEventListener('error', (e) => {
        console.error('[Voice] Audio playback error:', e);
        URL.revokeObjectURL(audioUrl);
      });

      // Play audio
      await audio.play();
      console.log('[Voice] Playing TTS audio');
    } catch (error: any) {
      // If it's a network error (endpoint doesn't exist), just log
      if (error.message?.includes('Failed to fetch') || error.message?.includes('404')) {
        console.log('[Voice] TTS endpoint not available. Backend needs to add /api/audio/speak');
        return;
      }
      console.error('[Voice] Failed to speak:', error);
      // Don't throw - just log the error so it doesn't break the UI
    }
  }

  /**
   * Enable/disable voice output (TTS)
   */
  setVoiceOutputEnabled(enabled: boolean): void {
    this.voiceOutputEnabled = enabled;
  }

  /**
   * Check if voice output is enabled
   */
  isVoiceOutputEnabled(): boolean {
    return this.voiceOutputEnabled;
  }

  /**
   * Check if currently recording
   */
  getIsRecording(): boolean {
    return this.isRecording;
  }

  /**
   * Get current recording session ID
   */
  getSessionId(): string | null {
    return this.recordingSessionId;
  }
}

export default VoiceService;
