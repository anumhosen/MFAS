# Build MFAS — Mathematical File Address Space

Build a new project called **MFAS (Mathematical File Address Space)**.

MFAS is a mathematical addressing and reconstruction system for finite digital data. The long-term goal is to create a universal address space in which arbitrary finite byte sequences can be represented by addresses and reconstructed exactly.

This is **not primarily a compression application**.

The core goals are:

* mathematically clean addressing
* exact reversible representation
* hexadecimal/base-16 alignment
* fixed logical pages
* recursive page/address references
* deterministic reconstruction
* very large file generation/reconstruction
* future CPU/GPU evaluation
* eventually a Tauri desktop explorer

The implementation must be modular and developed **step by step**.

---

# 1. IMPORTANT DEVELOPMENT RULES

Follow these rules throughout development:

1. Do NOT try to implement the entire application at once.
2. Complete one phase before moving to the next.
3. Keep the mathematical core independent from Tauri.
4. Keep CLI independent from Tauri.
5. Do not put mathematical logic inside the frontend.
6. Do not prematurely implement GPU support.
7. Do not add compression algorithms unless explicitly requested.
8. Do not use lossy transformations.
9. Every transformation must remain exactly reversible where it is part of the canonical representation.
10. Prefer simple, explicit mathematical models over clever abstractions.
11. Write tests for every mathematical invariant.
12. Never silently change the addressing model because of implementation convenience.
13. Preserve compatibility of the address format once formally defined.
14. Do not delete existing files or rewrite working modules unnecessarily.
15. If an architectural change becomes necessary, explain it and isolate it.
16. Keep modules small and focused.
17. Avoid giant files.
18. The Rust core must be usable without the GUI.
19. The CLI must be able to test almost everything in the core.
20. The first milestone is mathematical correctness, not visual polish.

---

# 2. TECHNOLOGY

Use:

## Core

* Rust
* Cargo workspace
* Rust standard library wherever practical

## CLI

* Rust
* `clap` for command-line parsing
* structured output where useful

## Desktop application — later phase

* Tauri
* TypeScript
* modern frontend
* Tailwind CSS if a CSS framework is useful

Do NOT use React unless it provides a clear benefit. The desktop UI should remain lightweight.

## GPU — future phase

Design interfaces so GPU execution can later be added without rewriting the mathematical core.

Do not implement CUDA/WGPU/GPU execution in the initial phases.

---

# 3. HIGH-LEVEL ARCHITECTURE

Use this architecture:

```text
MFAS
│
├── crates/
│   │
│   ├── mfas-core/
│   │   ├── address
│   │   ├── page
│   │   ├── node
│   │   ├── canonical
│   │   ├── encode
│   │   ├── decode
│   │   ├── resolver
│   │   ├── evaluator
│   │   ├── limits
│   │   └── io
│   │
│   ├── mfas-cli/
│   │   └── CLI application
│   │
│   └── mfas-gpu/
│       └── future GPU backend
│
├── apps/
│   │
│   └── mfas-desktop/
│       ├── frontend
│       └── src-tauri
│
├── tests/
│   ├── address
│   ├── encoding
│   ├── pages
│   ├── recursion
│   ├── roundtrip
│   └── large-data
│
└── docs/
```

The dependency direction must be:

```text
mfas-core
    ↑
    │
mfas-cli

mfas-core
    ↑
    │
mfas-desktop

mfas-core
    ↑
    │
mfas-gpu
```

`mfas-core` must never depend on the desktop application.

---

# 4. MATHEMATICAL MODEL

MFAS represents finite byte sequences.

Let:

```text
B = {0,1,...,255}
```

A file is a finite sequence:

```text
b0,b1,...,b(n-1)
```

Therefore the domain is:

```text
B*
```

the set of all finite byte strings.

The system must ultimately provide a reversible mapping:

```text
encode(data) -> Address
decode(Address) -> data
```

with the fundamental invariant:

```text
decode(encode(data)) == data
```

for every finite byte sequence supported by the implementation.

Do not claim that this automatically provides compression.

A random 2 GB file generally cannot have a tiny mathematical description.

MFAS is about **addressability and deterministic reconstruction**, not magical information compression.

---

# 5. BASE-16 DESIGN

MFAS uses hexadecimal as its primary address representation.

Important relationships:

```text
1 hexadecimal digit = 4 bits

2 hexadecimal digits = 1 byte

16^2 = 256

16^3 = 4096
```

Therefore the primary logical page size is:

```text
4096 bytes
```

which corresponds to:

```text
8192 hexadecimal digits
```

This is intentional.

4096 bytes is:

```text
2^12 bytes
16^3 bytes
```

and therefore aligns naturally with the hexadecimal radix.

---

# 6. LOGICAL PAGE MODEL

Define:

```text
PAGE_SIZE = 4096 bytes
```

A page is a logical unit.

Do not assume every page must physically occupy 4096 bytes on disk.

A page may represent:

* raw data
* references
* a sequence of references
* repeated references
* transformations
* metadata
* future executable mathematical expressions

Separate:

```text
logical page
```

from:

```text
physical storage representation
```

This distinction is important.

---

# 7. RADIX-16 HIERARCHY

Use a mathematically aligned hierarchy.

Starting from:

```text
Page = 16^3 bytes
```

we can define:

```text
Volume = 16 pages
       = 16^4 bytes

Shelf = 16 volumes
      = 16^5 bytes
      = 1 MiB

Wall = 16 shelves
     = 16^6 bytes
     = 16 MiB

Room = 16 walls
     = 16^7 bytes
     = 256 MiB

Floor = 16 rooms
      = 16^8 bytes
      = 4 GiB
```

Keep these relationships explicit in the implementation.

Do not hard-code arbitrary decimal sizes where hexadecimal/radix relationships can be expressed mathematically.

---

# 8. ADDRESS MODEL

Create a strongly typed Rust address representation.

Do NOT represent every address internally as a plain String.

For example:

```rust
pub struct Address {
    ...
}
```

The internal representation should be numeric/compact.

Provide:

```text
Address -> hexadecimal string
hexadecimal string -> Address
```

with strict validation.

The address system must support:

* canonical formatting
* parsing
* comparison
* ordering
* serialization
* validation

Avoid unnecessary leading zero ambiguity unless the address format explicitly requires fixed-width fields.

Define canonical formatting rules and document them.

---

# 9. ADDRESS SPACE VS PHYSICAL STORAGE

Keep these concepts separate:

```text
Address
    ↓
logical object
    ↓
physical representation
```

An address must identify a mathematical/logical object.

It should not inherently mean:

```text
D:\some\file\somewhere
```

or:

```text
SQLite row
```

Storage backends can be added later.

Possible future backends:

```text
memory
filesystem
SQLite
database
content-addressed storage
remote storage
```

The core mathematical model must not depend on one backend.

---

# 10. NODE MODEL

MFAS should support recursive objects.

Create a strongly typed node system.

Initial conceptual node types:

```rust
pub enum Node {
    Data(...),
    Reference(...),
    Sequence(...),
    Repeat(...),
    Slice(...),
}
```

Possible semantics:

### Data

Contains literal bytes.

```text
Data(bytes)
```

### Reference

Points to another address.

```text
Reference(address)
```

### Sequence

Concatenates multiple child nodes.

```text
Sequence([
    address1,
    address2,
    address3
])
```

### Repeat

Repeats another node.

```text
Repeat {
    address,
    count
}
```

### Slice

Represents a range of another node.

```text
Slice {
    address,
    offset,
    length
}
```

These are the initial primitives.

Do not add dozens of operations prematurely.

---

# 11. RECURSIVE RESOLUTION

The core operation is:

```text
resolve(address)
```

Conceptually:

```text
Address
   ↓
Node
   ↓
Reference
   ↓
Node
   ↓
Sequence
   ├── Node
   ├── Node
   └── Node
        ↓
      Data
```

The resolver must recursively evaluate the graph until concrete bytes can be produced.

Support:

```text
resolve(address) -> bytes
```

and preferably:

```text
resolve_to_writer(address, writer)
```

for large files.

Do NOT require a 2 GB or 10 GB reconstruction to exist completely in RAM.

---

# 12. DAG MODEL

Although the conceptual structure is recursive, do not assume it is always a tree.

Multiple nodes may reference the same node.

Therefore MFAS should support a DAG:

```text
        Root
       /    \
      A      B
       \    /
        C
```

C should be evaluated once when possible and reused.

Implement memoization/caching where appropriate.

Do not allow cycles in the initial implementation.

A cycle such as:

```text
A -> B -> C -> A
```

must be detected and rejected.

---

# 13. SAFETY LIMITS

Recursive reconstruction must have explicit limits.

Create a limits configuration containing at least:

```text
maximum recursion depth
maximum output bytes
maximum node count
maximum reference count
maximum evaluation time
```

Never allow malformed data to cause uncontrolled recursion or unbounded allocation.

Return structured errors.

Do not panic for normal malformed input.

---

# 14. CANONICALIZATION

MFAS should have a canonicalization stage.

Conceptually:

```text
ANY FILE
   ↓
Canonicalizer
   ↓
Canonical byte stream
   ↓
4096-byte logical pages
   ↓
MFAS representation
   ↓
root address
```

Reconstruction:

```text
root address
   ↓
MFAS graph
   ↓
pages
   ↓
canonical stream
   ↓
original file
```

The canonicalization process must be lossless.

The core invariant remains:

```text
decode(encode(file)) == file
```

Do not perform transformations merely because they make data "look better".

The canonical representation must be deterministic.

The same input must produce the same canonical representation under the same version of the specification.

---

# 15. PARTIAL FINAL PAGE

A file does not necessarily end exactly on a 4096-byte boundary.

Do not solve this by blindly adding meaningless bytes and forgetting how many were added.

Represent the logical length explicitly.

For example:

```text
Page {
    data
    logical_length
}
```

or an equivalent mathematically clean mechanism.

The reconstructed file must contain exactly the original number of bytes.

---

# 16. ENCODING

Implement:

```text
encode(bytes) -> Address
```

The initial implementation should favor correctness and determinism over sophisticated optimization.

For an arbitrary file:

```text
file
 ↓
canonical byte sequence
 ↓
pages
 ↓
nodes
 ↓
root
 ↓
address
```

The resulting address must uniquely identify the representation.

Do not attempt to find the shortest possible description yet.

That is a separate future research problem.

---

# 17. DECODING

Implement:

```text
decode(address) -> bytes
```

and:

```text
decode_to_file(address, path)
```

For large files use streaming/page-wise reconstruction.

Do not allocate the entire output when unnecessary.

---

# 18. STREAMING API

The core should support:

```rust
resolve_to_writer(
    address,
    writer
)
```

Conceptually:

```text
Address
 ↓
Resolver
 ↓
Page
 ↓
4096-byte buffer
 ↓
Writer
 ↓
disk/file/socket/etc.
```

This allows MFAS to eventually reconstruct:

```text
100 MB
1 GB
10 GB
100 GB
```

without requiring equivalent RAM.

---

# 19. CLI

Create a useful CLI around the core.

Initial commands:

```bash
mfas encode <file>
mfas decode <address> <output>
mfas inspect <address>
mfas page <address>
mfas verify <file>
mfas generate <address> <output>
mfas address <number>
mfas number <address>
```

Example:

```bash
mfas encode image.jpg
```

should print the canonical MFAS address.

Example:

```bash
mfas decode <address> restored.jpg
```

should reconstruct the file.

Example:

```bash
mfas verify image.jpg
```

should encode and reconstruct the data and verify equality.

Use SHA-256 for practical verification.

---

# 20. INSPECT COMMAND

Make `inspect` one of the most useful debugging tools.

Example:

```bash
mfas inspect <address>
```

should eventually display information such as:

```text
MFAS Address
────────────────────────────
Address:       ...
Type:          Sequence
Size:          2,147,483,648 bytes
Pages:         524,288
Depth:         6
References:    ...
Nodes:         ...
```

For recursive structures, provide a tree/DAG representation.

Example:

```text
ROOT
├── PAGE
├── PAGE
│   ├── REF
│   └── REF
└── PAGE
```

---

# 21. MATHEMATICAL ENUMERATION

Implement a separate experimental mapping between natural numbers and finite byte strings.

The goal is to establish:

```text
N ↔ B*
```

This is a fundamental mathematical experiment for MFAS.

Do not assume that a simple integer-to-bytes conversion is sufficient because length boundaries matter.

The encoding must distinguish:

```text
01
0001
000001
```

as different finite strings.

Document the chosen bijection clearly.

Create tests proving:

```text
decode_number(encode_number(data)) == data
```

and:

```text
encode_number(decode_number(n)) == n
```

within supported ranges.

---

# 22. PAGE ADDRESSING EXPERIMENT

Create tests around:

```text
16^3 = 4096
```

and the radix hierarchy.

Test boundaries:

```text
0
1
15
16
255
256
4095
4096
65535
65536
...
```

Verify page/volume/shelf/wall/room/floor calculations.

Avoid floating-point arithmetic for address calculations.

Use integer arithmetic.

---

# 23. TEST DATA

Do not test only English text.

Create round-trip tests for:

```text
empty file
1 byte
2 bytes
random binary
UTF-8 text
ASCII text
JPEG
PNG
ZIP
MP4 sample
large generated binary
repetitive data
high-entropy random data
```

The fundamental test is always:

```text
original
   ↓
encode
   ↓
address
   ↓
decode
   ↓
reconstructed
```

then:

```text
original == reconstructed
```

and preferably:

```text
SHA256(original) == SHA256(reconstructed)
```

---

# 24. PERFORMANCE TESTING

Do not optimize before correctness.

After correctness is established, add benchmarks for:

```text
address parsing
address formatting
page creation
page lookup
recursive resolution
streaming reconstruction
large sequential reconstruction
DAG reuse
```

Measure:

```text
throughput
latency
RAM
CPU usage
disk throughput
```

Do not claim GPU acceleration until an actual implementation and benchmark exist.

---

# 25. FUTURE GPU ARCHITECTURE

Prepare an interface for future GPU execution.

Conceptually:

```text
trait Evaluator {
    fn evaluate(...);
}
```

Potential implementations:

```text
CpuEvaluator
GpuEvaluator
```

But initially implement only:

```text
CpuEvaluator
```

The GPU backend will later be responsible for highly parallel operations such as:

```text
byte generation
page generation
mathematical transforms
independent page evaluation
```

CPU/Rust remains responsible for:

```text
address parsing
DAG traversal
dependency resolution
scheduling
I/O
streaming
```

Do not assume every recursive operation can run efficiently on a GPU.

---

# 26. IMPORTANT INFORMATION-THEORY RULE

Document this clearly in the project:

MFAS does NOT imply:

```text
tiny address → arbitrary huge file
```

for every possible file.

For arbitrary high-entropy data, the representation must contain comparable information.

MFAS instead provides:

```text
finite data
↔
mathematical address/description
```

and allows some highly structured data to have very compact descriptions.

This distinction must remain explicit throughout the project documentation.

---

# 27. Tauri APPLICATION — LATER

Do not begin with the Tauri UI.

After the Rust core and CLI are stable, create:

```text
apps/mfas-desktop
```

The desktop application should be an explorer for the MFAS system.

Potential interface:

```text
┌─────────────────────────────────────────────────────────┐
│ MFAS                                                     │
├──────────────┬──────────────────────────────────────────┤
│ Explorer     │ Address                                  │
│              │                                           │
│ Root         │ 0x......                                  │
│ ├ Page       │                                           │
│ ├ Page       │ Node Graph                               │
│ └ Page       │                                           │
│              │                                           │
├──────────────┴──────────────────────────────────────────┤
│ Size | Pages | Nodes | Depth | Backend | Performance    │
└─────────────────────────────────────────────────────────┘
```

The UI should expose the mathematical model rather than hide it.

Useful views:

* Address explorer
* Page explorer
* Node graph
* Hex viewer
* Byte viewer
* File encoder
* File decoder
* Reconstruction progress
* DAG inspector
* benchmark panel

---

# 28. DESKTOP COMMAND BOUNDARY

Tauri should communicate with Rust through a small command layer.

Example:

```text
Frontend
   ↓
Tauri command
   ↓
mfas-core
   ↓
result
   ↓
Frontend
```

Do not duplicate core logic in TypeScript.

TypeScript should handle:

```text
state
UI
interaction
visualization
formatting
```

Rust should handle:

```text
mathematics
addresses
pages
nodes
resolution
encoding
decoding
file I/O
limits
performance
```

---

# 29. STORAGE BACKEND

Do not make SQLite mandatory in the first version.

The mathematical system should work entirely in memory first.

Later provide storage implementations:

```text
MemoryStore
FileStore
SQLiteStore
ContentAddressedStore
```

The storage abstraction should not contaminate the mathematical model.

---

# 30. DOCUMENTATION

Create:

```text
docs/
├── architecture.md
├── mathematics.md
├── address-format.md
├── page-model.md
├── node-model.md
├── recursion.md
├── canonicalization.md
├── storage.md
├── gpu.md
└── roadmap.md
```

Explain the system mathematically and practically.

Include diagrams using ASCII/Markdown where appropriate.

---

# 31. DEVELOPMENT PHASES

Implement exactly in this order.

## Phase 1 — Workspace

Create:

```text
mfas-core
mfas-cli
mfas-gpu
```

Set up Cargo workspace.

No Tauri yet.

---

## Phase 2 — Address Mathematics

Implement:

```text
Address
hex parsing
hex formatting
validation
integer conversions
```

Create comprehensive tests.

---

## Phase 3 — Byte Enumeration

Implement:

```text
finite byte sequence ↔ natural number
```

for a clearly defined bounded range initially.

Prove round-trip properties.

---

## Phase 4 — Pages

Implement:

```text
4096-byte logical pages
page indexes
page boundaries
partial final pages
```

Test all boundary conditions.

---

## Phase 5 — Node Model

Implement:

```text
Data
Reference
Sequence
Repeat
Slice
```

with serialization/deserialization.

---

## Phase 6 — Resolver

Implement:

```text
resolve()
resolve_to_writer()
```

Add:

```text
cycle detection
depth limits
output limits
node limits
```

---

## Phase 7 — Encoder/Decoder

Implement:

```text
encode(file)
decode(address)
verify(file)
```

Run complete round-trip tests.

---

## Phase 8 — CLI

Build a useful CLI around every stable core feature.

The CLI should become the primary development/testing interface.

---

## Phase 9 — DAG + Caching

Add shared references and memoization.

Verify that repeated structures are evaluated efficiently.

---

## Phase 10 — Large Data

Test:

```text
100 MB
1 GB
2 GB+
```

using streaming reconstruction.

Do not require all output in memory.

---

## Phase 11 — Benchmarks

Measure CPU performance, memory, I/O, and recursive evaluation.

---

## Phase 12 — GPU Interface

Create the GPU abstraction.

Initially leave the GPU backend as a controlled experimental module.

Only implement actual GPU computation after the CPU evaluator is stable.

---

## Phase 13 — Tauri Desktop

Only now build the graphical explorer.

The GUI must consume the same `mfas-core` used by the CLI.

---

# 32. FIRST IMPLEMENTATION TARGET

Do NOT start by implementing all phases.

Start only with:

```text
Phase 1
+
Phase 2
```

After implementation, show:

1. project tree
2. files created
3. address model
4. mathematical assumptions
5. tests implemented
6. any unresolved architectural questions

Then continue to Phase 3.

---

# 33. QUALITY STANDARD

The most important property of MFAS is not UI quality.

It is this:

```text
                ┌───────────────┐
                │    DATA       │
                └───────┬───────┘
                        │
                     ENCODE
                        │
                        ▼
                ┌───────────────┐
                │    ADDRESS    │
                └───────┬───────┘
                        │
                     DECODE
                        │
                        ▼
                ┌───────────────┐
                │ RECONSTRUCTED  │
                │     DATA       │
                └───────────────┘
```

The invariant must be:

```text
decode(encode(X)) = X
```

for every supported finite byte sequence `X`.

Build MFAS around this invariant.

Do not sacrifice mathematical correctness for UI convenience, compression claims, premature GPU optimization, or unnecessary abstraction.

---

# FINAL GOAL

The eventual system should look conceptually like:

```text
                     MFAS
                      │
              Mathematical Space
                      │
              ┌───────┴───────┐
              │               │
          Address Space     Node Graph
              │               │
              └───────┬───────┘
                      │
                   Resolver
                      │
          ┌───────────┴───────────┐
          │                       │
         CPU                     GPU
          │                       │
          └───────────┬───────────┘
                      │
                  Page Stream
                      │
                      ▼
                 Any File
```

Build it as a **general mathematical data-addressing engine first**, a CLI second, GPU evaluator third, and Tauri explorer last.

Do not let the GUI define the architecture.
