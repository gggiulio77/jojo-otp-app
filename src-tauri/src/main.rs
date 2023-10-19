// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use load_dotenv::load_dotenv;
use log::*;
use reqwest::Response;
use tauri::Window;

load_dotenv!();

const NETWORK_SSID_FILTER: &'static str = env!("NETWORK_SSID_FILTER");
const NETWORK_PASSWORD: &'static str = env!("NETWORK_PASSWORD");
const MOUSE_DOMAIN: &'static str = env!("MOUSE_DOMAIN");

#[tauri::command]
async fn scan() -> Result<jojo_common::otp::ScanResponse, String> {
    info!("[command:scan]: making request");

    // let result = match reqwest::Client::new()
    //     .get(format!("{MOUSE_DOMAIN}/scan"))
    //     .send()
    //     .await
    // {
    //     Ok(result) => result,
    //     Err(_) => return Err("[command:scan]: could not send request".to_string()),
    // };

    let mock_response = http::response::Builder::new()
        .status(200)
        .body(r#"{"found_ssid": ["Fibertel WiFi017 2.4GHz","Personal Wifi Zone"]}"#)
        .unwrap();

    let result = Response::from(mock_response);

    match result.status() {
        reqwest::StatusCode::OK => {
            let response = result
                .json()
                .await
                .expect("[command:scan]: could not serialize json");

            Ok(response)
        }
        _ => Err(format!(
            "[command:scan]: request failed, {:?}",
            result.json::<serde_json::Value>().await
        )),
    }
}

#[tauri::command]
async fn save_credentials(
    _window: Window,
    ssid: jojo_common::network::Ssid,
    password: jojo_common::network::Password,
) -> Result<(), String> {
    info!("[command:save_credentials]: {:?}:{:?}", ssid, password);

    let result = match reqwest::Client::new()
        .post(format!("{MOUSE_DOMAIN}/save_credentials"))
        .json(&jojo_common::network::NetworkCredentials::new(
            ssid, password,
        ))
        .send()
        .await
    {
        Ok(response) => response,
        Err(_err) => return Err("[command:save_credentials]: could not send request".to_string()),
    };

    match result.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(format!(
            "[command:save_credentials]: request failed, {:?}",
            result.json::<serde_json::Value>().await
        )),
    }
}

#[tauri::command]
async fn scan_client() -> Result<jojo_common::otp::ScanResponse, String> {
    info!("[command:scan_client]: looking for client network");

    let networks = wifi_manager::scan().expect("Something went wrong while scanning networks");

    let filtered_network: Vec<wifi_manager::Network> = networks
        .into_iter()
        .filter(|net| net.ssid.contains(NETWORK_SSID_FILTER))
        .collect();

    Ok(jojo_common::otp::ScanResponse::new(
        filtered_network
            .into_iter()
            .map(|net| net.ssid.try_into().unwrap())
            .collect(),
    ))
}

#[tauri::command]
async fn connect_client(ssid: String) -> Result<(), String> {
    info!("[command:connect_client]: connecting to client network");

    wifi_manager::connect(ssid, NETWORK_PASSWORD.to_string())
        .expect("Something went wrong while connecting to client");

    Ok(())
}

#[tauri::command]
fn test_event(window: Window) {
    window.emit_and_trigger("event-name", "Event Gay").unwrap();
    info!("Event emitted");
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            save_credentials,
            scan,
            scan_client,
            connect_client,
            test_event
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
