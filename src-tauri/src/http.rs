use crate::error::BDMResult;
use crate::settings::retrieve_manager_settings;
use reqwest::header::{HeaderMap, HeaderValue, COOKIE, REFERER, USER_AGENT};
use tauri::AppHandle;

pub const REFERER_VALUE: &str = "https://www.bilibili.com/";
pub const USER_AGENT_VALUE: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36";

pub async fn get_json(
    app: AppHandle,
    url: &str,
    query: &[(&str, String)],
) -> BDMResult<serde_json::Value> {
    let manager_settings = retrieve_manager_settings(app).unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(COOKIE, manager_settings.access_token.parse().unwrap());
    headers.insert(REFERER, HeaderValue::from_static(REFERER_VALUE));
    headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let json: serde_json::Value = client.get(url).query(query).send().await?.json().await?;

    Ok(json)
}
