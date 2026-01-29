
import React from 'react';
import { WorkflowStep, StepStatus } from '../types';

interface WorkflowBlockProps {
  steps: WorkflowStep[];
}

const WorkflowBlock: React.FC<WorkflowBlockProps> = ({ steps }) => {
  return (
    <div className="bg-slate-50 dark:bg-panel-dark/30 rounded-xl border border-border-dark p-4 space-y-3 my-4 select-text">
      {steps.map((step, idx) => (
        <div key={idx} className="flex items-center gap-4">
          <div className={`flex items-center gap-2 ${step.colorClass} px-2 py-1 rounded font-mono text-[11px] font-medium tracking-tight uppercase`}>
            <span className="material-symbols-outlined text-[14px]">{step.icon}</span>
            [{step.label}]
          </div>
          
          <div className="flex-1 flex items-center gap-3">
             <span className="text-xs font-mono text-slate-400">
               {step.text} 
               {step.highlightText && <span className="text-slate-200 ml-1">{step.highlightText}</span>}
             </span>
             
             {step.label === StepStatus.BUILD && step.progress !== undefined && (
               <div className="flex items-center gap-3 flex-1 max-w-[200px]">
                 <div className="flex-1 h-1 bg-slate-200 dark:bg-slate-800 rounded-full overflow-hidden">
                    <div 
                      className="h-full bg-amber-500 transition-all duration-500" 
                      style={{ width: `${step.progress}%` }}
                    />
                 </div>
                 <span className="text-[10px] font-mono text-slate-500 italic shrink-0">Compiling binaries...</span>
               </div>
             )}
          </div>
        </div>
      ))}
    </div>
  );
};

export default WorkflowBlock;
