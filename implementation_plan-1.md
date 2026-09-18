# Phase 9 Implementation Plan: DAG Engine & Optimization

In this phase, we implement the **Directed Acyclic Graph (DAG) Engine**, **Memoization & Evaluation Caching**, **Common Subexpression Elimination (CSE)**, **Graph Normalization/Reduction**, **Graphviz DOT Visualization**, and the CLI subcommand **`mfas dag`**.

---

## Proposed Changes

### Core Library: `crates/mfas-core`

#### [MODIFY] [resolver/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/resolver/mod.rs)
- **Evaluation Caching / Memoization**:
  - Add bounded memoization cache (`HashMap<Address, Vec<u8>>`) to `ResolverState` for resolved nodes up to a threshold (e.g. 64 KiB), enabling instantaneous reuse when evaluating shared nodes (diamond structures) in a DAG.
  - Optimize `Node::Repeat { target, count }` to resolve the target once into a reusable memory buffer (when small) and write repeatedly, avoiding redundant recursive descent.

#### [NEW] [dag/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/dag/mod.rs)
- **`DagGraph` Structure**:
  - Topological exploration from a root `Address` using `AddressStore`.
  - Node tracking, directed edges, parent-child maps, in-degree and out-degree metrics.
  - Calculation of `DagStats`:
    - `total_nodes`: Number of vertices $V$.
    - `total_edges`: Number of directed edges $E$.
    - `shared_nodes`: Nodes with in-degree $> 1$ (multi-parent shared subtrees).
    - `sharing_ratio`: Measure of DAG de-duplication efficiency ($(E - V + 1) / E$).
    - `max_depth`: Longest path from root to leaf.
    - `leaf_nodes`: Count of terminal data/page nodes.
- **Graphviz DOT Exporter (`to_dot`)**:
  - Emits formatted Graphviz syntax with color-coded nodes per `NodeType`, displaying size, child counts, and truncated address identifiers.
- **Common Subexpression Elimination (CSE) & Graph Reduction**:
  - Recursive bottom-up structural canonicalization.
  - Fold adjacent identical items in `Sequence` into `Repeat`.
  - Flatten associative nested sequences (`Sequence([Sequence([A, B]), C]) -> Sequence([A, B, C])`).
  - Collapse single-child sequences (`Sequence([A]) -> A`).
  - Collapse redundant slices (`Slice(A, 0, len) -> A`).
  - Collapse identity repeats (`Repeat(A, 1) -> A`).
  - Emits `OptimizationReport` comparing before/after stats.

#### [MODIFY] [lib.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/lib.rs)
- Export `dag` module: `DagGraph`, `DagStats`, `OptimizationReport`, `build_dag`, `optimize_dag`.

---

### CLI Interface: `crates/mfas-cli`

#### [NEW] [commands/dag.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/dag.rs)
Implement `mfas dag` subcommand with actions:
- `mfas dag stats <address>`: Computes and displays graph topology metrics (or `--json`).
- `mfas dag dot <address> [-o <file.dot>]`: Generates Graphviz DOT representation.
- `mfas dag optimize <address>`: Executes CSE and normalization, persists newly generated nodes to `FileStore`, and outputs the optimized root address and optimization delta.

#### [MODIFY] [commands/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/mod.rs)
- Register `pub mod dag;`.

#### [MODIFY] [main.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/main.rs)
- Add `Commands::Dag(DagArgs)` variant and command routing.

---

### Verification & Testing

#### [NEW] [crates/mfas-cli/tests/cli_dag_tests.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/tests/cli_dag_tests.rs)
- `test_cli_dag_stats_diamond`: Verifies node sharing detection and statistics on a diamond DAG.
- `test_cli_dag_dot_export`: Verifies valid Graphviz DOT output structure.
- `test_cli_dag_optimize`: Verifies CSE reduction and byte-for-byte resolution preservation.

#### Core Unit Tests in `crates/mfas-core/src/dag/mod.rs`
- In-degree/out-degree calculation.
- Shared reference detection.
- Graphviz DOT syntax validity.
- Optimization preserving data integrity (`resolve(optimized) == resolve(original)`).

---

## Verification Plan

### Automated Tests
Run:
```bash
cargo test --workspace
```
Strictly complying with user rules:
- **Never run `cargo check`**.
- **Never delete files automatically**.
