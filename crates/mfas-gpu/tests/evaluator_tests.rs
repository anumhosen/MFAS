use mfas_gpu::{
    CpuEvaluator, Evaluator, EvaluatorBackend, GpuEvaluator, PageEvalRequest, SimdEvaluator,
};

#[test]
fn test_cpu_evaluator_patterns() {
    let cpu = CpuEvaluator::new();
    assert_eq!(cpu.name(), "Standard CPU Evaluator");
    assert_eq!(cpu.backend(), EvaluatorBackend::Cpu);
    assert!(cpu.is_available());

    let zeros = cpu.generate_bytes("zeros", 1024, 0).unwrap();
    assert_eq!(zeros.len(), 1024);
    assert!(zeros.iter().all(|&b| b == 0));

    let counter = cpu.generate_bytes("counter", 512, 0).unwrap();
    assert_eq!(counter.len(), 512);
    assert_eq!(counter[0], 0);
    assert_eq!(counter[1], 1);
    assert_eq!(counter[255], 255);
    assert_eq!(counter[256], 0);

    let repeat = cpu.generate_bytes("repeat", 8, 0).unwrap();
    assert_eq!(repeat, b"MFASMFAS");

    let rand1 = cpu.generate_bytes("random", 256, 12345).unwrap();
    let rand2 = cpu.generate_bytes("random", 256, 12345).unwrap();
    assert_eq!(rand1, rand2); // Deterministic
}

#[test]
fn test_simd_evaluator_equality() {
    let cpu = CpuEvaluator::new();
    let simd = SimdEvaluator::new();
    assert_eq!(simd.backend(), EvaluatorBackend::Simd);

    let cpu_zeros = cpu.generate_bytes("zeros", 4096, 0).unwrap();
    let simd_zeros = simd.generate_bytes("zeros", 4096, 0).unwrap();
    assert_eq!(cpu_zeros, simd_zeros);

    let cpu_counter = cpu.generate_bytes("counter", 4096, 0).unwrap();
    let simd_counter = simd.generate_bytes("counter", 4096, 0).unwrap();
    assert_eq!(cpu_counter, simd_counter);

    let cpu_repeat = cpu.generate_bytes("repeat", 4096, 0).unwrap();
    let simd_repeat = simd.generate_bytes("repeat", 4096, 0).unwrap();
    assert_eq!(cpu_repeat, simd_repeat);
}

#[test]
fn test_gpu_evaluator_detection_and_fallback() {
    let gpu = GpuEvaluator::new();
    let adapters = GpuEvaluator::detect_adapters();
    assert!(!adapters.is_empty());

    // GPU evaluator routes calls reliably
    let bytes = gpu.generate_bytes("counter", 100, 0).unwrap();
    assert_eq!(bytes.len(), 100);
}

#[test]
fn test_batch_page_evaluation() {
    let cpu = CpuEvaluator::new();
    let reqs = vec![
        PageEvalRequest {
            page_index: 0,
            logical_len: 4096,
            pattern: "zeros".to_string(),
            seed: 0,
        },
        PageEvalRequest {
            page_index: 1,
            logical_len: 4096,
            pattern: "counter".to_string(),
            seed: 0,
        },
        PageEvalRequest {
            page_index: 2,
            logical_len: 2048,
            pattern: "repeat".to_string(),
            seed: 0,
        },
    ];

    let results = cpu.batch_evaluate_pages(&reqs).unwrap();
    assert_eq!(results.len(), 3);
    assert_eq!(results[0].len(), 4096);
    assert_eq!(results[1].len(), 4096);
    assert_eq!(results[2].len(), 2048);
}
