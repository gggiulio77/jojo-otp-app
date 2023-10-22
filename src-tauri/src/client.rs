use jojo_common::network::{Password, Ssid};
use load_dotenv::load_dotenv;
use log::info;
use tauri::Window;

load_dotenv!();
const NETWORK_SSID_FILTER: &'static str = env!("NETWORK_SSID_FILTER");

#[tauri::command]
pub async fn scan_client() -> Result<jojo_common::otp::ScanResponse, String> {
    info!("[]: looking for client network");

    let networks = jojo_wifi_manager::scan().expect("Something went wrong while scanning networks");

    info!("[command:scan_client]: {:?}", networks);

    let filtered_network: Vec<jojo_wifi_manager::Network> = networks
        .into_iter()
        .filter(|net| net.ssid.contains(NETWORK_SSID_FILTER))
        .collect();

    // TODO: in case filtered_network is zero return an Error (Not found), add a retry button

    Ok(jojo_common::otp::ScanResponse::new(
        filtered_network
            .into_iter()
            .map(|net| net.ssid.try_into().unwrap())
            .collect(),
    ))
}

#[tauri::command]
pub async fn mock_scan_client() -> Result<jojo_common::otp::ScanResponse, String> {
    info!("[]: looking for client network");
    let mock_response = vec![
        "AP TEST".to_string().try_into().unwrap(),
        "AP TEST 2".to_string().try_into().unwrap(),
        "AP TEST 3".to_string().try_into().unwrap(),
        "AP TEST 4".to_string().try_into().unwrap(),
        "AP TEST 5".to_string().try_into().unwrap(),
    ];
    Ok(jojo_common::otp::ScanResponse::new(mock_response))
}

#[tauri::command]
pub async fn connect_client(ssid: String, password: String) -> Result<(), String> {
    info!("[command:connect_client]: connecting to client network");

    jojo_wifi_manager::connect(ssid, password)
        .expect("Something went wrong while connecting to client");

    Ok(())
}

#[tauri::command]
pub async fn mock_connect_client(
    _window: Window,
    ssid: Ssid,
    password: Password,
) -> Result<(), String> {
    info!(
        "[command:connect_client]: connecting to client network {:?}:{:?}",
        ssid, password
    );

    info!("[command:connect_client]: connected");

    // TODO: think about the error variant

    Ok(())
}
