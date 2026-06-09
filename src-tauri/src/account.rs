use crate::http::get_json;
use crate::settings::ManagerSettings;
use crate::{error::BDMResult, storage::Storage};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

#[derive(Deserialize, Serialize)]
pub struct GenerateResponse {
    url: String,
    qrcode_key: String,
}

#[derive(Deserialize, Serialize)]
pub struct PollResponse {
    url: String,
    refresh_token: String,
    timestamp: u64,
    code: i64,
    message: String,
}

#[tauri::command]
pub async fn generate_qrcode(app: AppHandle) -> BDMResult<GenerateResponse> {
    let url = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
    let json = get_json(app, url, &[]).await?;

    Ok(GenerateResponse {
        url: json["data"]["url"].to_string(),
        qrcode_key: json["data"]["qrcode_key"].to_string(),
    })
}

#[tauri::command]
pub async fn poll_qrcode(app: AppHandle, qrcode_key: String) -> BDMResult<PollResponse> {
    let url = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll";
    let json = get_json(app, url, &[("qrcode_key", qrcode_key)]).await?;

    Ok(PollResponse {
        url: json["data"]["url"].to_string(),
        refresh_token: json["data"]["refresh_token"].to_string(),
        timestamp: json["data"]["timestamp"].as_u64().unwrap(),
        code: json["data"]["code"].as_i64().unwrap(),
        message: json["data"]["message"].to_string(),
    })
}

#[tauri::command]
pub async fn check_login(app: AppHandle) -> BDMResult<bool> {
    let url = "https://passport.bilibili.com/x/passport-login/web/cookie/info";
    let json = get_json(app, url, &[]).await?;

    if json["code"] != 0 || json["data"]["refresh"].as_bool().unwrap() {
        Ok(false)
    } else {
        Ok(true)
    }
}

#[tauri::command]
pub async fn login(app: AppHandle, cookie: String) -> BDMResult<()> {
    let binding = app.state::<Mutex<ManagerSettings>>();
    let mut state = binding.lock()?;
    state.access_token = cookie;
    state.save()?;
    Ok(())
}

#[tauri::command]
pub async fn logout(app: AppHandle) -> BDMResult<()> {
    let binding = app.state::<Mutex<ManagerSettings>>();
    let mut state = binding.lock()?;
    state.access_token = "".to_string();
    state.save()?;
    Ok(())
}
