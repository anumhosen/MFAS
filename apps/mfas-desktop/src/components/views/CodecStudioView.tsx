import React, { useState } from 'react';
import { FiHardDrive, FiCheckCircle, FiShield, FiArrowRight, FiFileText, FiDownload } from 'react-icons/fi';
import { tauriService } from '../../services/tauri';
import type { EncodeResultDto, DecodeResultDto, VerifyResultDto } from '../../types';

interface Props {
  onStatusChange: (msg: string) => void;
}

export const CodecStudioView: React.FC<Props> = ({ onStatusChange }) => {
  const [filePath, setFilePath] = useState('MFAS Plan.md');
  const [decodeAddr, setDecodeAddr] = useState('');
  const [outputPath, setOutputPath] = useState('restored_output.bin');

  const [encodeResult, setEncodeResult] = useState<EncodeResultDto | null>(null);
  const [decodeResult, setDecodeResult] = useState<DecodeResultDto | null>(null);
  const [verifyResult, setVerifyResult] = useState<VerifyResultDto | null>(null);

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleEncode = async () => {
    if (!filePath.trim()) return;
    try {
      setLoading(true);
      setError(null);
      const res = await tauriService.encodeFile(filePath.trim());
      setEncodeResult(res);
      setDecodeAddr(res.root_address);
      onStatusChange(`Encoded file to address in ${res.elapsed_ms}ms: ${res.root_address}`);
    } catch (err: any) {
      setError(err?.toString() || 'Encoding failed');
    } finally {
      setLoading(false);
    }
  };

  const handleDecode = async () => {
    if (!decodeAddr.trim() || !outputPath.trim()) return;
    try {
      setLoading(true);
      setError(null);
      const res = await tauriService.decodeFile(decodeAddr.trim(), outputPath.trim());
      setDecodeResult(res);
      onStatusChange(`Decoded ${res.bytes_written.toLocaleString()} bytes to ${res.output_path} in ${res.elapsed_ms}ms`);
    } catch (err: any) {
      setError(err?.toString() || 'Decoding failed');
    } finally {
      setLoading(false);
    }
  };

  const handleVerify = async () => {
    if (!filePath.trim()) return;
    try {
      setLoading(true);
      setError(null);
      const res = await tauriService.verifyFile(filePath.trim());
      setVerifyResult(res);
      onStatusChange(`Integrity verified: SHA-256 match = ${res.verified}`);
    } catch (err: any) {
      setError(err?.toString() || 'Verification failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors">
      <div>
        <h1 className="text-xl font-bold tracking-tight">File Codec & Verification Studio</h1>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Lossless file encoding to MFAS addresses, streaming reconstruction, and SHA-256 cryptographic verification.
        </p>
      </div>

      {error && (
        <div className="p-3 text-xs rounded border border-red-300 dark:border-red-900 bg-red-50 dark:bg-red-950/40 text-red-700 dark:text-red-400">
          {error}
        </div>
      )}

      {/* Verification Master Card */}
      <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
        <div className="flex justify-between items-center">
          <label className="text-xs font-semibold text-gray-700 dark:text-gray-300 flex items-center gap-1.5">
            <FiFileText className="w-3.5 h-3.5" /> Source File Path for Encoding & Verification:
          </label>
          <div className="flex gap-2">
            <button
              onClick={handleEncode}
              disabled={loading}
              className="px-3 py-1.5 text-xs font-medium rounded bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-300 transition-colors flex items-center gap-1"
            >
              <FiHardDrive className="w-3.5 h-3.5" /> Encode to Address
            </button>
            <button
              onClick={handleVerify}
              disabled={loading}
              className="px-3 py-1.5 text-xs font-medium rounded bg-emerald-600 hover:bg-emerald-500 text-white transition-colors flex items-center gap-1"
            >
              <FiShield className="w-3.5 h-3.5" /> Run End-to-End Verification
            </button>
          </div>
        </div>

        <input
          type="text"
          value={filePath}
          onChange={(e) => setFilePath(e.target.value)}
          placeholder="Path to file (e.g. MFAS Plan.md)"
          className="w-full px-3 py-2 text-xs font-mono rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 focus:outline-none"
        />
      </div>

      {/* Verification Result Banner */}
      {verifyResult && (
        <div
          className={`p-4 rounded-lg border ${
            verifyResult.verified
              ? 'border-emerald-300 dark:border-emerald-800 bg-emerald-50/50 dark:bg-emerald-950/20'
              : 'border-red-300 dark:border-red-800 bg-red-50 dark:bg-red-950/20'
          } space-y-2`}
        >
          <div className="flex items-center gap-2 text-xs font-bold">
            <FiCheckCircle
              className={`w-4 h-4 ${verifyResult.verified ? 'text-emerald-600' : 'text-red-600'}`}
            />
            {verifyResult.verified
              ? 'Lossless Reconstructibility Invariant Verified: decode(encode(file)) == file'
              : 'Verification Failed: Hash Mismatch'}
          </div>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-2 text-xs">
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">Total Bytes Verified</span>
              <span className="font-mono font-semibold">{verifyResult.total_bytes.toLocaleString()} B</span>
            </div>
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">SHA-256 Checksum</span>
              <span className="font-mono text-[10px] break-all">{verifyResult.sha256}</span>
            </div>
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">Total Pipeline Time</span>
              <span className="font-mono font-semibold">{verifyResult.elapsed_ms} ms</span>
            </div>
          </div>
        </div>
      )}

      {/* Encode Result Details */}
      {encodeResult && (
        <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
          <div className="text-xs font-semibold text-gray-700 dark:text-gray-300">
            Encoded MFAS Address Output
          </div>
          <div className="p-2.5 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
            <span className="text-[10px] text-gray-500 block mb-1">Canonical Root Address URI:</span>
            <code className="text-xs font-mono break-all text-gray-800 dark:text-gray-200">
              {encodeResult.root_address}
            </code>
          </div>
        </div>
      )}

      {/* Decoder Studio Section */}
      <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
        <h2 className="text-sm font-semibold text-gray-800 dark:text-gray-200 flex items-center gap-1.5">
          <FiDownload className="w-4 h-4" /> Address Decoder & File Reconstructor
        </h2>
        <div className="space-y-2">
          <div>
            <label className="text-[10px] uppercase font-semibold text-gray-500 block mb-1">
              Source Address to Reconstruct:
            </label>
            <input
              type="text"
              value={decodeAddr}
              onChange={(e) => setDecodeAddr(e.target.value)}
              placeholder="e.g. mfas:v1:seq:... or canonical hex"
              className="w-full px-3 py-2 text-xs font-mono rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 focus:outline-none"
            />
          </div>
          <div>
            <label className="text-[10px] uppercase font-semibold text-gray-500 block mb-1">
              Destination File Path:
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={outputPath}
                onChange={(e) => setOutputPath(e.target.value)}
                placeholder="e.g. restored_output.bin"
                className="flex-1 px-3 py-2 text-xs font-mono rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 focus:outline-none"
              />
              <button
                onClick={handleDecode}
                disabled={loading}
                className="px-4 py-2 text-xs font-medium rounded bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-300 transition-colors flex items-center gap-1"
              >
                Decode <FiArrowRight className="w-3 h-3" />
              </button>
            </div>
          </div>
        </div>

        {decodeResult && (
          <div className="p-3 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 text-xs space-y-1">
            <div className="flex items-center gap-1.5 text-emerald-600 dark:text-emerald-400 font-semibold">
              <FiCheckCircle className="w-3.5 h-3.5" /> Reconstruction complete!
            </div>
            <div className="text-gray-600 dark:text-gray-400">
              Wrote <b>{decodeResult.bytes_written.toLocaleString()} bytes</b> to{' '}
              <code>{decodeResult.output_path}</code> in {decodeResult.elapsed_ms}ms.
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
