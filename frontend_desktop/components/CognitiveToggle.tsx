import React from 'react';
import { useAtom } from 'jotai';
import { modeAtom, toggleModeAtom } from '../stores/modeStore';
import { motion } from 'framer-motion';

interface CognitiveToggleProps {
  className?: string;
}

const CognitiveToggle: React.FC<CognitiveToggleProps> = ({ className }) => {
  const [mode] = useAtom(modeAtom);
  const [, toggleMode] = useAtom(toggleModeAtom);

  return (
    <div className={`flex items-center gap-3 ${className}`}>
      <span className={`text-sm font-medium transition-colors duration-300 ${
        mode === 'Professional' ? 'text-slate-700' : 'text-red-900/60'
      }`}>
        Professional
      </span>
      
      <button
        onClick={() => toggleMode()}
        className={`relative h-8 w-16 rounded-full p-1 transition-colors duration-500 focus:outline-none focus:ring-2 focus:ring-offset-2 ${
          mode === 'Professional' 
            ? 'bg-slate-200 focus:ring-indigo-500' 
            : 'bg-red-900 focus:ring-red-500'
        }`}
        aria-label={`Switch to ${mode === 'Professional' ? 'Personal' : 'Professional'} mode`}
      >
        <motion.div
          className="h-6 w-6 rounded-full shadow-md"
          layout
          transition={{
            type: "spring",
            stiffness: 700,
            damping: 30
          }}
          animate={{
            x: mode === 'Professional' ? 0 : 32,
            backgroundColor: mode === 'Professional' ? '#ffffff' : '#fbbf24' // White to Gold
          }}
        />
      </button>

      <span className={`text-sm font-medium transition-colors duration-300 ${
        mode === 'Personal' ? 'text-red-900' : 'text-slate-400'
      }`}>
        Personal
      </span>
    </div>
  );
};

export default CognitiveToggle;