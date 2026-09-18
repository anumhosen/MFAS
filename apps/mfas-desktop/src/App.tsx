import React, { useState } from 'react';
import { ThemeProvider } from './context/ThemeContext';
import { TitleBar } from './components/layout/TitleBar';
import { Sidebar } from './components/layout/Sidebar';
import { StatusBar } from './components/layout/StatusBar';
import { AddressExplorerView } from './components/views/AddressExplorerView';
import { PageExplorerView } from './components/views/PageExplorerView';
import { DagVisualizerView } from './components/views/DagVisualizerView';
import { CodecStudioView } from './components/views/CodecStudioView';
import { BenchmarkGpuView } from './components/views/BenchmarkGpuView';
import type { ViewMode } from './types';

export const App: React.FC = () => {
  const [currentView, setCurrentView] = useState<ViewMode>('address');
  const [statusMessage, setStatusMessage] = useState('MFAS Desktop Explorer Ready');

  return (
    <ThemeProvider>
      <div className="flex flex-col h-screen w-screen overflow-hidden bg-gray-50 dark:bg-gray-950 text-gray-900 dark:text-gray-100">
        {/* Custom Frameless Titlebar */}
        <TitleBar />

        {/* Main Content Body */}
        <div className="flex flex-1 overflow-hidden">
          {/* Left Navigation Sidebar */}
          <Sidebar currentView={currentView} onViewChange={setCurrentView} />

          {/* Active Explorer View */}
          <main className="flex-1 flex flex-col overflow-hidden">
            {currentView === 'address' && (
              <AddressExplorerView onStatusChange={setStatusMessage} />
            )}
            {currentView === 'page' && (
              <PageExplorerView onStatusChange={setStatusMessage} />
            )}
            {currentView === 'dag' && (
              <DagVisualizerView onStatusChange={setStatusMessage} />
            )}
            {currentView === 'codec' && (
              <CodecStudioView onStatusChange={setStatusMessage} />
            )}
            {currentView === 'gpu' && (
              <BenchmarkGpuView onStatusChange={setStatusMessage} />
            )}
          </main>
        </div>

        {/* Bottom Status Bar */}
        <StatusBar statusMessage={statusMessage} />
      </div>
    </ThemeProvider>
  );
};

export default App;
