export function Aura({
  isThinking,
  isEvolving,
  isRollingBack,
  rollbackFailed,
}: {
  isThinking: boolean
  isEvolving: boolean
  isRollingBack: boolean
  rollbackFailed: boolean
}) {
  const glow = rollbackFailed
    ? 'bg-red-500/18'
    : isRollingBack
      ? 'bg-orange-400/14'
      : isEvolving
        ? 'bg-purple-400/12'
        : isThinking
          ? 'bg-emerald-400/10'
          : 'bg-emerald-400/10'

  const core = rollbackFailed
    ? 'bg-red-400/25'
    : isRollingBack
      ? 'bg-orange-300/25'
      : isEvolving
        ? 'bg-amber-300/25'
        : 'bg-indigo-400/30'

  return (
    <div className="relative flex h-64 w-64 items-center justify-center">
      {/* Outer glow */}
      <div
        className={
          'absolute inset-0 rounded-full blur-3xl transition-all duration-700 ' +
          glow +
          ' ' +
          (isThinking || isEvolving || isRollingBack || rollbackFailed
            ? 'scale-110 opacity-70'
            : 'scale-100 opacity-35')
        }
      />

      {/* Ring */}
      <div
        className={
          'absolute inset-6 rounded-full border border-white/12 bg-white/5 backdrop-blur ' +
          (isThinking || isEvolving || isRollingBack || rollbackFailed ? 'animate-pulse' : '')
        }
      />

      {/* Core */}
      <div className="relative">
        <div
          className={
            'h-16 w-16 rounded-full blur-xl transition-opacity duration-300 ' +
            core +
            ' ' +
            (isThinking || isEvolving || isRollingBack || rollbackFailed ? 'opacity-90' : 'opacity-50')
          }
        />
        <div className="absolute inset-0 h-16 w-16 rounded-full border border-white/15 bg-white/10" />
      </div>

      {/* Subtle orbit */}
      <div
        className={
          'absolute h-64 w-64 rounded-full border border-white/5 ' +
          (isThinking || isEvolving || isRollingBack || rollbackFailed
            ? 'animate-[spin_6s_linear_infinite]'
            : '')
        }
      />
    </div>
  )
}
