import React, { useState, useEffect } from 'react';
import { FiCompass } from 'react-icons/fi';
import { tauriService } from '../../services/tauri';
import type { RadixCoordinatesDto } from '../../types';

interface Props {
  onStatusChange: (msg: string) => void;
}

export const PageExplorerView: React.FC<Props> = ({ onStatusChange }) => {
  const [inputValue, setInputValue] = useState('65536');
  const [isPageIndex, setIsPageIndex] = useState(false);
  const [coords, setCoords] = useState<RadixCoordinatesDto | null>(null);
  const [error, setError] = useState<string | null>(null);

  const calculateRadix = async (valStr: string, isPage: boolean) => {
    const val = parseInt(valStr.trim(), 10);
    if (isNaN(val) || val < 0) {
      setError('Please enter a valid positive integer');
      setCoords(null);
      return;
    }

    try {
      setError(null);
      const res = await tauriService.getRadixCoordinates(val, isPage);
      setCoords(res);
      onStatusChange(`Radix coordinate calculated: Page ${res.page_index}, Offset ${res.byte_offset}`);
    } catch (err: any) {
      setError(err?.toString() || 'Calculation failed');
      setCoords(null);
    }
  };

  useEffect(() => {
    calculateRadix(inputValue, isPageIndex);
  }, [inputValue, isPageIndex]);

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors">
      <div>
        <h1 className="text-xl font-bold tracking-tight">Radix-16 Pages & Coordinates</h1>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Spatial hierarchy based on base-16 addresses and 4096-byte ($16^3$) canonical logical pages.
        </p>
      </div>

      {/* Input controls */}
      <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/50 space-y-3">
        <div className="flex justify-between items-center">
          <label className="text-xs font-semibold text-gray-700 dark:text-gray-300">
            Target Coordinate Offset:
          </label>
          <div className="flex items-center gap-2 text-xs">
            <button
              onClick={() => setIsPageIndex(false)}
              className={`px-2.5 py-1 rounded transition-colors ${
                !isPageIndex
                  ? 'bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 font-medium'
                  : 'text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800'
              }`}
            >
              Byte Offset
            </button>
            <button
              onClick={() => setIsPageIndex(true)}
              className={`px-2.5 py-1 rounded transition-colors ${
                isPageIndex
                  ? 'bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 font-medium'
                  : 'text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800'
              }`}
            >
              Page Index (×4096)
            </button>
          </div>
        </div>

        <div className="flex gap-2">
          <input
            type="number"
            min="0"
            value={inputValue}
            onChange={(e) => setInputValue(e.target.value)}
            className="flex-1 px-3 py-2 text-xs font-mono rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 focus:outline-none"
          />
        </div>

        {/* Boundary Quick Jumps */}
        <div className="flex flex-wrap gap-2 pt-1 text-[11px] text-gray-500 items-center">
          <span>Boundary Jumps:</span>
          {[
            { label: '0 (Origin)', val: '0' },
            { label: '4096 (Page 1)', val: '4096' },
            { label: '65536 (1 Volume = 16 Pages)', val: '65536' },
            { label: '1 MiB (1 Shelf)', val: '1048576' },
            { label: '16 MiB (1 Wall)', val: '16777216' },
            { label: '256 MiB (1 Room)', val: '268435456' },
            { label: '4 GiB (1 Floor)', val: '4294967296' },
          ].map((item) => (
            <button
              key={item.label}
              onClick={() => {
                setIsPageIndex(false);
                setInputValue(item.val);
              }}
              className="px-2 py-0.5 rounded border border-gray-300 dark:border-gray-700 hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors font-mono"
            >
              {item.label}
            </button>
          ))}
        </div>
      </div>

      {error && (
        <div className="p-3 text-xs rounded border border-red-300 dark:border-red-900 bg-red-50 dark:bg-red-950/40 text-red-700 dark:text-red-400">
          {error}
        </div>
      )}

      {/* Coordinate Hierarchy Cards */}
      {coords && (
        <div className="space-y-4">
          <div className="p-3.5 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40">
            <div className="text-[10px] uppercase font-semibold text-gray-500 mb-1 flex items-center gap-1.5">
              <FiCompass className="w-3.5 h-3.5" /> Canonical Coordinate String
            </div>
            <code className="text-xs font-mono font-semibold text-gray-800 dark:text-gray-200 break-all">
              {coords.formatted}
            </code>
          </div>

          <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
            {[
              { label: 'Floor (4 GiB)', val: coords.floor, hex: coords.floor.toString(16).toUpperCase() },
              { label: 'Room (256 MiB)', val: coords.room, hex: coords.room.toString(16).toUpperCase() },
              { label: 'Wall (16 MiB)', val: coords.wall, hex: coords.wall.toString(16).toUpperCase() },
              { label: 'Shelf (1 MiB)', val: coords.shelf, hex: coords.shelf.toString(16).toUpperCase() },
              { label: 'Volume (64 KiB)', val: coords.volume, hex: coords.volume.toString(16).toUpperCase() },
              { label: 'Page (4 KiB)', val: coords.page, hex: coords.page.toString(16).toUpperCase() },
            ].map((level) => (
              <div
                key={level.label}
                className="p-3 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/50 text-center"
              >
                <div className="text-[10px] text-gray-500 truncate">{level.label}</div>
                <div className="text-lg font-bold font-mono text-gray-800 dark:text-gray-100 mt-1">
                  0x{level.hex}
                </div>
                <div className="text-[10px] text-gray-400 font-mono mt-0.5">
                  ({level.val})
                </div>
              </div>
            ))}
          </div>

          {/* Detailed Offset Card */}
          <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 grid grid-cols-1 md:grid-cols-3 gap-3 text-xs">
            <div className="p-2.5 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">Absolute Byte Offset:</span>
              <span className="font-mono font-semibold text-gray-800 dark:text-gray-200">
                {coords.byte_offset.toLocaleString()} bytes
              </span>
            </div>
            <div className="p-2.5 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">Logical Page Index:</span>
              <span className="font-mono font-semibold text-gray-800 dark:text-gray-200">
                Page #{coords.page_index.toLocaleString()}
              </span>
            </div>
            <div className="p-2.5 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">Intra-Page Offset ($16^3$):</span>
              <span className="font-mono font-semibold text-gray-800 dark:text-gray-200">
                0x{coords.offset_in_page.toString(16).toUpperCase().padStart(3, '0')} ({coords.offset_in_page} / 4096)
              </span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
