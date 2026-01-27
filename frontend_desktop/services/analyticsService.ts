/**
 * Analytics Service for Sola AGI
 * 
 * Simple opt-in usage tracking to improve Sola.
 * All data is anonymous and aggregated.
 */

import { getPhoenixApiBase } from '../env';

interface AnalyticsEvent {
  event: string;
  data?: Record<string, any>;
  timestamp: number;
}

class AnalyticsService {
  private enabled: boolean = false;
  private eventQueue: AnalyticsEvent[] = [];
  private sessionId: string;

  constructor() {
    // Check if analytics is enabled (opt-in)
    this.enabled = localStorage.getItem('phx_analytics_enabled') === 'true';
    this.sessionId = this.getOrCreateSessionId();
    
    // Load queued events if any
    this.loadQueue();
  }

  private getOrCreateSessionId(): string {
    let sessionId = localStorage.getItem('phx_session_id');
    if (!sessionId) {
      sessionId = `session_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
      localStorage.setItem('phx_session_id', sessionId);
    }
    return sessionId;
  }

  private loadQueue(): void {
    const queue = localStorage.getItem('phx_analytics_queue');
    if (queue) {
      try {
        this.eventQueue = JSON.parse(queue);
      } catch (e) {
        this.eventQueue = [];
      }
    }
  }

  private saveQueue(): void {
    localStorage.setItem('phx_analytics_queue', JSON.stringify(this.eventQueue));
  }

  /**
   * Enable analytics (opt-in)
   */
  enable(): void {
    this.enabled = true;
    localStorage.setItem('phx_analytics_enabled', 'true');
    // Send queued events
    this.flushQueue();
  }

  /**
   * Disable analytics
   */
  disable(): void {
    this.enabled = false;
    localStorage.setItem('phx_analytics_enabled', 'false');
    this.eventQueue = [];
    this.saveQueue();
  }

  /**
   * Check if analytics is enabled
   */
  isEnabled(): boolean {
    return this.enabled;
  }

  /**
   * Track an event
   */
  track(event: string, data?: Record<string, any>): void {
    if (!this.enabled) return;

    const eventData: AnalyticsEvent = {
      event,
      data,
      timestamp: Date.now(),
    };

    this.eventQueue.push(eventData);
    this.saveQueue();

    // Send immediately (non-blocking)
    this.sendEvent(eventData).catch(() => {
      // If send fails, event is already in queue and will be sent later
    });
  }

  /**
   * Send event to backend
   */
  private async sendEvent(event: AnalyticsEvent): Promise<void> {
    try {
      const apiBase = getPhoenixApiBase();
      await fetch(`${apiBase}/api/analytics/track`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          session_id: this.sessionId,
          ...event,
        }),
      });
    } catch (error) {
      // Silently fail - events are queued and will be sent later
      console.debug('[Analytics] Failed to send event:', error);
    }
  }

  /**
   * Flush queued events
   */
  private async flushQueue(): Promise<void> {
    if (this.eventQueue.length === 0) return;

    const events = [...this.eventQueue];
    this.eventQueue = [];
    this.saveQueue();

    for (const event of events) {
      try {
        await this.sendEvent(event);
      } catch (error) {
        // Re-queue failed events
        this.eventQueue.push(event);
      }
    }
    this.saveQueue();
  }

  /**
   * Track common events
   */
  trackMessageSent(): void {
    this.track('message_sent');
  }

  trackVoiceEnabled(): void {
    this.track('voice_enabled');
  }

  trackFeatureUsed(feature: string): void {
    this.track('feature_used', { feature });
  }

  trackCommandUsed(command: string): void {
    this.track('command_used', { command });
  }
}

export const analyticsService = new AnalyticsService();
export default analyticsService;
