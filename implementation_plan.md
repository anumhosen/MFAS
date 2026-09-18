# Phase-by-Phase Plan: MFAS CLI & Core Engine

MFAS (**Mathematical File Address Space**) is a mathematical addressing and reconstruction system for finite digital data ($B^*$). This implementation plan outlines the phase-by-phase development of **`mfas-cli` in lockstep with `mfas-core`**, establishing a usable command-line testbed from the very first phase.

---

## Decisions Confirmed During Grilling

> [!NOTE]
> 1. **Incremental CLI Evolution**: The CLI will not be postponed until Phase 8. Instead, `mfas-cli` will expand with dedicated subcommands in each phase as corresponding core features are implemented.
> 2. **Canonical Address Specification**: Documented in [Address Format.md](file:///d:/Development/TAURI/MFAS/Address%20Format.md) as a versioned, typed hexadecimal URI (`mfas:v1:<node_type>:<hex_payload>`) with total ordering and binary serialization format.
> 3. **Subcommand Architecture**: Flat top-level commands (`mfas address`, `mfas number`, `mfas page`, `mfas encode`, `mfas decode`, `mfas verify`, `mfas inspect`, `mfas generate`) powered by `clap` derive macros.
> 4. **Dual Output Mode**: Terminal output defaults to styled human-readable tables/trees, with global `--json` and `--quiet` flags for script and test automation.
> 5. **Arbitrary-Precision Numbers**: Mapping $N \leftrightarrow B^*$ utilizes `num-bigint` to prevent overflow on arbitrary byte lengths.
> 6. **Desktop Isolation**: Tauri desktop app remains decoupled for later phases (Phase 13), keeping `mfas-core` and `mfas-cli` free of GUI dependencies.

---

## Proposed Changes by Phase

### Phase 1: Workspace & CLI Base Scaffold

Establish the Cargo workspace and baseline CLI infrastructure.

#### [NEW] [Cargo.toml](file:///d:/Development/TAURI/MFAS/Cargo.toml)
Root workspace defining member crates:
- `crates/mfas-core`
- `crates/mfas-cli`
- `crates/mfas-gpu` (skeleton interface crate for future phases)

#### [NEW] [crates/mfas-core/Cargo.toml](file:///d:/Development/TAURI/MFAS/crates/mfas-core/Cargo.toml)
Core library with zero GUI dependencies; dependencies: `thiserror`, `serde`, `hex`, `num-bigint`, `sha2`.

#### [NEW] [crates/mfas-cli/Cargo.toml](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/Cargo.toml)
CLI binary crate; dependencies: `mfas-core`, `clap` (derive feature), `serde_json`, `colored`/`tabled` (or minimalist ASCII formatting).

#### [NEW] [crates/mfas-cli/src/main.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/main.rs)
Top-level entrypoint parsing global flags (`--json`, `--quiet`, `--verbose`) and dispatching subcommands.

---

### Phase 2: Address Mathematics & `mfas address`

Implement strongly-typed address handling according to [Address Format.md](file:///d:/Development/TAURI/MFAS/Address%20Format.md).

#### [NEW] [crates/mfas-core/src/address/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/address/mod.rs)
- `Address`: Internal compact numeric/byte representation.
- `NodeType`: Enum for `Data`, `Ref`, `Seq`, `Rep`, `Slice`, `Page`.
- Parsing & Canonical Formatting: Validation for lower-case hex, even byte length, correct prefix `mfas:v1:<type>:<payload>`.
- Binary serialization (`MFAS` magic header, version, type, varint length, payload).
- Equality, lexicographical ordering (`Ord`), and total validation.

#### [NEW] [crates/mfas-cli/src/commands/address.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/address.rs)
CLI subcommand `mfas address`:
```bash
mfas address parse "mfas:v1:data:48656c6c6f" [--json]
mfas address format --type data --payload 48656c6c6f
mfas address validate "mfas:v1:data:48656c6c6f"
mfas address to-binary "mfas:v1:data:48656c6c6f"
```

---

### Phase 3: Natural Number Mapping & `mfas number`

Implement bijective mapping $N \leftrightarrow B^*$ separating length boundaries (e.g. `01`, `0001`, `000001`).

#### [NEW] [crates/mfas-core/src/enumeration/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/enumeration/mod.rs)
- Bijective base-256 enumeration:
  $$\text{bytes\_to\_number}(B) \longleftrightarrow \text{number\_to\_bytes}(N)$$
  distinguishing leading zeros through bijective base-256 numeration ($k$-length strings offset by $\sum_{i=1}^{k-1} 256^i$).
- Invariant: `number_to_bytes(bytes_to_number(data)) == data` for all byte slices.

#### [NEW] [crates/mfas-cli/src/commands/number.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/number.rs)
CLI subcommand `mfas number`:
```bash
mfas number encode "hello" [--hex]
mfas number decode 1413829035 [--hex]
```

---

### Phase 4: Logical Pages & Radix-16 Hierarchy & `mfas page`

Implement 4096-byte pages, partial final pages, and Radix-16 spatial hierarchy.

#### [NEW] [crates/mfas-core/src/page/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/page/mod.rs)
- `PAGE_SIZE = 4096` bytes ($16^3$).
- `Page`: holds data buffer and `logical_len` ($1 \le \text{len} \le 4096$).
- Radix-16 hierarchy coordinates: Byte ($16^0$), Word ($16^1$), Block ($16^2$), Page ($16^3$), Volume ($16^4$), Shelf ($16^5$), Wall ($16^6$), Room ($16^7$), Floor ($16^8$).

#### [NEW] [crates/mfas-cli/src/commands/page.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/page.rs)
CLI subcommand `mfas page`:
```bash
mfas page inspect <address-or-file> [--index N] [--hierarchy]
```
Displays page boundary alignment and Radix-16 spatial coordinate breakdown (`Floor.Room.Wall.Shelf.Volume.Page + Offset`).

---

### Phase 5: Recursive Node Model

Implement typed recursive nodes.

#### [NEW] [crates/mfas-core/src/node/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/node/mod.rs)
- `Node` enum:
  - `Data(Vec<u8>)`
  - `Reference(Address)`
  - `Sequence(Vec<Address>)`
  - `Repeat { target: Address, count: u64 }`
  - `Slice { target: Address, offset: u64, length: u64 }`
- Serialization / Deserialization.

---

### Phase 6: Recursive Resolver & Safety Limits

Deterministic reconstruction of bytes from DAG nodes with strict resource limits.

#### [NEW] [crates/mfas-core/src/resolver/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/resolver/mod.rs)
- `Limits`: max recursion depth, max node count, max memory allocation, max output bytes.
- Cycle detection (visited set / path tracking) preventing infinite loops.
- `resolve(address, store, limits) -> Result<Vec<u8>, ResolveError>`
- `resolve_to_writer(address, store, limits, writer) -> Result<u64, ResolveError>` for streaming reconstruction.

---

### Phase 7: Canonical Encoder / Decoder & `mfas encode/decode/verify`

Streaming canonicalization from files into page trees/nodes and back.

#### [NEW] [crates/mfas-core/src/encode/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/encode/mod.rs)
- Stream reader partitioning input into 4096-byte pages.
- Constructs deterministic DAG / Sequence node.
- Returns root canonical `Address`.

#### [NEW] [crates/mfas-core/src/decode/mod.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-core/src/decode/mod.rs)
- Streams decoded pages directly to output writer/file without allocating full output in RAM.

#### [NEW] [crates/mfas-cli/src/commands/codec.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/codec.rs)
CLI subcommands:
```bash
mfas encode <input_file> [--out-address]
mfas decode <address> <output_file>
mfas verify <file>
```
`mfas verify` encodes the file, decodes it into a temporary streaming verifier, and checks both byte-for-byte equality and SHA-256 match.

---

### Phase 8: Inspection, Diagnostics & Large Data Rebuilder

Interactive terminal diagnostics and large dataset stress testing.

#### [NEW] [crates/mfas-cli/src/commands/inspect.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/inspect.rs)
CLI command: `mfas inspect <address>`
Outputs the formatted summary table and ASCII DAG hierarchy:
```text
MFAS Address
────────────────────────────
Address:       mfas:v1:seq:...
Type:          Sequence
Size:          2,147,483,648 bytes
Pages:         524,288
Depth:         6
References:    ...
Nodes:         ...
```

#### [NEW] [crates/mfas-cli/src/commands/generate.rs](file:///d:/Development/TAURI/MFAS/crates/mfas-cli/src/commands/generate.rs)
CLI command: `mfas generate <address> <output_path>`
Reconstructs large files (100MB, 1GB+) with streaming buffer and progress meter.

---

## Verification Plan

### Automated Tests
Run tests using `cargo test` (adhering strictly to the rule: **never run `cargo check`**):

1. **Unit Tests**:
   - `cargo test -p mfas-core address`: Roundtrip URI parsing and canonical formatting invariants.
   - `cargo test -p mfas-core enumeration`: Bijective mapping $N \leftrightarrow B^*$ boundary tests (`""`, `0x00`, `0x01`, `0x0001`, `0x0100`, random multi-kilobyte strings).
   - `cargo test -p mfas-core page`: Page boundary conditions ($0$, $1$, $4095$, $4096$, $4097$ bytes, partial final page length).
   - `cargo test -p mfas-core resolver`: Cycle rejection, depth limit exceeded, streaming reconstruction equality.
2. **CLI Integration Tests**:
   - `cargo test -p mfas-cli`: Test CLI arguments, exit codes, `--json` serialization correctness, and human-readable table outputs.
3. **End-to-End File Verification**:
   - Automated test suite encoding and verifying sample files across test vectors:
     - Empty file ($0$ bytes)
     - Single byte ($1$ byte)
     - Exactly $4096$ bytes
     - $4097$ bytes (page boundary + 1)
     - Repetitive data (compressible via repeat node)
     - High-entropy pseudorandom binary block
     - SHA-256 hash identity check on all reconstructions.

### Manual Verification
Execute CLI commands manually via terminal:
- `cargo run -p mfas-cli -- address parse mfas:v1:data:48656c6c6f`
- `cargo run -p mfas-cli -- number encode "hello"`
- Verify `--json` outputs with jq or pipe to verify scriptability.
