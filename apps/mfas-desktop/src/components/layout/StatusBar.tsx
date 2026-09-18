import React from 'react';
import { FiCheckCircle, FiCpu, FiHardDrive } from 'react-icons/fi';

interface StatusBarProps {
  statusMessage: string;
}

export const StatusBar: React.FC<StatusBarProps> = ({ statusMessage }) => {
  return (
    <footer className="h-6 px-3 bg-gray-100 dark:bg-gray-900 border-t border-gray-200 dark:border-gray-800 flex items-center justify-between text-[11px] text-gray-500 dark:text-gray-400 select-none transition-colors duration-150 shrink-0">
      <div className="flex items-center gap-2">
        <FiCheckCircle className="w-3 h-3 text-emerald-500" />
        <span className="truncate max-w-md">{statusMessage || 'Ready'}</span>
      </div>

      <div className="flex items-center gap-4">
        <div className="flex items-center gap-1">
          <FiHardDrive className="w-3 h-3" />
          <span>Store: <code>.mfas/objects</code></span>
        </div>
        <div className="flex items-center gap-1">
          <FiCpu className="w-3 h-3" />
          <span>Backend: Evaluator (CPU/SIMD/GPU)</span>
        </div>
        <div>
          <span>Radix: <code>16³ = 4096B</code></span>
        </div>
      </div>
    </footer>
  );
};
