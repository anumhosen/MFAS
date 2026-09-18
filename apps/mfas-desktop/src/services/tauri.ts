import { invoke } from '@tauri-apps/api/core';
import type {
  AddressDetailsDto,
  RadixCoordinatesDto,
  PageDetailsDto,
  DagAnalysisDto,
  DagOptimizationDto,
  EncodeResultDto,
  DecodeResultDto,
  VerifyResultDto,
  GpuInfoDetailsDto,
  EvaluatorBenchmarkDto,
} from '../types';

// Safely detect if running inside Tauri runtime
const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

export const tauriService = {
  // Window controls
  async minimizeWindow(): Promise<void> {
    if (isTauri) {
      await invoke('minimize_window');
    } else {
      console.log('Mock: minimizeWindow');
    }
  },

  async toggleMaximizeWindow(): Promise<boolean> {
    if (isTauri) {
      return await invoke<boolean>('toggle_maximize_window');
    } else {
      console.log('Mock: toggleMaximizeWindow');
      return false;
    }
  },

  async isWindowMaximized(): Promise<boolean> {
    if (isTauri) {
      return await invoke<boolean>('is_window_maximized');
    }
    return false;
  },

  async closeWindow(): Promise<void> {
    if (isTauri) {
      await invoke('close_window');
    } else {
      console.log('Mock: closeWindow');
    }
  },

  // Address
  async parseAddress(raw: string): Promise<AddressDetailsDto> {
    if (isTauri) {
      return await invoke<AddressDetailsDto>('parse_address', { raw });
    }
    // Browser fallback mock
    return {
      raw,
      canonical_hex: '0100' + (raw.length > 4 ? raw.slice(2) : '00'),
      version: 1,
      node_type: 0,
      node_type_name: 'Data',
      payload_hex: '48656c6c6f',
      payload_len: 5,
      leb128_hex: '01 00 05 48 65 6c 6c 6f',
      is_valid: true,
    };
  },

  async formatAddress(nodeTypeVal: number, payloadHex: string): Promise<string> {
    if (isTauri) {
      return await invoke<string>('format_address', { nodeTypeVal, payloadHex });
    }
    return `010${nodeTypeVal}${payloadHex}`;
  },

  async numberToAddress(numStr: string): Promise<string> {
    if (isTauri) {
      return await invoke<string>('number_to_address', { numStr });
    }
    return '010001020304';
  },

  async addressToNumber(rawAddr: string): Promise<string> {
    if (isTauri) {
      return await invoke<string>('address_to_number', { rawAddr });
    }
    return '65536';
  },

  // Page
  async getRadixCoordinates(inputVal: number, isPageIndex: boolean): Promise<RadixCoordinatesDto> {
    if (isTauri) {
      return await invoke<RadixCoordinatesDto>('get_radix_coordinates', { inputVal, isPageIndex });
    }
    const offset = isPageIndex ? inputVal * 4096 : inputVal;
    const pageIndex = Math.floor(offset / 4096);
    return {
      byte_offset: offset,
      page_index: pageIndex,
      floor: Math.floor(pageIndex / 1048576),
      room: Math.floor(pageIndex / 65536) % 16,
      wall: Math.floor(pageIndex / 4096) % 16,
      shelf: Math.floor(pageIndex / 256) % 16,
      volume: Math.floor(pageIndex / 16) % 16,
      page: pageIndex % 16,
      offset_in_page: offset % 4096,
      formatted: `Floor[0].Room[0].Wall[0].Shelf[0].Volume[0].Page[${pageIndex % 16}]+Offset[0x${(offset % 4096).toString(16).toUpperCase().padStart(3, '0')}]`,
    };
  },

  async inspectPageData(rawAddr: string): Promise<PageDetailsDto> {
    if (isTauri) {
      return await invoke<PageDetailsDto>('inspect_page_data', { rawAddr });
    }
    return {
      page_index: 0,
      logical_len: 4096,
      is_full: true,
      hex_preview: '4d46415320506167652044617461',
      ascii_preview: 'MFAS Page Data...',
      total_size: 4096,
      address_hex: '010500000000000000001000',
    };
  },

  // DAG
  async analyzeDag(rawAddr: string): Promise<DagAnalysisDto> {
    if (isTauri) {
      return await invoke<DagAnalysisDto>('analyze_dag', { rawAddr });
    }
    return {
      root_address: rawAddr || 'mfas:v1:seq:[...]',
      total_nodes: 5,
      total_edges: 6,
      shared_nodes: 2,
      leaf_nodes: 2,
      max_depth: 3,
      sharing_ratio: 0.333,
    };
  },

  async getDagDot(rawAddr: string): Promise<string> {
    if (isTauri) {
      return await invoke<string>('get_dag_dot', { rawAddr });
    }
    return `digraph MFAS {
    rankdir=TB;
    node [shape=box, style="filled,rounded"];
    root [label="Root (Sequence)", fillcolor="#F3E5F5"];
    page1 [label="Page 0", fillcolor="#E8F5E9"];
    page2 [label="Page 1", fillcolor="#E8F5E9"];
    root -> page1;
    root -> page2;
}`;
  },

  async optimizeDag(rawAddr: string): Promise<DagOptimizationDto> {
    if (isTauri) {
      return await invoke<DagOptimizationDto>('optimize_dag', { rawAddr });
    }
    return {
      original_root: rawAddr,
      optimized_root: rawAddr + '_optimized',
      nodes_before: 12,
      nodes_after: 7,
      edges_before: 15,
      edges_after: 8,
      nodes_eliminated: 5,
      reduction_pct: 41.67,
      transformations: [
        'Flattened nested sequence with 3 children',
        'Folded 4x adjacent occurrences into Repeat',
        'Eliminated redundant full-range slice',
      ],
    };
  },

  // Codec
  async encodeFile(path: string): Promise<EncodeResultDto> {
    if (isTauri) {
      return await invoke<EncodeResultDto>('encode_file', { path });
    }
    return {
      file_path: path,
      root_address: 'mfas:v1:data:48656c6c6f',
      canonical_hex: '010048656c6c6f',
      file_size: 1048576,
      sha256: '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824',
      elapsed_ms: 12,
    };
  },

  async decodeFile(addrStr: string, outPath: string): Promise<DecodeResultDto> {
    if (isTauri) {
      return await invoke<DecodeResultDto>('decode_file', { addrStr, outPath });
    }
    return {
      root_address: addrStr,
      output_path: outPath,
      bytes_written: 1048576,
      sha256: '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824',
      elapsed_ms: 8,
    };
  },

  async verifyFile(path: string): Promise<VerifyResultDto> {
    if (isTauri) {
      return await invoke<VerifyResultDto>('verify_file', { path });
    }
    return {
      file_path: path,
      root_address: 'mfas:v1:data:48656c6c6f',
      total_bytes: 1048576,
      sha256: '2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824',
      verified: true,
      elapsed_ms: 19,
    };
  },

  // GPU & Benchmark
  async getGpuInfo(): Promise<GpuInfoDetailsDto> {
    if (isTauri) {
      return await invoke<GpuInfoDetailsDto>('get_gpu_info');
    }
    return {
      has_gpu: true,
      adapter_name: 'DirectX 12 / Vulkan Compute Adapter (Emulated)',
      backend: 'DirectX 12',
      device_type: 'Discrete / Integrated GPU',
      is_dedicated: true,
      max_buffer_mb: 2048,
      fallback_mode: false,
      cpu_evaluator_ready: true,
      simd_evaluator_ready: true,
    };
  },

  async runEvaluatorBenchmark(backend: string, sizeBytes: number): Promise<EvaluatorBenchmarkDto> {
    if (isTauri) {
      return await invoke<EvaluatorBenchmarkDto>('run_evaluator_benchmark', { backend, sizeBytes });
    }
    const simulatedMbps = backend === 'simd' ? 4200 : backend === 'gpu' ? 7800 : 2600;
    return {
      backend,
      bytes_evaluated: sizeBytes || 16777216,
      elapsed_ms: 3.8,
      throughput_mbps: simulatedMbps,
      status: 'Completed',
    };
  },
};
