mod error;
mod settings;
mod storage;

use crate::settings::ManagerSettings;
use crate::storage::Storage;
use std::path::PathBuf;
use std::sync::OnceLock;
use tauri::path::BaseDirectory;
use tauri::Manager;

static APP_DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            settings::retrieve_manager_settings,
            settings::update_manager_settings
        ])
        .setup(|app| {
            APP_DATA_DIR
                .set(app.path().resolve("", BaseDirectory::AppData).unwrap())
                .expect("APP_DATA_DIR initialization failed");

            let mut app_settings: ManagerSettings = ManagerSettings::load().unwrap_or_default();
            app_settings.setup_with_app(app.handle()).unwrap();
            app_settings.save().unwrap();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
