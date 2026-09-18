pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Window controls
            commands::window::minimize_window,
            commands::window::toggle_maximize_window,
            commands::window::is_window_maximized,
            commands::window::close_window,
            // Address commands
            commands::address::parse_address,
            commands::address::format_address,
            commands::address::number_to_address,
            commands::address::address_to_number,
            // Page commands
            commands::page::get_radix_coordinates,
            commands::page::inspect_page_data,
            // DAG commands
            commands::dag::analyze_dag,
            commands::dag::get_dag_dot,
            commands::dag::optimize_dag,
            // Codec commands
            commands::codec::encode_file,
            commands::codec::decode_file,
            commands::codec::verify_file,
            // GPU & Benchmark commands
            commands::gpu::get_gpu_info,
            commands::gpu::run_evaluator_benchmark,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
