import React, { useState } from 'react';
import { FiGitBranch, FiMinimize2, FiLayers, FiDownload, FiCheck } from 'react-icons/fi';
import { tauriService } from '../../services/tauri';
import type { DagAnalysisDto, DagOptimizationDto } from '../../types';

interface Props {
  onStatusChange: (msg: string) => void;
}

export const DagVisualizerView: React.FC<Props> = ({ onStatusChange }) => {
  const [rootAddress, setRootAddress] = useState('010202010041010042');
  const [analysis, setAnalysis] = useState<DagAnalysisDto | null>(null);
  const [optResult, setOptResult] = useState<DagOptimizationDto | null>(null);
  const [dotCode, setDotCode] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [copiedDot, setCopiedDot] = useState(false);

  const handleAnalyze = async () => {
    if (!rootAddress.trim()) return;
    try {
      setLoading(true);
      setError(null);
      setOptResult(null);

      const [res, dot] = await Promise.all([
        tauriService.analyzeDag(rootAddress.trim()),
        tauriService.getDagDot(rootAddress.trim()),
      ]);

      setAnalysis(res);
      setDotCode(dot);
      onStatusChange(`Analyzed DAG: ${res.total_nodes} nodes, Sharing Ratio=${(res.sharing_ratio * 100).toFixed(1)}%`);
    } catch (err: any) {
      setError(err?.toString() || 'Analysis failed');
    } finally {
      setLoading(false);
    }
  };

  const handleOptimize = async () => {
    if (!rootAddress.trim()) return;
    try {
      setLoading(true);
      setError(null);
      const res = await tauriService.optimizeDag(rootAddress.trim());
      setOptResult(res);
      setRootAddress(res.optimized_root);

      // Re-fetch dot for optimized root
      const dot = await tauriService.getDagDot(res.optimized_root);
      setDotCode(dot);

      onStatusChange(`DAG Optimized: ${res.nodes_eliminated} nodes eliminated (-${res.reduction_pct.toFixed(1)}%)`);
    } catch (err: any) {
      setError(err?.toString() || 'Optimization failed');
    } finally {
      setLoading(false);
    }
  };

  const copyDot = () => {
    navigator.clipboard.writeText(dotCode);
    setCopiedDot(true);
    setTimeout(() => setCopiedDot(false), 2000);
  };

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors">
      <div>
        <h1 className="text-xl font-bold tracking-tight">DAG Engine & Graph Analysis</h1>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Explore directed acyclic graph topology, shared subexpressions, and run automated Common Subexpression Elimination (CSE).
        </p>
      </div>

      {/* Control Box */}
      <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/50 space-y-3">
        <label className="block text-xs font-semibold text-gray-700 dark:text-gray-300">
          Root Address for DAG Traversal:
        </label>
        <div className="flex gap-2">
          <input
            type="text"
            value={rootAddress}
            onChange={(e) => setRootAddress(e.target.value)}
            placeholder="Root address hex or URI"
            className="flex-1 px-3 py-2 text-xs font-mono rounded border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 focus:outline-none"
          />
          <button
            onClick={handleAnalyze}
            disabled={loading}
            className="px-4 py-2 text-xs font-medium rounded bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-300 transition-colors flex items-center gap-1.5"
          >
            <FiGitBranch className="w-3.5 h-3.5" /> Analyze DAG
          </button>
          <button
            onClick={handleOptimize}
            disabled={loading}
            className="px-4 py-2 text-xs font-medium rounded bg-emerald-600 hover:bg-emerald-500 text-white transition-colors flex items-center gap-1.5"
          >
            <FiMinimize2 className="w-3.5 h-3.5" /> Run CSE Optimizer
          </button>
        </div>
      </div>

      {error && (
        <div className="p-3 text-xs rounded border border-red-300 dark:border-red-900 bg-red-50 dark:bg-red-950/40 text-red-700 dark:text-red-400">
          {error}
        </div>
      )}

      {/* Optimization Report if available */}
      {optResult && (
        <div className="p-4 rounded-lg border border-emerald-300 dark:border-emerald-800/80 bg-emerald-50/50 dark:bg-emerald-950/20 space-y-3">
          <div className="flex items-center gap-2 text-xs font-semibold text-emerald-800 dark:text-emerald-300">
            <FiCheck className="w-4 h-4 text-emerald-600" />
            CSE Reduction Report: {optResult.nodes_eliminated} redundant nodes eliminated ({optResult.reduction_pct.toFixed(1)}% reduction)
          </div>

          <div className="grid grid-cols-2 sm:grid-cols-4 gap-2 text-xs">
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-emerald-200 dark:border-emerald-900">
              <span className="text-[10px] text-gray-500">Nodes Before</span>
              <div className="font-bold text-sm">{optResult.nodes_before}</div>
            </div>
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-emerald-200 dark:border-emerald-900">
              <span className="text-[10px] text-gray-500">Nodes After</span>
              <div className="font-bold text-sm text-emerald-600 dark:text-emerald-400">{optResult.nodes_after}</div>
            </div>
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-emerald-200 dark:border-emerald-900">
              <span className="text-[10px] text-gray-500">Edges Before</span>
              <div className="font-bold text-sm">{optResult.edges_before}</div>
            </div>
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-emerald-200 dark:border-emerald-900">
              <span className="text-[10px] text-gray-500">Edges After</span>
              <div className="font-bold text-sm text-emerald-600 dark:text-emerald-400">{optResult.edges_after}</div>
            </div>
          </div>

          {optResult.transformations.length > 0 && (
            <div className="p-2.5 rounded bg-white dark:bg-gray-900 border border-emerald-200 dark:border-emerald-900 text-xs">
              <span className="text-[10px] font-semibold text-gray-500 block mb-1">Applied Reductions:</span>
              <ul className="list-disc pl-4 space-y-0.5 text-gray-700 dark:text-gray-300">
                {optResult.transformations.map((t, idx) => (
                  <li key={idx}>{t}</li>
                ))}
              </ul>
            </div>
          )}
        </div>
      )}

      {/* Metrics Grid */}
      {analysis && (
        <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-5 gap-3">
          {[
            { label: 'Total Vertices (V)', val: analysis.total_nodes },
            { label: 'Directed Edges (E)', val: analysis.total_edges },
            { label: 'Shared Subtrees', val: analysis.shared_nodes },
            { label: 'Topological Depth', val: analysis.max_depth },
            { label: 'Sharing Ratio', val: `${(analysis.sharing_ratio * 100).toFixed(1)}%` },
          ].map((item) => (
            <div
              key={item.label}
              className="p-3 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/50 text-center"
            >
              <div className="text-[10px] text-gray-500">{item.label}</div>
              <div className="text-base font-bold font-mono text-gray-800 dark:text-gray-100 mt-1">
                {item.val}
              </div>
            </div>
          ))}
        </div>
      )}

      {/* Graphviz DOT Export Card */}
      {dotCode && (
        <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-2">
          <div className="flex justify-between items-center">
            <span className="text-xs font-semibold text-gray-700 dark:text-gray-300 flex items-center gap-1.5">
              <FiLayers className="w-3.5 h-3.5" /> Graphviz DOT Format (Exportable)
            </span>
            <button
              onClick={copyDot}
              className="text-xs text-gray-500 hover:text-gray-900 dark:hover:text-gray-100 flex items-center gap-1 px-2 py-1 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800"
            >
              {copiedDot ? <FiCheck className="text-emerald-500" /> : <FiDownload />}
              {copiedDot ? 'Copied' : 'Copy DOT Code'}
            </button>
          </div>
          <pre className="p-3 text-[11px] font-mono rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 overflow-x-auto max-h-60 text-gray-800 dark:text-gray-200">
            {dotCode}
          </pre>
        </div>
      )}
    </div>
  );
};
