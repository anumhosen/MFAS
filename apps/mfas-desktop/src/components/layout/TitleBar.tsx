import React, { useState } from 'react';
import { VscChromeMinimize, VscChromeMaximize, VscChromeRestore, VscChromeClose } from 'react-icons/vsc';
import { FiSun, FiMoon } from 'react-icons/fi';
import { useTheme } from '../../context/ThemeContext';
import { tauriService } from '../../services/tauri';

export const TitleBar: React.FC = () => {
  const { theme, toggleTheme } = useTheme();
  const [isMaximized, setIsMaximized] = useState(false);

  const handleMinimize = async () => {
    try {
      await tauriService.minimizeWindow();
    } catch (err) {
      console.error(err);
    }
  };

  const handleToggleMaximize = async () => {
    try {
      const state = await tauriService.toggleMaximizeWindow();
      setIsMaximized(state);
    } catch (err) {
      console.error(err);
    }
  };

  const handleClose = async () => {
    try {
      await tauriService.closeWindow();
    } catch (err) {
      console.error(err);
    }
  };

  const handleDoubleClick = async (e: React.MouseEvent) => {
    // Only maximize/restore if double clicking the draggable region, not interactive buttons
    if ((e.target as HTMLElement).closest('button')) return;
    await handleToggleMaximize();
  };

  return (
    <header
      data-tauri-drag-region
      onDoubleClick={handleDoubleClick}
      className="h-9 w-full flex items-center justify-between px-3 bg-gray-100 dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800 select-none transition-colors duration-150 drag-region"
    >
      {/* App Branding & Draggable Title Area */}
      <div data-tauri-drag-region className="flex items-center gap-2 drag-region pointer-events-auto cursor-default">
        <div data-tauri-drag-region className="w-4 h-4 rounded bg-gray-800 dark:bg-gray-200 flex items-center justify-center text-[10px] font-bold text-gray-100 dark:text-gray-900">
          M
        </div>
        <span data-tauri-drag-region className="text-xs font-semibold text-gray-700 dark:text-gray-300 tracking-wide">
          MFAS Explorer
        </span>
        <span data-tauri-drag-region className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-gray-200 dark:bg-gray-800 text-gray-600 dark:text-gray-400">
          v1.0 (Canonical)
        </span>
      </div>

      {/* Middle Spacer Area (Draggable) */}
      <div data-tauri-drag-region className="flex-1 h-full drag-region" />

      {/* Right Controls: Theme Toggle + Window Controls (Explicit No-Drag Region) */}
      <div
        className="flex items-center no-drag-region -mr-3"
        data-tauri-drag-region="false"
      >
        <button
          onClick={toggleTheme}
          title={theme === 'dark' ? 'Switch to Light Mode' : 'Switch to Dark Mode'}
          className="p-1.5 mr-2 rounded text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100 hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors no-drag-region cursor-pointer"
        >
          {theme === 'dark' ? <FiSun className="w-3.5 h-3.5" /> : <FiMoon className="w-3.5 h-3.5" />}
        </button>

        <div className="flex items-center no-drag-region">
          <button
            onClick={handleMinimize}
            title="Minimize"
            className="w-10 h-9 flex items-center justify-center text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-100 transition-colors no-drag-region cursor-pointer"
          >
            <VscChromeMinimize className="w-3.5 h-3.5" />
          </button>

          <button
            onClick={handleToggleMaximize}
            title={isMaximized ? 'Restore' : 'Maximize'}
            className="w-10 h-9 flex items-center justify-center text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-100 transition-colors no-drag-region cursor-pointer"
          >
            {isMaximized ? (
              <VscChromeRestore className="w-4.5 h-4.5" />
            ) : (
              <VscChromeMaximize className="w-3.5 h-3.5" />
            )}
          </button>

          <button
            onClick={handleClose}
            title="Close"
            className="w-10 h-9 flex items-center justify-center text-gray-600 dark:text-gray-400 hover:bg-red-600 hover:text-white transition-colors no-drag-region cursor-pointer"
          >
            <VscChromeClose className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    </header>
  );
};
