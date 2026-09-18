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

  return (
    <header
      data-tauri-drag-region
      className="h-9 w-full flex items-center justify-between px-3 bg-gray-100 dark:bg-gray-900 border-b border-gray-200 dark:border-gray-800 select-none transition-colors duration-150"
    >
      {/* App Branding */}
      <div data-tauri-drag-region className="flex items-center gap-2 pointer-events-none">
        <div className="w-4 h-4 rounded bg-gray-800 dark:bg-gray-200 flex items-center justify-center text-[10px] font-bold text-gray-100 dark:text-gray-900">
          M
        </div>
        <span className="text-xs font-semibold text-gray-700 dark:text-gray-300 tracking-wide">
          MFAS Explorer
        </span>
        <span className="text-[10px] font-mono px-1.5 py-0.5 rounded bg-gray-200 dark:bg-gray-800 text-gray-600 dark:text-gray-400">
          v1.0 (Canonical)
        </span>
      </div>

      {/* Right Controls: Theme Toggle + Window Controls */}
      <div className="flex items-center">
        <button
          onClick={toggleTheme}
          title={theme === 'dark' ? 'Switch to Light Mode' : 'Switch to Dark Mode'}
          className="p-1.5 mr-2 rounded text-gray-500 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-100 hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors"
        >
          {theme === 'dark' ? <FiSun className="w-3.5 h-3.5" /> : <FiMoon className="w-3.5 h-3.5" />}
        </button>

        <div className="flex items-center">
          <button
            onClick={handleMinimize}
            title="Minimize"
            className="w-10 h-9 flex items-center justify-center text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-100 transition-colors"
          >
            <VscChromeMinimize className="w-3.5 h-3.5" />
          </button>

          <button
            onClick={handleToggleMaximize}
            title={isMaximized ? 'Restore' : 'Maximize'}
            className="w-10 h-9 flex items-center justify-center text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-gray-100 transition-colors"
          >
            {isMaximized ? (
              <VscChromeRestore className="w-3.5 h-3.5" />
            ) : (
              <VscChromeMaximize className="w-3.5 h-3.5" />
            )}
          </button>

          <button
            onClick={handleClose}
            title="Close"
            className="w-10 h-9 flex items-center justify-center text-gray-600 dark:text-gray-400 hover:bg-red-600 hover:text-white transition-colors"
          >
            <VscChromeClose className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>
    </header>
  );
};
