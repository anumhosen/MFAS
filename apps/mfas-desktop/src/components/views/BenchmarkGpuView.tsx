import React, { useState, useEffect } from 'react';
import { FiZap, FiPlay } from 'react-icons/fi';
import { tauriService } from '../../services/tauri';
import type { GpuInfoDetailsDto, EvaluatorBenchmarkDto } from '../../types';

interface Props {
  onStatusChange: (msg: string) => void;
}

export const BenchmarkGpuView: React.FC<Props> = ({ onStatusChange }) => {
  const [gpuInfo, setGpuInfo] = useState<GpuInfoDetailsDto | null>(null);
  const [benchmarks, setBenchmarks] = useState<{ [backend: string]: EvaluatorBenchmarkDto }>({});
  const [runningBackend, setRunningBackend] = useState<string | null>(null);
  const [testSizeMb, setTestSizeMb] = useState<number>(16);

  const fetchGpuInfo = async () => {
    try {
      const res = await tauriService.getGpuInfo();
      setGpuInfo(res);
      onStatusChange(`Detected Compute Adapter: ${res.adapter_name} (${res.backend})`);
    } catch (err) {
      console.error(err);
    }
  };

  useEffect(() => {
    fetchGpuInfo();
  }, []);

  const runBenchmark = async (backend: string) => {
    try {
      setRunningBackend(backend);
      const sizeBytes = testSizeMb * 1024 * 1024;
      const res = await tauriService.runEvaluatorBenchmark(backend, sizeBytes);
      setBenchmarks((prev) => ({ ...prev, [backend]: res }));
      onStatusChange(`Benchmarked ${backend.toUpperCase()}: ${res.throughput_mbps.toFixed(1)} MB/s`);
    } catch (err: any) {
      console.error(err);
    } finally {
      setRunningBackend(null);
    }
  };

  const runAll = async () => {
    await runBenchmark('cpu');
    await runBenchmark('simd');
    await runBenchmark('gpu');
  };

  const maxThroughput = Math.max(
    ...Object.values(benchmarks).map((b) => b.throughput_mbps),
    1000
  );

  return (
    <div className="flex-1 overflow-y-auto p-6 space-y-6 bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100 transition-colors">
      <div>
        <h1 className="text-xl font-bold tracking-tight">Hardware Evaluators & GPU Benchmarks</h1>
        <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
          Hardware discovery, extensible Evaluator backends, and parallel byte synthesis performance meters.
        </p>
      </div>

      {/* GPU Hardware Adapter Status */}
      {gpuInfo && (
        <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2 text-xs font-semibold text-gray-800 dark:text-gray-200">
              <FiZap className="w-4 h-4 text-amber-500" />
              Compute Hardware Discovery
            </div>
            <span
              className={`px-2 py-0.5 rounded text-[10px] font-semibold ${
                gpuInfo.has_gpu
                  ? 'bg-emerald-100 dark:bg-emerald-950 text-emerald-700 dark:text-emerald-400'
                  : 'bg-amber-100 dark:bg-amber-950 text-amber-700 dark:text-amber-400'
              }`}
            >
              {gpuInfo.has_gpu ? 'Hardware Active' : 'Fallback Active'}
            </span>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-2 text-xs">
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">Adapter Device</span>
              <span className="font-semibold">{gpuInfo.adapter_name}</span>
            </div>
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">Driver Backend</span>
              <span className="font-semibold font-mono">{gpuInfo.backend}</span>
            </div>
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">Max Buffer Size</span>
              <span className="font-semibold">{gpuInfo.max_buffer_mb} MB</span>
            </div>
            <div className="p-2 rounded bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800">
              <span className="text-[10px] text-gray-500 block">SIMD Vectorization</span>
              <span className="font-semibold text-emerald-600 dark:text-emerald-400">Available</span>
            </div>
          </div>
        </div>
      )}

      {/* Benchmark Runner Controls */}
      <div className="p-4 rounded-lg border border-gray-200 dark:border-gray-800 bg-gray-50 dark:bg-gray-900/40 space-y-4">
        <div className="flex flex-wrap justify-between items-center gap-2">
          <div className="flex items-center gap-3">
            <span className="text-xs font-semibold text-gray-700 dark:text-gray-300">
              Test Workload Size:
            </span>
            <div className="flex gap-1 text-xs">
              {[4, 16, 64].map((size) => (
                <button
                  key={size}
                  onClick={() => setTestSizeMb(size)}
                  className={`px-2.5 py-1 rounded transition-colors ${
                    testSizeMb === size
                      ? 'bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 font-medium'
                      : 'border border-gray-300 dark:border-gray-700 text-gray-600 dark:text-gray-400 hover:bg-gray-200 dark:hover:bg-gray-800'
                  }`}
                >
                  {size} MB
                </button>
              ))}
            </div>
          </div>

          <button
            onClick={runAll}
            disabled={runningBackend !== null}
            className="px-4 py-2 text-xs font-medium rounded bg-gray-800 dark:bg-gray-200 text-gray-100 dark:text-gray-900 hover:bg-gray-700 dark:hover:bg-gray-300 transition-colors flex items-center gap-1.5"
          >
            <FiPlay className="w-3.5 h-3.5" /> Run Comparison Benchmark
          </button>
        </div>

        {/* Backend Performance Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-3">
          {[
            { id: 'cpu', label: 'CPU Evaluator', desc: 'Single-thread baseline synthesis' },
            { id: 'simd', label: 'SIMD Evaluator', desc: 'Vectorized parallel instructions' },
            { id: 'gpu', label: 'GPU Evaluator', desc: 'DirectX 12 / Vulkan parallel kernels' },
          ].map((item) => {
            const bench = benchmarks[item.id];
            const isRunning = runningBackend === item.id;
            const pct = bench ? (bench.throughput_mbps / maxThroughput) * 100 : 0;

            return (
              <div
                key={item.id}
                className="p-3.5 rounded-lg border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 flex flex-col justify-between space-y-3"
              >
                <div>
                  <div className="flex justify-between items-center">
                    <span className="text-xs font-bold text-gray-800 dark:text-gray-200">
                      {item.label}
                    </span>
                    <button
                      onClick={() => runBenchmark(item.id)}
                      disabled={runningBackend !== null}
                      className="text-[10px] px-2 py-0.5 rounded border border-gray-300 dark:border-gray-700 hover:bg-gray-100 dark:hover:bg-gray-800 text-gray-600 dark:text-gray-300 transition-colors"
                    >
                      {isRunning ? 'Running...' : 'Benchmark'}
                    </button>
                  </div>
                  <p className="text-[10px] text-gray-500 mt-0.5">{item.desc}</p>
                </div>

                {bench ? (
                  <div className="space-y-1.5">
                    <div className="flex justify-between items-baseline">
                      <span className="text-lg font-bold font-mono text-gray-900 dark:text-gray-100">
                        {bench.throughput_mbps.toFixed(1)}{' '}
                        <span className="text-xs font-normal text-gray-500">MB/s</span>
                      </span>
                      <span className="text-[10px] text-gray-400 font-mono">
                        {bench.elapsed_ms.toFixed(1)} ms
                      </span>
                    </div>
                    {/* Visual meter */}
                    <div className="w-full h-1.5 rounded-full bg-gray-200 dark:bg-gray-800 overflow-hidden">
                      <div
                        className="h-full bg-gray-800 dark:bg-gray-200 rounded-full transition-all duration-300"
                        style={{ width: `${pct}%` }}
                      />
                    </div>
                  </div>
                ) : (
                  <div className="text-xs text-gray-400 italic py-2 text-center">
                    Not benchmarked yet
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
