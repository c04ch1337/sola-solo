import React from 'react';

interface OnboardingMessageProps {
  phoenixName: string;
  onDismiss: () => void;
}

const OnboardingMessage: React.FC<OnboardingMessageProps> = ({ phoenixName, onDismiss }) => {
  return (
    <div className="mb-6 p-6 rounded-2xl bg-linear-to-r from-primary/20 via-primary/10 to-transparent border border-primary/30 shadow-lg">
      <div className="flex items-start gap-4">
        <div className="size-12 rounded-xl bg-primary/20 flex items-center justify-center shrink-0 border border-primary/30">
          <span className="material-symbols-outlined text-2xl text-primary">sparkles</span>
        </div>
        <div className="flex-1">
          <h3 className="text-lg font-bold text-white mb-2">
            Welcome to {phoenixName}! 🕊️
          </h3>
          <p className="text-sm text-slate-300 mb-4 leading-relaxed">
            I'm your AI assistant, ready to help with anything you need. Type <code className="bg-black/40 px-1.5 py-0.5 rounded text-primary font-mono text-xs">help</code> to see available commands, or just start chatting!
          </p>
          <div className="flex flex-wrap gap-2 mb-4">
            <span className="text-xs px-2 py-1 bg-primary/10 text-primary rounded border border-primary/20">
              💬 Chat
            </span>
            <span className="text-xs px-2 py-1 bg-primary/10 text-primary rounded border border-primary/20">
              🎤 Voice
            </span>
            <span className="text-xs px-2 py-1 bg-primary/10 text-primary rounded border border-primary/20">
              🌐 Browser
            </span>
            <span className="text-xs px-2 py-1 bg-primary/10 text-primary rounded border border-primary/20">
              💭 Dreams
            </span>
          </div>
          <button
            onClick={onDismiss}
            className="px-4 py-2 bg-primary/20 hover:bg-primary/30 text-primary rounded-lg text-sm font-medium transition-colors border border-primary/30"
          >
            Got it!
          </button>
        </div>
        <button
          onClick={onDismiss}
          className="p-1 hover:bg-white/10 rounded-lg transition-colors text-slate-400 hover:text-white shrink-0"
        >
          <span className="material-symbols-outlined text-[20px]">close</span>
        </button>
      </div>
    </div>
  );
};

export default OnboardingMessage;
