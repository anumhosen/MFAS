export type ViewMode = 'address' | 'page' | 'dag' | 'codec' | 'gpu';

export interface AddressDetailsDto {
  raw: string;
  canonical_hex: string;
  version: number;
  node_type: number;
  node_type_name: string;
  payload_hex: string;
  payload_len: number;
  leb128_hex: string;
  is_valid: boolean;
}

export interface RadixCoordinatesDto {
  byte_offset: number;
  page_index: number;
  floor: number;
  room: number;
  wall: number;
  shelf: number;
  volume: number;
  page: number;
  offset_in_page: number;
  formatted: string;
}

export interface PageDetailsDto {
  page_index: number;
  logical_len: number;
  is_full: boolean;
  hex_preview: string;
  ascii_preview: string;
  total_size: number;
  address_hex: string;
}

export interface DagAnalysisDto {
  root_address: string;
  total_nodes: number;
  total_edges: number;
  shared_nodes: number;
  leaf_nodes: number;
  max_depth: number;
  sharing_ratio: number;
}

export interface DagOptimizationDto {
  original_root: string;
  optimized_root: string;
  nodes_before: number;
  nodes_after: number;
  edges_before: number;
  edges_after: number;
  nodes_eliminated: number;
  reduction_pct: number;
  transformations: string[];
}

export interface EncodeResultDto {
  file_path: string;
  root_address: string;
  canonical_hex: string;
  file_size: number;
  sha256: string;
  elapsed_ms: number;
}

export interface DecodeResultDto {
  root_address: string;
  output_path: string;
  bytes_written: number;
  sha256: string;
  elapsed_ms: number;
}

export interface VerifyResultDto {
  file_path: string;
  root_address: string;
  total_bytes: number;
  sha256: string;
  verified: boolean;
  elapsed_ms: number;
}

export interface FileMetadataDto {
  exists: boolean;
  file_name: string;
  file_size: number;
  is_file: boolean;
}

export interface GpuInfoDetailsDto {
  has_gpu: boolean;
  adapter_name: string;
  backend: string;
  device_type: string;
  is_dedicated: boolean;
  max_buffer_mb: number;
  fallback_mode: boolean;
  cpu_evaluator_ready: boolean;
  simd_evaluator_ready: boolean;
}

export interface EvaluatorBenchmarkDto {
  backend: string;
  bytes_evaluated: number;
  elapsed_ms: number;
  throughput_mbps: number;
  status: string;
}

export interface MfasPackageManifest {
  manifest_version: string;
  created_at: string;
  source_file: {
    file_name: string;
    file_size_bytes: number;
    file_type: string;
    sha256: string;
  };
  mfas_address: {
    canonical_uri: string;
    canonical_hex: string;
    node_type: string;
  };
  pipeline: {
    codec: string;
    elapsed_ms: number;
    verified?: boolean;
  };
  reconstruction_guide: {
    recommended_output_name: string;
    expected_sha256: string;
  };
}

