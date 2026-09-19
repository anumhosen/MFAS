import React, { useState, useEffect } from 'react';
import {
  FiHardDrive,
  FiCheckCircle,
  FiShield,
  FiArrowRight,
  FiFileText,
  FiDownload,
  FiFolder,
  FiChevronDown,
  FiChevronUp,
  FiCopy,
  FiSave,
  FiCheck,
  FiUploadCloud,
  FiAlertTriangle,
  FiX,
} from 'react-icons/fi';
import { tauriService } from '../../services/tauri';
import type {
  EncodeResultDto,
  DecodeResultDto,
  VerifyResultDto,
  FileMetadataDto,
  MfasPackageManifest,
} from '../../types';

interface Props {
  onStatusChange: (msg: string) => void;
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KiB', 'MiB', 'GiB', 'TiB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return `${(bytes / Math.pow(k, i)).toFixed(2)} ${sizes[i]}`;
};

export const CodecStudioView: React.FC<Props> = ({ onStatusChange }) => {
  const [filePath, setFilePath] = useState('MFAS Plan.md');
  const [fileInfo, setFileInfo] = useState<FileMetadataDto | null>(null);
  const [decodeAddr, setDecodeAddr] = useState('');
  const [outputPath, setOutputPath] = useState('restored_output.bin');

  const [encodeResult, setEncodeResult] = useState<EncodeResultDto | null>(null);
  const [decodeResult, setDecodeResult] = useState<DecodeResultDto | null>(null);
  const [verifyResult, setVerifyResult] = useState<VerifyResultDto | null>(null);

  // Address UI expansion and saving state
  const [addressExpanded, setAddressExpanded] = useState(false);
  const [copiedAddress, setCopiedAddress] = useState(false);
  const [savedMsg, setSavedMsg] = useState<string | null>(null);

  // Manifest load & reconstruct state
  const [loadedManifest, setLoadedManifest] = useState<MfasPackageManifest | null>(null);
  const [manifestVerification, setManifestVerification] = useState<{
    verified: boolean;
    sha256: string;
    expectedSha256: string;
    message: string;
  } | null>(null);

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    if (!filePath.trim()) {
      setFileInfo(null);
      return;
    }
    let isMounted = true;
    tauriService.getFileMetadata(filePath.trim()).then((info) => {
      if (isMounted) {
        setFileInfo(info);
      }
    });
    return () => {
      isMounted = false;
    };
  }, [filePath]);

  const handleBrowseSourceFile = async () => {
    try {
      const selected = await tauriService.pickSourceFile();
      if (selected) {
        setFilePath(selected);
        onStatusChange(`Selected source file: ${selected}`);
      }
    } catch (err: any) {
      setError(err?.toString() || 'File selection failed');
    }
  };

  const handleBrowseDestinationFile = async () => {
    try {
      const selected = await tauriService.pickDestinationFile(outputPath || 'restored_output.bin');
      if (selected) {
        setOutputPath(selected);
        onStatusChange(`Selected destination path: ${selected}`);
      }
    } catch (err: any) {
      setError(err?.toString() || 'Destination selection failed');
    }
  };

  const handleEncode = async () => {
    if (!filePath.trim()) return;
    try {
      setLoading(true);
      setError(null);
      setSavedMsg(null);
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
      setManifestVerification(null);
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

  const handleCopyAddress = (addr: string) => {
    navigator.clipboard.writeText(addr);
    setCopiedAddress(true);
    setTimeout(() => setCopiedAddress(false), 2000);
  };

  const handleSaveAddressText = async () => {
    if (!encodeResult) return;
    try {
      const baseName = fileInfo?.file_name || filePath.split(/[/\\]/).pop() || 'encoded';
      const defaultName = `${baseName}.mfas-addr.txt`;
      const savePath = await tauriService.pickSaveAddressFile(defaultName);
      if (savePath) {
        await tauriService.saveTextFile(savePath, encodeResult.root_address);
        const fileName = savePath.split(/[/\\]/).pop() || savePath;
        setSavedMsg(`Address saved to: ${fileName}`);
        onStatusChange(`Address file saved: ${savePath}`);
        setTimeout(() => setSavedMsg(null), 4000);
      }
    } catch (err: any) {
      setError(err?.toString() || 'Failed to save address file');
    }
  };

  const handleExportManifest = async () => {
    if (!encodeResult) return;
    try {
      const fileName = fileInfo?.file_name || filePath.split(/[/\\]/).pop() || 'unknown_file';
      const ext = fileName.includes('.') ? fileName.split('.').pop() || 'bin' : 'bin';
      const nodeType = encodeResult.root_address.startsWith('mfas:v1:')
        ? encodeResult.root_address.split(':')[2]
        : 'data';

      const manifest: MfasPackageManifest = {
        manifest_version: '1.0',
        created_at: new Date().toISOString(),
        source_file: {
          file_name: fileName,
          file_size_bytes: encodeResult.file_size,
          file_type: ext,
          sha256: encodeResult.sha256,
        },
        mfas_address: {
          canonical_uri: encodeResult.root_address,
          canonical_hex: encodeResult.canonical_hex,
          node_type: nodeType,
        },
        pipeline: {
          codec: 'MFAS-Canonical-v1',
          elapsed_ms: encodeResult.elapsed_ms,
          verified: verifyResult?.verified ?? true,
        },
        reconstruction_guide: {
          recommended_output_name: fileName,
          expected_sha256: encodeResult.sha256,
        },
      };

      const defaultName = `${fileName}.mfas.json`;
      const savePath = await tauriService.pickSaveManifestFile(defaultName);
      if (savePath) {
        await tauriService.saveTextFile(savePath, JSON.stringify(manifest, null, 2));
        const outName = savePath.split(/[/\\]/).pop() || savePath;
        setSavedMsg(`Manifest exported: ${outName}`);
        onStatusChange(`MFAS Package Manifest exported: ${savePath}`);
        setTimeout(() => setSavedMsg(null), 4000);
      }
    } catch (err: any) {
      setError(err?.toString() || 'Failed to export manifest');
    }
  };

  const handleLoadManifest = async () => {
    try {
      const selectedPath = await tauriService.pickManifestFile();
      if (!selectedPath) return;

      const content = await tauriService.readTextFile(selectedPath);
      const manifest: MfasPackageManifest = JSON.parse(content);

      if (!manifest.mfas_address?.canonical_uri) {
        throw new Error('Invalid MFAS manifest: missing mfas_address.canonical_uri');
      }

      setLoadedManifest(manifest);
      setDecodeAddr(manifest.mfas_address.canonical_uri);

      // Pre-fill destination output path using original filename
      const recName = manifest.reconstruction_guide?.recommended_output_name || manifest.source_file.file_name;
      setOutputPath(recName);

      // Reset prior results
      setDecodeResult(null);
      setManifestVerification(null);
      onStatusChange(`Loaded MFAS Manifest: ${manifest.source_file.file_name}`);
    } catch (err: any) {
      setError(err?.toString() || 'Failed to load manifest JSON');
    }
  };

  const handleReconstructAndVerify = async () => {
    if (!decodeAddr.trim() || !outputPath.trim()) return;
    try {
      setLoading(true);
      setError(null);
      setManifestVerification(null);

      // 1. Decode to output path
      const res = await tauriService.decodeFile(decodeAddr.trim(), outputPath.trim());
      setDecodeResult(res);

      // 2. Automated integrity verification against manifest
      if (loadedManifest) {
        const expected = loadedManifest.reconstruction_guide?.expected_sha256 || loadedManifest.source_file.sha256;
        const actual = res.sha256;
        const matches = expected.toLowerCase() === actual.toLowerCase();

        setManifestVerification({
          verified: matches,
          sha256: actual,
          expectedSha256: expected,
          message: matches
            ? 'Lossless reconstruction verified: SHA-256 matches manifest perfectly.'
            : 'Integrity Warning: Reconstructed file hash does not match manifest expected hash.',
        });
        onStatusChange(
          matches
            ? `Reconstructed & verified: SHA-256 match confirmed for ${res.output_path}`
            : `Reconstruction completed with hash mismatch!`
        );
      } else {
        onStatusChange(`Decoded ${res.bytes_written.toLocaleString()} bytes to ${res.output_path}`);
      }
    } catch (err: any) {
      setError(err?.toString() || 'Reconstruction failed');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors">
      <div>
        <h1 className="text-xl font-bold tracking-tight">File Codec & Verification Studio</h1>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Lossless file encoding to MFAS addresses, structured manifest export, streaming reconstruction, and SHA-256 verification.
        </p>
      </div>

      {error && (
        <div className="p-3 text-xs rounded border border-red-300 dark:border-red-900 bg-red-50 dark:bg-red-950/40 text-red-700 dark:text-red-400 flex items-center justify-between">
          <div className="flex items-center gap-2">
            <FiAlertTriangle className="w-4 h-4 shrink-0" />
            <span>{error}</span>
          </div>
          <button
            onClick={() => setError(null)}
            className="text-red-600 dark:text-red-400 hover:text-red-800"
          >
            <FiX className="w-4 h-4" />
          </button>
        </div>
      )}

      {/* Verification Master Card */}
      <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
        <div className="flex flex-wrap justify-between items-center gap-2">
          <label className="text-xs font-semibold text-gray-700 dark:text-gray-300 flex items-center gap-1.5">
            <FiFileText className="w-3.5 h-3.5" /> Source File Path for Encoding & Verification:
          </label>
          <div className="flex gap-2">
            <button
              onClick={handleEncode}
              disabled={loading}
              className="px-3 py-1.5 text-xs font-medium rounded bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-300 transition-colors flex items-center gap-1 shadow-sm"
            >
              <FiHardDrive className="w-3.5 h-3.5" /> Encode to Address
            </button>
            <button
              onClick={handleVerify}
              disabled={loading}
              className="px-3 py-1.5 text-xs font-medium rounded bg-emerald-600 hover:bg-emerald-500 text-white transition-colors flex items-center gap-1 shadow-sm"
            >
              <FiShield className="w-3.5 h-3.5" /> Run End-to-End Verification
            </button>
          </div>
        </div>

        {/* Source File Picker Input Row */}
        <div className="flex gap-2">
          <input
            type="text"
            value={filePath}
            onChange={(e) => setFilePath(e.target.value)}
            placeholder="Select or enter file path (e.g. MFAS Plan.md)"
            className="flex-1 px-3 py-2 text-xs font-mono rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-1 focus:ring-gray-500"
          />
          <button
            type="button"
            onClick={handleBrowseSourceFile}
            className="px-3.5 py-2 text-xs font-medium rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors flex items-center gap-1.5 shadow-sm shrink-0"
            title="Open native file picker"
          >
            <FiFolder className="w-3.5 h-3.5" />
            Browse...
          </button>
        </div>

        {/* Selected File Metadata Chip */}
        {fileInfo && fileInfo.exists && (
          <div className="flex flex-wrap items-center gap-2 pt-0.5 text-[11px] text-gray-600 dark:text-gray-400">
            <span className="flex items-center gap-1 px-2 py-0.5 rounded bg-gray-200/80 dark:bg-gray-800 font-mono text-gray-700 dark:text-gray-300">
              <FiCheckCircle className="w-3 h-3 text-emerald-500" />
              {fileInfo.file_name}
            </span>
            <span className="text-gray-400">•</span>
            <span className="font-mono font-medium text-gray-700 dark:text-gray-300">{formatFileSize(fileInfo.file_size)}</span>
            <span className="text-gray-500">({fileInfo.file_size.toLocaleString()} bytes)</span>
          </div>
        )}
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

      {/* Encode Result Details with Expand/Collapse and Save Options */}
      {encodeResult && (
        <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
          <div className="flex flex-wrap items-center justify-between gap-2">
            <div className="text-xs font-semibold text-gray-800 dark:text-gray-200 flex items-center gap-1.5">
              <FiCheckCircle className="w-4 h-4 text-emerald-500" />
              Encoded MFAS Address Output
            </div>
            <div className="flex flex-wrap gap-2">
              <button
                type="button"
                onClick={() => handleCopyAddress(encodeResult.root_address)}
                className="px-2.5 py-1 text-xs font-medium rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors flex items-center gap-1 shadow-sm"
                title="Copy address to clipboard"
              >
                {copiedAddress ? <FiCheck className="w-3.5 h-3.5 text-emerald-500" /> : <FiCopy className="w-3.5 h-3.5" />}
                {copiedAddress ? 'Copied!' : 'Copy'}
              </button>
              <button
                type="button"
                onClick={handleSaveAddressText}
                className="px-2.5 py-1 text-xs font-medium rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors flex items-center gap-1 shadow-sm"
                title="Save raw address to text file"
              >
                <FiSave className="w-3.5 h-3.5" />
                Save Address (.txt)
              </button>
              <button
                type="button"
                onClick={handleExportManifest}
                className="px-2.5 py-1 text-xs font-medium rounded bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-300 transition-colors flex items-center gap-1 shadow-sm"
                title="Export structured package manifest JSON"
              >
                <FiDownload className="w-3.5 h-3.5" />
                Export Manifest (.json)
              </button>
            </div>
          </div>

          {savedMsg && (
            <div className="p-2 rounded bg-emerald-50 dark:bg-emerald-950/30 border border-emerald-300 dark:border-emerald-800 text-emerald-700 dark:text-emerald-300 text-xs flex items-center gap-1.5">
              <FiCheck className="w-3.5 h-3.5 shrink-0" />
              <span>{savedMsg}</span>
            </div>
          )}

          {/* Expandable Address Container */}
          <div className="p-3 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 relative">
            <span className="text-[10px] uppercase font-semibold text-gray-500 block mb-1">
              Canonical Root Address URI:
            </span>
            <div
              className={`font-mono text-xs break-all text-gray-800 dark:text-gray-200 transition-all duration-200 ${
                addressExpanded ? 'max-h-none' : 'max-h-16 overflow-hidden'
              }`}
            >
              <code>{encodeResult.root_address}</code>
            </div>

            {/* Fade effect when collapsed */}
            {!addressExpanded && (
              <div className="absolute bottom-7 left-0 right-0 h-8 bg-gradient-to-t from-white dark:from-gray-900 to-transparent pointer-events-none rounded-b" />
            )}

            {/* Expand / Collapse Button */}
            <div className="mt-2 pt-1 border-t border-gray-100 dark:border-gray-800/80 flex items-center justify-between text-xs">
              <button
                type="button"
                onClick={() => setAddressExpanded(!addressExpanded)}
                className="text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-gray-100 font-medium flex items-center gap-1 transition-colors"
              >
                {addressExpanded ? (
                  <>
                    <FiChevronUp className="w-3.5 h-3.5" /> Collapse preview
                  </>
                ) : (
                  <>
                    <FiChevronDown className="w-3.5 h-3.5" /> Show full address ({encodeResult.root_address.length} chars)
                  </>
                )}
              </button>
              <span className="text-[11px] text-gray-400 font-mono">
                {formatFileSize(encodeResult.file_size)} • {encodeResult.elapsed_ms}ms
              </span>
            </div>
          </div>
        </div>
      )}

      {/* Decoder Studio Section */}
      <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
        <div className="flex flex-wrap items-center justify-between gap-2">
          <h2 className="text-sm font-semibold text-gray-800 dark:text-gray-200 flex items-center gap-1.5">
            <FiDownload className="w-4 h-4" /> Address Decoder & File Reconstructor
          </h2>
          <button
            type="button"
            onClick={handleLoadManifest}
            className="px-3 py-1.5 text-xs font-medium rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors flex items-center gap-1.5 shadow-sm"
            title="Import an exported MFAS package manifest JSON"
          >
            <FiUploadCloud className="w-3.5 h-3.5" />
            Load Manifest (.mfas.json)
          </button>
        </div>

        {/* Loaded Manifest Banner if active */}
        {loadedManifest && (
          <div className="p-3.5 rounded-lg border border-blue-300 dark:border-blue-900 bg-blue-50/60 dark:bg-blue-950/30 space-y-2 text-xs">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2 font-semibold text-blue-900 dark:text-blue-300">
                <FiFileText className="w-4 h-4 text-blue-600 dark:text-blue-400" />
                Package Manifest Loaded: {loadedManifest.source_file.file_name}
              </div>
              <button
                type="button"
                onClick={() => {
                  setLoadedManifest(null);
                  setManifestVerification(null);
                }}
                className="text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
                title="Clear loaded manifest"
              >
                <FiX className="w-4 h-4" />
              </button>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-3 gap-2 text-[11px]">
              <div className="p-2 rounded bg-white dark:bg-gray-900/80 border border-blue-200 dark:border-blue-900/60">
                <span className="text-gray-500 block">Original Size</span>
                <span className="font-mono font-medium">
                  {formatFileSize(loadedManifest.source_file.file_size_bytes)} ({loadedManifest.source_file.file_size_bytes.toLocaleString()} B)
                </span>
              </div>
              <div className="p-2 rounded bg-white dark:bg-gray-900/80 border border-blue-200 dark:border-blue-900/60">
                <span className="text-gray-500 block">File Type / Codec</span>
                <span className="font-mono font-medium">
                  .{loadedManifest.source_file.file_type} • {loadedManifest.pipeline.codec}
                </span>
              </div>
              <div className="p-2 rounded bg-white dark:bg-gray-900/80 border border-blue-200 dark:border-blue-900/60">
                <span className="text-gray-500 block">Expected SHA-256</span>
                <span className="font-mono text-[10px] truncate block" title={loadedManifest.source_file.sha256}>
                  {loadedManifest.source_file.sha256}
                </span>
              </div>
            </div>

            <div className="pt-1 flex items-center justify-end">
              <button
                type="button"
                onClick={handleReconstructAndVerify}
                disabled={loading}
                className="px-3.5 py-1.5 text-xs font-semibold rounded bg-emerald-600 hover:bg-emerald-500 text-white transition-colors flex items-center gap-1.5 shadow-sm"
              >
                <FiShield className="w-3.5 h-3.5" />
                Reconstruct & Verify SHA-256
              </button>
            </div>
          </div>
        )}

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
                type="button"
                onClick={handleBrowseDestinationFile}
                className="px-3.5 py-2 text-xs font-medium rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-800 text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors flex items-center gap-1.5 shadow-sm shrink-0"
                title="Select destination file location"
              >
                <FiFolder className="w-3.5 h-3.5" />
                Save As...
              </button>
              {loadedManifest ? (
                <button
                  onClick={handleReconstructAndVerify}
                  disabled={loading}
                  className="px-4 py-2 text-xs font-medium rounded bg-emerald-600 hover:bg-emerald-500 text-white transition-colors flex items-center gap-1.5 shrink-0"
                >
                  <FiShield className="w-3.5 h-3.5" /> Reconstruct & Verify
                </button>
              ) : (
                <button
                  onClick={handleDecode}
                  disabled={loading}
                  className="px-4 py-2 text-xs font-medium rounded bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-300 transition-colors flex items-center gap-1 shrink-0"
                >
                  Decode <FiArrowRight className="w-3 h-3" />
                </button>
              )}
            </div>
          </div>
        </div>

        {/* Manifest Verification Outcome Banner */}
        {manifestVerification && (
          <div
            className={`p-3.5 rounded-lg border text-xs space-y-1.5 ${
              manifestVerification.verified
                ? 'border-emerald-300 dark:border-emerald-800 bg-emerald-50/50 dark:bg-emerald-950/20 text-emerald-800 dark:text-emerald-200'
                : 'border-red-300 dark:border-red-800 bg-red-50 dark:bg-red-950/20 text-red-800 dark:text-red-200'
            }`}
          >
            <div className="flex items-center gap-2 font-bold">
              {manifestVerification.verified ? (
                <FiCheckCircle className="w-4 h-4 text-emerald-600 shrink-0" />
              ) : (
                <FiAlertTriangle className="w-4 h-4 text-red-600 shrink-0" />
              )}
              {manifestVerification.message}
            </div>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2 text-[11px] font-mono mt-1">
              <div>
                <span className="text-gray-500 block">Reconstructed SHA-256:</span>
                <span className="break-all">{manifestVerification.sha256}</span>
              </div>
              <div>
                <span className="text-gray-500 block">Manifest Expected SHA-256:</span>
                <span className="break-all">{manifestVerification.expectedSha256}</span>
              </div>
            </div>
          </div>
        )}

        {/* Standard Decode Result if not verified via manifest */}
        {!manifestVerification && decodeResult && (
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

