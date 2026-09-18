# MFAS: Mathematical File Address Space

[![CI / Release](https://github.com/anumhosen/MFAS/actions/workflows/release.yml/badge.svg)](https://github.com/anumhosen/MFAS/actions/workflows/release.yml)
[![Tests](https://img.shields.io/badge/Tests-99%20Passing-brightgreen)](https://github.com/anumhosen/MFAS)
[![Rust](https://img.shields.io/badge/Rust-2021%20Edition-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/Tauri-v2.0-blue.svg)](https://tauri.app)
[![License](https://img.shields.io/badge/License-MIT%2FApache--2.0-blue.svg)](LICENSE)

**MFAS (Mathematical File Address Space)** is a deterministic, content-addressed, mathematically rigorous data representation and reconstruction engine. MFAS treats arbitrary digital information not merely as transient filesystem paths or database rows, but as unique, coordinate-mapped objects within a well-defined discrete mathematical space.

MFAS combines:
- A canonical **v1 self-describing address format** and binary LEB128 serialization.
- A provable **bijective base-256 mapping** between natural numbers and finite byte strings ($\mathbb{N}_0 \leftrightarrow B^*$), strictly preserving leading zeros.
- A **Radix-16 spatial hierarchy** based on canonical $16^3 = 4096$-byte logical pages.
- A **recursive Directed Acyclic Graph (DAG)** node engine featuring cycle rejection, Common Subexpression Elimination (CSE), and Graphviz DOT export.
- A **constant-memory ($O(1)$ RAM) streaming codec** with on-the-fly run-length deduplication and cryptographic SHA-256 integrity verification.
- An extensible **Hardware Evaluator** pipeline supporting CPU baseline, SIMD vectorization, and GPU compute discovery.
- A modern **Tauri 2 Desktop Explorer** with Tailwind CSS gray scale aesthetics, custom window controls, and interactive mathematical inspection views.

---

## The Core Invariant

$$\text{decode}(\text{encode}(X)) == X \quad \forall X \in B^*$$

For any supported finite byte sequence $X$:
1. Encoding is **deterministic** and **lossless**.
2. Reconstruction restores the exact original byte sequence bit-for-bit.
3. Cryptographic integrity is validated via streaming SHA-256 digests:
   $$\text{SHA256}(\text{decode}(\text{encode}(X))) == \text{SHA256}(X)$$

> [!NOTE]
> **Information-Theoretic Principle**: MFAS does *not* claim that an arbitrarily large high-entropy file can be compressed into a tiny address. For arbitrary high-entropy data, the representation contains comparable information. MFAS provides a mathematically unified address space and allows structured or repetitive data to enjoy compact, deduplicated DAG representations.

---

## Workspace Architecture

```text
MFAS/
├── .github/workflows/
│   └── release.yml             # Cross-platform multi-artifact auto-release workflow
├── crates/
│   ├── mfas-core/              # Mathematical engine & core abstractions
│   │   ├── src/address/        # Address types, canonical URI parser, LEB128 codec
│   │   ├── src/enumeration/    # Bijective N <-> B* mapping preserving leading zeros
│   │   ├── src/page/           # 4096-byte pages & Radix-16 spatial coordinates
│   │   ├── src/node/           # Recursive node model (Data, Ref, Seq, Rep, Slice, Page)
│   │   ├── src/resolver/       # Streaming resolver, memoization cache, cycle detection
│   │   ├── src/codec/          # Sharded FileStore, streaming pipeline, SHA-256 verifier
│   │   ├── src/inspector/      # ASCII hierarchy tree formatting & metadata inspection
│   │   ├── src/synthetic/      # Deterministic stream generators (counter, repeat, zeros)
│   │   ├── src/dag/            # DAG topology metrics, Graphviz DOT export, CSE optimizer
│   │   └── src/bench/          # Micro-benchmark engine (throughput, latency, I/O)
│   ├── mfas-cli/               # Production CLI binary (mfas) with 10 command suites
│   └── mfas-gpu/               # Evaluator trait, CpuEvaluator, SimdEvaluator, GpuEvaluator
└── apps/
    └── mfas-desktop/           # Tauri 2 Desktop Explorer
        ├── src-tauri/          # Rust command layer (address, page, dag, codec, gpu, window)
        └── src/                # React 19 + TypeScript + Tailwind CSS (Gray scale theme)
```

---

## Core Mathematical Foundations

### 1. Canonical Address Model v1
Addresses are typed, versioned, self-describing objects with two isomorphic representations:
- **Canonical URI Syntax**: `mfas:v1:<node_type>:<hex_payload>`
- **Canonical Binary Layout**:
  ```text
  [Magic: "MFAS" (4B)][Version: 0x01 (1B)][TypeID (1B)][LEB128 Payload Length][Payload Bytes...]
  ```
- **Node Primitives**:
  - `Data` (`0x01`): Literal inline byte sequence.
  - `Reference` (`0x02`): Explicit reference to another address.
  - `Sequence` (`0x03`): Ordered concatenation of child addresses.
  - `Repeat` (`0x04`): Repetition of a child node $n$ times.
  - `Slice` (`0x05`): Offset and length window into a child node.
  - `Page` (`0x06`): Canonical 4096-byte logical page.

### 2. Bijective Natural Number Mapping ($\mathbb{N}_0 \leftrightarrow B^*$)
Conventional base conversions cannot distinguish strings with identical values but differing leading zeros (e.g. `[0x01]`, `[0x00, 0x01]`, `[0x00, 0x00, 0x01]`). MFAS resolves this via length-partitioned bijective base-256 mapping:
$$\text{Offset}(L) = \sum_{i=0}^{L-1} 256^i = \frac{256^L - 1}{255}$$
$$N = \text{Offset}(|B|) + \text{IntegerBE}(B)$$
Every finite byte string maps to a unique natural number, and vice versa.

### 3. Radix-16 Spatial Coordinate Hierarchy
Data offsets are decomposed into base-16 spatial coordinates using $16^3 = 4096$-byte pages:
```text
Floor[4 GiB] . Room[256 MiB] . Wall[16 MiB] . Shelf[1 MiB] . Volume[64 KiB] . Page[4 KiB] + Offset[0x000..0xFFF]
```

### 4. DAG Optimization & Common Subexpression Elimination (CSE)
MFAS represents recursive data as Directed Acyclic Graphs. The CSE engine automatically:
- Collapses redundant reference chains: $\text{Ref}(\text{Ref}(X)) \to \text{Ref}(X)$
- Folds adjacent identical sequence items into compact repeats: $[A, A, A] \to \text{Repeat}(A, 3)$
- Flattens nested sequences and eliminates redundant full-range slices.
- Exports structural diagrams in standard Graphviz DOT format.

---

## CLI Usage Guide

The `mfas` CLI is a self-contained binary offering 10 primary subcommands:

### Address Operations
```bash
# Parse and inspect an address URI or binary hex
mfas address parse mfas:v1:data:48656c6c6f
mfas address parse mfas:v1:data:48656c6c6f --json

# Format components into canonical URI
mfas address format -t data -p 48656c6c6f

# Validate syntax and payload
mfas address validate mfas:v1:data:48656c6c6f
```

### Bijective Enumeration
```bash
# Convert address payload to its unique natural number
mfas number mfas:v1:data:48656c6c6f

# Convert natural number to canonical address
mfas address 1751477356
```

### Page & Radix Coordinates
```bash
# Compute spatial radix coordinates for a byte offset
mfas page 65536

# Compute radix coordinates by logical page index
mfas page 16 --page-index
```

### File Codec & Cryptographic Verification
```bash
# Encode file to canonical root address (streaming, O(1) RAM)
mfas encode document.pdf

# Decode address to output file
mfas decode <address> restored_document.pdf

# Verify lossless reconstructibility and SHA-256 match
mfas verify document.pdf
```

### DAG Analysis & Graphviz Export
```bash
# Display topological metrics (V, E, shared nodes, depth, sharing ratio)
mfas dag stats <address>

# Export DAG to standard Graphviz DOT language
mfas dag dot <address> > graph.dot

# Optimize DAG via Common Subexpression Elimination (CSE)
mfas dag optimize <address>
```

### Hardware Evaluators & Benchmarks
```bash
# Display compute hardware discovery and fallback status
mfas gpu info

# Compare CPU vs SIMD vs GPU evaluator throughput
mfas gpu bench --size 16777216

# Run full micro-benchmark suite (address, page, codec, DAG)
mfas bench all
```

---

## Desktop GUI Explorer

The desktop explorer is built with **Tauri v2**, **React 19**, **TypeScript**, and **Tailwind CSS** (utilizing the curated Tailwind gray scale for dark/light theme).

### Features
- **Custom Frameless Titlebar**: Native window dragging (`data-tauri-drag-region`), dark/light mode toggle, and window controls using `react-icons/vsc` (`VscChromeMinimize`, `VscChromeMaximize`, `VscChromeClose`).
- **Address Space Explorer**: Decomposes addresses, shows binary LEB128 streams, and tests bijective natural number mappings.
- **Radix-16 Pages**: Interactive spatial hierarchy calculator with quick boundary jumps.
- **DAG Visualizer**: Live metrics, CSE reduction reports, and exportable Graphviz DOT viewer.
- **Codec Studio**: File encoding, decoding, and end-to-end SHA-256 verification runner.
- **Hardware & GPU Panel**: Real-time throughput meters comparing CPU, SIMD, and GPU batch evaluation.

### Running Locally
```bash
# Navigate to desktop app
cd apps/mfas-desktop

# Install dependencies
npm install

# Start development server with Tauri desktop window
npm run tauri dev

# Build production installer
npm run tauri build
```

---

## Automated Verification

The entire workspace maintains a strict automated test suite:

```bash
cargo test --workspace
```

```text
running 52 tests in crates/mfas-core (unit tests) ... ok
running 3 tests in crates/mfas-core (100MB streaming stress tests) ... ok
running 37 tests in crates/mfas-cli (integration tests) ... ok
running 4 tests in crates/mfas-gpu (evaluator & fallback tests) ... ok
running 3 tests in apps/mfas-desktop/src-tauri (desktop commands) ... ok

Test result: 99 passed; 0 failed; 0 ignored; finished with exit code 0.
```

---

## Continuous Integration & Auto-Release

The repository includes a GitHub Actions workflow (`.github/workflows/release.yml`) that automatically builds and packages both the **CLI binary** and **Tauri Desktop installers** across all major platforms whenever a version tag (`v*`) is pushed:

| Platform | Architecture | CLI Binary | Desktop GUI Installer |
|---|---|---|---|
| **Windows** | x86_64 | `mfas.exe` (`.zip`) | `.msi`, `.exe` (NSIS) |
| **Linux** | x86_64 | `mfas` (`.tar.gz`) | `.deb`, `.AppImage` |
| **macOS** | Intel (x86_64) | `mfas` (`.tar.gz`) | `.dmg`, `.app` |
| **macOS** | Apple Silicon (aarch64) | `mfas` (`.tar.gz`) | `.dmg`, `.app` |

Releases automatically generate SHA-256 checksums (`SHA256SUMS.txt`) and attach all assets to the GitHub Release.

---

## License

Dual-licensed under either of:
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT License](http://opensource.org/licenses/MIT)

at your option.
