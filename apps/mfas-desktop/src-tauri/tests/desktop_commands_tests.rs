use mfas_desktop_lib::commands::address::{address_to_number, format_address, number_to_address, parse_address};
use mfas_desktop_lib::commands::gpu::{get_gpu_info, run_evaluator_benchmark};
use mfas_desktop_lib::commands::page::get_radix_coordinates;

#[test]
fn test_desktop_address_commands() {
    // Parse URI
    let res = parse_address("mfas:v1:data:48656c6c6f".to_string()).expect("Should parse");
    assert_eq!(res.version, 1);
    assert_eq!(res.node_type_name, "Data");
    assert_eq!(res.payload_len, 5);
    assert!(res.is_valid);

    // Parse compact hex
    let res_hex = parse_address("010148656c6c6f".to_string()).expect("Should parse hex");
    assert_eq!(res_hex.node_type_name, "Data");
    assert_eq!(res_hex.payload_len, 5);

    // Format
    let formatted = format_address(1, "48656c6c6f".to_string()).expect("Should format");
    assert_eq!(formatted, "mfas:v1:data:48656c6c6f");

    // Bijection number roundtrip
    let num_str = "1751477356".to_string();
    let addr = number_to_address(num_str.clone()).expect("Should convert to address");
    let restored_num = address_to_number(addr).expect("Should convert back to number");
    assert_eq!(restored_num, num_str);
}

#[test]
fn test_desktop_page_commands() {
    // Byte offset test
    let res1 = get_radix_coordinates(65536, false).expect("Should compute");
    assert_eq!(res1.page_index, 16);
    assert_eq!(res1.volume, 1);
    assert_eq!(res1.page, 0);

    // Page index test
    let res2 = get_radix_coordinates(16, true).expect("Should compute");
    assert_eq!(res2.byte_offset, 65536);
    assert_eq!(res2.volume, 1);
}

#[test]
fn test_desktop_gpu_commands() {
    let info = get_gpu_info().expect("Should get GPU info");
    assert!(info.cpu_evaluator_ready);
    assert!(info.simd_evaluator_ready);

    let bench_cpu = run_evaluator_benchmark("cpu".to_string(), 1024 * 1024).expect("CPU bench");
    assert_eq!(bench_cpu.bytes_evaluated, 1024 * 1024);
    assert!(bench_cpu.throughput_mbps > 0.0);

    let bench_simd = run_evaluator_benchmark("simd".to_string(), 1024 * 1024).expect("SIMD bench");
    assert_eq!(bench_simd.bytes_evaluated, 1024 * 1024);
    assert!(bench_simd.throughput_mbps > 0.0);
}
