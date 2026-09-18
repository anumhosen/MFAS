import React from 'react';
import { FiHash, FiLayers, FiGitBranch, FiHardDrive, FiZap } from 'react-icons/fi';
import type { ViewMode } from '../../types';

interface SidebarProps {
  currentView: ViewMode;
  onViewChange: (view: ViewMode) => void;
}

interface NavItem {
  id: ViewMode;
  label: string;
  icon: React.ComponentType<{ className?: string }>;
  description: string;
}

const navItems: NavItem[] = [
  {
    id: 'address',
    label: 'Address Space',
    icon: FiHash,
    description: 'Decomposition, LEB128 & Bijection',
  },
  {
    id: 'page',
    label: 'Radix Pages',
    icon: FiLayers,
    description: 'Spatial 4096B Hierarchy',
  },
  {
    id: 'dag',
    label: 'DAG Engine',
    icon: FiGitBranch,
    description: 'Graphviz & CSE Optimization',
  },
  {
    id: 'codec',
    label: 'Codec Studio',
    icon: FiHardDrive,
    description: 'Streaming & SHA-256 Verifier',
  },
  {
    id: 'gpu',
    label: 'Hardware & GPU',
    icon: FiZap,
    description: 'SIMD/GPU Benchmarks',
  },
];

export const Sidebar: React.FC<SidebarProps> = ({ currentView, onViewChange }) => {
  return (
    <aside className="w-60 h-full flex flex-col bg-gray-100 dark:bg-gray-900 border-r border-gray-200 dark:border-gray-800 shrink-0 transition-colors duration-150 select-none">
      <div className="p-3 border-b border-gray-200 dark:border-gray-800">
        <div className="text-[11px] font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">
          Mathematical Explorer
        </div>
      </div>

      <nav className="flex-1 p-2 space-y-1">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = currentView === item.id;

          return (
            <button
              key={item.id}
              onClick={() => onViewChange(item.id)}
              className={`w-full flex items-start gap-3 px-3 py-2.5 rounded text-left transition-all duration-150 ${
                isActive
                  ? 'bg-gray-200 dark:bg-gray-800 text-gray-900 dark:text-gray-100 shadow-sm'
                  : 'text-gray-600 dark:text-gray-400 hover:bg-gray-200/60 dark:hover:bg-gray-800/60 hover:text-gray-900 dark:hover:text-gray-200'
              }`}
            >
              <Icon className={`w-4 h-4 mt-0.5 shrink-0 ${isActive ? 'text-gray-900 dark:text-gray-100' : 'text-gray-500'}`} />
              <div>
                <div className="text-xs font-medium leading-none">{item.label}</div>
                <div className="text-[10px] text-gray-500 dark:text-gray-400 mt-1 leading-tight">
                  {item.description}
                </div>
              </div>
            </button>
          );
        })}
      </nav>

      <div className="p-3 border-t border-gray-200 dark:border-gray-800 bg-gray-50/50 dark:bg-gray-950/40">
        <div className="text-[10px] text-gray-500 dark:text-gray-400">
          <span className="font-semibold text-gray-700 dark:text-gray-300">Invariant:</span>{' '}
          <code>decode(encode(X)) == X</code>
        </div>
      </div>
    </aside>
  );
};
