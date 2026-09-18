# MFAS Address Format Specification (v1)

## 1. Overview

The Mathematical File Address Space (MFAS) addresses finite digital byte strings ($B^*$) through a deterministic, reversible, and mathematically clean scheme.

Addresses exist in two isomorphic forms:
1. **Canonical Text URI Representation**: A human-readable, typed, versioned string suitable for CLI arguments, configuration, logs, and user interfaces.
2. **Canonical Binary Representation**: A compact byte sequence suitable for disk storage, network serialization, and internal fast comparisons.

---

## 2. Canonical URI Format

The text representation follows a strict URI syntax:

```text
mfas:v1:<node_type>:<hex_payload>
```

### 2.1 Scheme Components

| Component | Format | Description | Examples |
| :--- | :--- | :--- | :--- |
| **Prefix** | `mfas` | Protocol identifier (strictly lower-case) | `mfas` |
| **Version** | `v1` | Address format revision tag | `v1` |
| **Node Type** | 3-4 lower-case ASCII chars or 2-digit hex | The logical node classification | `data`, `ref`, `seq`, `rep`, `slice`, `page` |
| **Payload** | Even-length lower-case hexadecimal string | The typed node descriptor or content address | `48656c6c6f`, `001a4f...` |

### 2.2 Canonical Formatting Rules

1. **Case Sensitivity**: The scheme prefix (`mfas`), version (`v1`), node type, and hex digits `[0-9a-f]` must be **strictly lower-case**. Uppercase input is rejected or normalized upon parsing.
2. **Hex Alignment**: The payload length must always be an **even number of hexadecimal digits** (1 byte = 2 hex characters). Odd-length hex strings are strictly invalid.
3. **No Redundant Prefixes**: The hex payload must not be prefixed with `0x`.
4. **Leading Zeros**: Leading zeros in fixed-size fields (e.g. hashes, page indices) are significant and must be preserved to maintain exact field widths.

---

## 3. Node Types and Payload Definitions

### 3.1 Data Node (`data` / Type ID: `0x01`)
Represents literal raw byte content directly embedded within the address (suitable for small payloads, headers, or leaves):
- **URI Format**: `mfas:v1:data:<even_hex_bytes>`
- **Example**: `mfas:v1:data:48656c6c6f` (represents ASCII string `"Hello"`)

### 3.2 Reference Node (`ref` / Type ID: `0x02`)
Points to another canonical address or content-addressed identifier (e.g., Blake3/SHA-256 digest of a resolved node):
- **URI Format**: `mfas:v1:ref:<hash_or_id_hex>`
- **Example**: `mfas:v1:ref:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`

### 3.3 Sequence Node (`seq` / Type ID: `0x03`)
Concatenates an ordered list of child node references:
- **URI Format**: `mfas:v1:seq:<child_count_hex>:<child_ref_1><child_ref_2>...`
- **Example**: `mfas:v1:seq:0002:<ref1_hex><ref2_hex>`

### 3.4 Repeat Node (`rep` / Type ID: `0x04`)
Repeats a targeted node or byte pattern $N$ times:
- **Payload**: `<repeat_count_hex_8B><child_address_hex>`
- **URI Format**: `mfas:v1:rep:<count_hex>:<target_hex>`
- **Example**: `mfas:v1:rep:0000000000000400:41` (repeats byte `0x41` 1024 times)

### 3.5 Slice Node (`slice` / Type ID: `0x05`)
Represents an offset range within another node:
- **Payload**: `<offset_hex_8B><length_hex_8B><child_address_hex>`
- **URI Format**: `mfas:v1:slice:<offset_hex>:<len_hex>:<target_hex>`

### 3.6 Page Node (`page` / Type ID: `0x06`)
Represents a canonical $16^3 = 4096$-byte logical page or a partial final page:
- **Payload**: `<page_index_hex_8B><logical_len_hex_4B><page_data_or_hash_hex>`
- **Logical length**: `0x0001` to `0x1000` (1 to 4096 bytes).
- **URI Format**: `mfas:v1:page:<index_hex>:<len_hex>:<content_hex>`

---

## 4. Radix-16 Hierarchy Coordinates

The address space maps logically into a base-16 spatial hierarchy:

| Level | Unit | Formula | Bytes | Hex Digits (Data) | Equivalent Size |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **0** | **Byte** | $16^0$ | 1 B | 2 | 1 Byte |
| **1** | **Word** | $16^1$ | 16 B | 32 | 16 Bytes |
| **2** | **Block** | $16^2$ | 256 B | 512 | 256 Bytes |
| **3** | **Page** | $16^3$ | 4,096 B | 8,192 | 4 KiB |
| **4** | **Volume** | $16^4$ | 65,536 B | 131,072 | 64 KiB (16 pages) |
| **5** | **Shelf** | $16^5$ | 1,048,576 B | 2,097,152 | 1 MiB (16 volumes) |
| **6** | **Wall** | $16^6$ | 16,777,216 B | 33,554,432 | 16 MiB (16 shelves) |
| **7** | **Room** | $16^7$ | 268,435,456 B | 536,870,912 | 256 MiB (16 walls) |
| **8** | **Floor** | $16^8$ | 4,294,967,296 B | 8,589,934,592 | 4 GiB (16 rooms) |

### 4.1 Radix-16 Coordinate Representation
Any byte offset $O \in \mathbb{N}$ can be expressed canonically as a radix-16 hierarchical coordinate:
```text
Floor[f].Room[r].Wall[w].Shelf[s].Volume[v].Page[p] + Offset[b]
```
where each coordinate $f, r, w, s, v, p \in \{0, \dots, 15\}$ corresponds to a hexadecimal nibble of the page index, and $b \in \{0, \dots, 4095\}$ is the offset within the page.

---

## 5. Binary Serialization Format

When stored on disk or transferred across low-overhead channels:

```text
┌──────────────┬──────────────┬──────────────┬────────────────────────┐
│ Magic (4B)   │ Version (1B) │ Type ID (1B) │ Length-Prefixed Payload│
│ 0x4D 46 41 53│ 0x01         │ 0x01..0x06   │ [varint_len] + [bytes] │
└──────────────┴──────────────┴──────────────┴────────────────────────┘
```
- **Magic**: `b"MFAS"` (`0x4D464153`)
- **Version**: `0x01`
- **Type ID**: `0x01` (data), `0x02` (ref), `0x03` (seq), `0x04` (rep), `0x05` (slice), `0x06` (page)
- **Length**: Unsigned LEB128 varint encoding the payload byte length
- **Payload**: Raw byte content

---

## 6. Mathematical Invariants

1. **Reversibility**:
   $$\text{parse}(\text{format}(A)) = A \quad \forall A \in \text{Address}$$
   $$\text{format}(\text{parse}(S)) = S \quad \forall S \in \text{ValidCanonicalURI}$$
2. **Determinism**:
   Given identical node specifications, the emitted canonical URI and binary representation must be byte-for-byte identical across all architectures.
3. **Ordering**:
   Addresses are lexicographically ordered by `(version, type_id, payload_bytes)` providing total ordering for indexing and search.
