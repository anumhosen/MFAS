import React, { useState, useEffect } from 'react';
import { FiSearch, FiCode, FiArrowRight, FiCheck } from 'react-icons/fi';
import { tauriService } from '../../services/tauri';
import type { AddressDetailsDto } from '../../types';

interface Props {
  onStatusChange: (msg: string) => void;
}

export const AddressExplorerView: React.FC<Props> = ({ onStatusChange }) => {
  const [inputAddress, setInputAddress] = useState('010048656c6c6f');
  const [details, setDetails] = useState<AddressDetailsDto | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);

  // Bijection converter state
  const [numberInput, setNumberInput] = useState('1751477356');
  const [bijectionAddress, setBijectionAddress] = useState('');
  const [bijectionError, setBijectionError] = useState<string | null>(null);

  const handleParse = async (addrStr: string) => {
    if (!addrStr.trim()) {
      setDetails(null);
      setError(null);
      return;
    }
    try {
      setError(null);
      const res = await tauriService.parseAddress(addrStr.trim());
      setDetails(res);
      onStatusChange(`Parsed address: Type=${res.node_type_name}, Payload=${res.payload_len}B`);
    } catch (err: any) {
      setError(err?.toString() || 'Failed to parse address');
      setDetails(null);
      onStatusChange('Address parse error');
    }
  };

  const handleConvertNumber = async () => {
    try {
      setBijectionError(null);
      const addrHex = await tauriService.numberToAddress(numberInput);
      setBijectionAddress(addrHex);
      onStatusChange(`Converted natural number to address: ${addrHex}`);
    } catch (err: any) {
      setBijectionError(err?.toString() || 'Conversion error');
    }
  };

  useEffect(() => {
    handleParse(inputAddress);
  }, []);

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors">
      <div>
        <h1 className="text-xl font-bold tracking-tight">Canonical Address Space</h1>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Mathematical decomposition of canonical <code>mfas:v1</code> self-describing addresses and LEB128 serialization.
        </p>
      </div>

      {/* Input & Search */}
      <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/50">
        <label className="block text-xs font-semibold text-gray-600 dark:text-gray-300 mb-2">
          Enter MFAS Address (Hex or URI):
        </label>
        <div className="flex gap-2">
          <div className="relative flex-1">
            <FiSearch className="absolute left-3 top-3 w-4 h-4 text-gray-400" />
            <input
              type="text"
              value={inputAddress}
              onChange={(e) => {
                setInputAddress(e.target.value);
                handleParse(e.target.value);
              }}
              placeholder="e.g. 010048656c6c6f or mfas:v1:data:48656c6c6f"
              className="w-full pl-9 pr-3 py-2 text-xs font-mono rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-1 focus:ring-gray-500"
            />
          </div>
          <button
            onClick={() => handleParse(inputAddress)}
            className="px-4 py-2 text-xs font-medium rounded bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-300 transition-colors"
          >
            Parse
          </button>
        </div>

        {/* Quick sample chips */}
        <div className="flex flex-wrap gap-2 mt-3 items-center text-[11px] text-gray-500">
          <span>Samples:</span>
          {[
            { label: 'Data: "Hello"', val: '010048656c6c6f' },
            { label: 'Repeat: 3x', val: '0103010048656c6c6f03' },
            { label: 'Sequence', val: '010202010041010042' },
          ].map((sample) => (
            <button
              key={sample.label}
              onClick={() => {
                setInputAddress(sample.val);
                handleParse(sample.val);
              }}
              className="px-2 py-0.5 rounded border border-gray-300 dark:border-gray-700 hover:bg-gray-200 dark:hover:bg-gray-800 transition-colors font-mono"
            >
              {sample.label}
            </button>
          ))}
        </div>
      </div>

      {error && (
        <div className="p-3 text-xs rounded border border-red-300 dark:border-red-900 bg-red-50 dark:bg-red-950/40 text-red-700 dark:text-red-400">
          {error}
        </div>
      )}

      {/* Decomposition Grid */}
      {details && (
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          {/* Header Card */}
          <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
            <div className="text-xs font-semibold text-gray-700 dark:text-gray-300">
              Address Header & Metadata
            </div>
            <div className="grid grid-cols-2 gap-2 text-xs">
              <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
                <div className="text-[10px] text-gray-500">Specification Version</div>
                <div className="font-mono font-semibold text-sm">v{details.version} (0x{details.version.toString(16).padStart(2, '0')})</div>
              </div>
              <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
                <div className="text-[10px] text-gray-500">Node Type Tag</div>
                <div className="font-semibold text-sm">
                  {details.node_type_name}{' '}
                  <span className="font-mono text-xs text-gray-500">
                    (0x{details.node_type.toString(16).padStart(2, '0')})
                  </span>
                </div>
              </div>
            </div>

            <div className="p-2.5 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <div className="flex justify-between items-center mb-1">
                <span className="text-[10px] text-gray-500">Canonical Hex Representation:</span>
                <button
                  onClick={() => copyToClipboard(details.canonical_hex)}
                  className="text-[10px] text-gray-500 hover:text-gray-900 dark:hover:text-gray-100 flex items-center gap-1"
                >
                  {copied ? <FiCheck className="text-emerald-500" /> : <FiCode />}
                  {copied ? 'Copied' : 'Copy'}
                </button>
              </div>
              <code className="text-xs font-mono break-all text-gray-800 dark:text-gray-200">
                0x{details.canonical_hex}
              </code>
            </div>
          </div>

          {/* Payload Card */}
          <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
            <div className="text-xs font-semibold text-gray-700 dark:text-gray-300">
              Payload & LEB128 Encodings
            </div>
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <div className="text-[10px] text-gray-500">Payload Length</div>
              <div className="font-mono text-sm">{details.payload_len} bytes</div>
            </div>
            <div className="p-2.5 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block mb-1">LEB128 Binary Stream:</span>
              <code className="text-xs font-mono break-all text-gray-800 dark:text-gray-200">
                {details.leb128_hex}
              </code>
            </div>
          </div>
        </div>
      )}

      {/* Bijective Enumeration Section */}
      <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
        <h2 className="text-sm font-semibold text-gray-800 dark:text-gray-200">
          Mathematical Bijection: N₀ ↔ B*
        </h2>
        <p className="text-xs text-gray-500 dark:text-gray-400">
          Calculates the exact bijective base-256 mapping between natural numbers and finite byte strings, strictly preserving leading zeros.
        </p>

        <div className="flex gap-2">
          <input
            type="text"
            value={numberInput}
            onChange={(e) => setNumberInput(e.target.value)}
            placeholder="Enter natural number N (e.g. 1751477356)"
            className="flex-1 px-3 py-2 text-xs font-mono rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 focus:outline-none"
          />
          <button
            onClick={handleConvertNumber}
            className="px-4 py-2 text-xs font-medium rounded bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-300 transition-colors flex items-center gap-1.5"
          >
            Convert <FiArrowRight className="w-3 h-3" />
          </button>
        </div>

        {bijectionError && (
          <div className="text-xs text-red-600 dark:text-red-400">{bijectionError}</div>
        )}

        {bijectionAddress && (
          <div className="p-3 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 text-xs">
            <span className="text-[10px] text-gray-500 block mb-1">Resulting Canonical Address:</span>
            <code className="font-mono text-gray-800 dark:text-gray-200 break-all">
              0x{bijectionAddress}
            </code>
          </div>
        )}
      </div>
    </div>
  );
};
