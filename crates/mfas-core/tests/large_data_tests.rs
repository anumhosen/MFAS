use mfas_core::codec::verify_stream;
use mfas_core::resolver::MemoryStore;
use mfas_core::synthetic::SyntheticStream;

#[test]
fn test_large_repetitive_stream_100mb() {
    // 100 MiB of repetitive data
    let total_bytes = 100 * 1024 * 1024; // 104,857,600 bytes
    let mut stream = SyntheticStream::repeat(vec![0xAA; 4096], total_bytes / 4096);
    let mut store = MemoryStore::new();

    let report = verify_stream(&mut stream, &mut store).unwrap();
    assert!(report.verified, "100 MB repetitive stream must verify bit-for-bit");
    assert_eq!(report.total_bytes, total_bytes);

    // Assert O(1) memory / node efficiency:
    // With identical 4096-byte pages, all pages share the same address,
    // so exactly 1 page node and 1 repeat node (or sequence of 1 repeat) are created in store!
    assert!(
        store.len() <= 3,
        "Store must only contain 1-3 nodes due to on-the-fly run-length deduplication, got {}",
        store.len()
    );
}

#[test]
fn test_large_counter_stream_10mb() {
    // 10 MiB of sequential counter data (2,560 distinct pages)
    let total_bytes = 10 * 1024 * 1024; // 10,485,760 bytes
    let mut stream = SyntheticStream::counter(total_bytes);
    let mut store = MemoryStore::new();

    let report = verify_stream(&mut stream, &mut store).unwrap();
    assert!(report.verified, "10 MB counter stream must verify bit-for-bit");
    assert_eq!(report.total_bytes, total_bytes);
}

#[test]
fn test_large_zeros_stream_50mb() {
    // 50 MiB of zeros
    let total_bytes = 50 * 1024 * 1024; // 52,428,800 bytes
    let mut stream = SyntheticStream::zeros(total_bytes);
    let mut store = MemoryStore::new();

    let report = verify_stream(&mut stream, &mut store).unwrap();
    assert!(report.verified, "50 MB zeros stream must verify bit-for-bit");
    assert_eq!(report.total_bytes, total_bytes);
}
