// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;
mod otp;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            otp::scan,
            otp::mock_scan,
            otp::save_credentials,
            otp::mock_save_credentials,
            client::scan_client,
            client::mock_scan_client,
            client::connect_client,
            client::mock_connect_client
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
