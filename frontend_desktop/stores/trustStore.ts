import { atom } from 'jotai';

export type TrustPhase = 'Acquaintance' | 'Friend' | 'Intimate' | 'ResearchPartner';

export type ZodiacSign =
  | 'aries'
  | 'taurus'
  | 'gemini'
  | 'cancer'
  | 'leo'
  | 'virgo'
  | 'libra'
  | 'scorpio'
  | 'sagittarius'
  | 'capricorn'
  | 'aquarius'
  | 'pisces';

export const TRUST_PHASE_THRESHOLDS: Record<TrustPhase, number> = {
  Acquaintance: 0.2,
  Friend: 0.5,
  Intimate: 0.8,
  ResearchPartner: 0.95,
};

export function computeTrustPhase(trustScore: number): TrustPhase {
  if (trustScore >= TRUST_PHASE_THRESHOLDS.ResearchPartner) return 'ResearchPartner';
  if (trustScore >= TRUST_PHASE_THRESHOLDS.Intimate) return 'Intimate';
  if (trustScore >= TRUST_PHASE_THRESHOLDS.Friend) return 'Friend';
  return 'Acquaintance';
}

// Current trust score for the active persona (0..1).
// TODO: wire to backend L7 memory/trust score.
export const trustScoreAtom = atom<number>(0.12);

// Predictive trust gain (ghost bar segment), 0..1.
// Represents the next planned interaction's likely delta.
export const predictedTrustGainAtom = atom<number>(0.08);

export const zodiacSignAtom = atom<ZodiacSign>('scorpio');

