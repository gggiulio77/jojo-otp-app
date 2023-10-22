use load_dotenv::load_dotenv;
use log::info;
use reqwest::Response;
use tauri::Window;

load_dotenv!();

const DEVICE_DOMAIN: &'static str = env!("DEVICE_DOMAIN");

#[tauri::command]
pub async fn scan() -> Result<jojo_common::otp::ScanResponse, String> {
    info!("[command:scan]: making request");

    let result = match reqwest::Client::new()
        .get(format!("{DEVICE_DOMAIN}/scan"))
        .send()
        .await
    {
        Ok(result) => result,
        Err(_) => return Err("[command:scan]: could not send request".to_string()),
    };

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
pub async fn mock_scan() -> Result<jojo_common::otp::ScanResponse, String> {
    info!("[command:scan]: making request");

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
pub async fn save_credentials(
    _window: Window,
    ssid: jojo_common::network::Ssid,
    password: jojo_common::network::Password,
) -> Result<(), String> {
    info!("[command:save_credentials]: {:?}:{:?}", ssid, password);

    let result = match reqwest::Client::new()
        .post(format!("{DEVICE_DOMAIN}/save_credentials"))
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
pub async fn mock_save_credentials(
    _window: Window,
    ssid: jojo_common::network::Ssid,
    password: jojo_common::network::Password,
) -> Result<(), String> {
    info!("[command:save_credentials]: {:?}:{:?}", ssid, password);

    let mock_response = http::response::Builder::new().status(200).body("").unwrap();

    let result = Response::from(mock_response);

    match result.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => Err(format!(
            "[command:save_credentials]: request failed, {:?}",
            result.json::<serde_json::Value>().await
        )),
    }
}
