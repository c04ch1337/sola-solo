import React, { useEffect, useMemo, useRef } from 'react';
import { AnimatePresence, motion } from 'framer-motion';
import type { TrustPhase, ZodiacSign } from '../stores/trustStore';
import { computeTrustPhase } from '../stores/trustStore';

type Props = {
  trustScore: number; // 0..1
  predictedGain?: number; // 0..1
  zodiacSign: ZodiacSign;
  className?: string;
  onPhaseUnlocked?: (phase: TrustPhase) => void;
};

const phaseColorClasses: Record<TrustPhase, string> = {
  Acquaintance: 'bg-sky-500',
  Friend: 'bg-emerald-500',
  Intimate: 'bg-violet-700',
  ResearchPartner: 'bg-amber-500',
};

const zodiacIcon: Record<ZodiacSign, string> = {
  aries: '♈',
  taurus: '♉',
  gemini: '♊',
  cancer: '♋',
  leo: '♌',
  virgo: '♍',
  libra: '♎',
  scorpio: '♏',
  sagittarius: '♐',
  capricorn: '♑',
  aquarius: '♒',
  pisces: '♓',
};

const growthStyle: Record<ZodiacSign, string> = {
  aries: 'Aries: Bold and fast-bonding',
  taurus: 'Taurus: Slow, steady, and loyal',
  gemini: 'Gemini: Curious, playful growth',
  cancer: 'Cancer: Protective and nurturing',
  leo: 'Leo: Warm, expressive devotion',
  virgo: 'Virgo: Careful, consistent gains',
  libra: 'Libra: Harmony-first trust building',
  scorpio: 'Scorpio: Guarded but intense',
  sagittarius: 'Sagittarius: Freedom-loving, optimistic',
  capricorn: 'Capricorn: Earned, durable trust',
  aquarius: 'Aquarius: Idea-driven, independent',
  pisces: 'Pisces: Soft, empathic bonding',
};

export default function TrustProgressBar({
  trustScore,
  predictedGain = 0,
  zodiacSign,
  className,
  onPhaseUnlocked,
}: Props) {
  const clampedTrust = Math.min(1, Math.max(0, trustScore));
  const clampedGain = Math.min(1, Math.max(0, predictedGain));

  const phase = useMemo(() => computeTrustPhase(clampedTrust), [clampedTrust]);
  const prevPhaseRef = useRef<TrustPhase>(phase);

  useEffect(() => {
    const prev = prevPhaseRef.current;
    if (prev !== phase) {
      prevPhaseRef.current = phase;
      if (onPhaseUnlocked) onPhaseUnlocked(phase);
    }
  }, [phase, onPhaseUnlocked]);

  const fillPct = clampedTrust * 100;
  const ghostStartPct = fillPct;
  const ghostEndPct = Math.min(100, (clampedTrust + clampedGain) * 100);
  const ghostWidthPct = Math.max(0, ghostEndPct - ghostStartPct);

  return (
    <div className={className}>
      <div className="flex items-center justify-between mb-2">
        <div className="flex items-center gap-2">
          <span
            className="text-lg select-none"
            title={growthStyle[zodiacSign]}
            aria-label={growthStyle[zodiacSign]}
          >
            {zodiacIcon[zodiacSign]}
          </span>
          <div className="flex flex-col">
            <div className="text-xs font-bold text-white tracking-wide">
              Trust Progress
            </div>
            <div className="text-[10px] text-slate-400">
              Phase: <span className="text-slate-200 font-semibold">{phase}</span>
            </div>
          </div>
        </div>

        <div className="text-[10px] font-mono text-slate-400">
          {(clampedTrust * 100).toFixed(0)}%
        </div>
      </div>

      <div className="relative h-3 rounded-full bg-slate-800 border border-border-dark overflow-hidden">
        {/* Main fill */}
        <motion.div
          className={`absolute left-0 top-0 h-full ${phaseColorClasses[phase]}`}
          initial={false}
          animate={{ width: `${fillPct}%` }}
          transition={{ type: 'spring', stiffness: 260, damping: 28 }}
        />

        {/* Ghost segment (predictive trust) */}
        {ghostWidthPct > 0 && (
          <motion.div
            className="absolute top-0 h-full bg-white/20"
            style={{ left: `${ghostStartPct}%` }}
            initial={false}
            animate={{ width: `${ghostWidthPct}%` }}
            transition={{ type: 'tween', duration: 0.25 }}
            title={`Predicted gain: +${(clampedGain * 100).toFixed(0)}%`}
          />
        )}
      </div>

      {/* Unlock animation */}
      <AnimatePresence>
        {phase === 'Intimate' && (
          <motion.div
            key="unlock"
            initial={{ opacity: 0, y: 6 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: 6 }}
            transition={{ duration: 0.25 }}
            className="mt-2 text-[10px] text-violet-200 flex items-center gap-1"
          >
            <motion.span
              initial={{ scale: 0.8 }}
              animate={{ scale: [0.9, 1.15, 1.0] }}
              transition={{ duration: 0.45 }}
            >
              🔓
            </motion.span>
            Intimate threshold reached: photos unlock
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}

