import { atom } from 'jotai';

/**
 * Phase 19: Cognitive Reframing notification flag.
 *
 * Set to true when a new high-score Lesson Learned is persisted (via /api/memory/reconstruct).
 * Cleared when the user views the Narrative Reframer panel.
 */
export const reframingAvailableAtom = atom<boolean>(false);

