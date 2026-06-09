use crate::error::BDMResult;
use crate::storage::Storage;
use crate::APP_DATA_DIR;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

pub const SETTINGS_FILE_NAME: &str = "settings.json";

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct ManagerSettings {
    pub access_token: String,
    pub download_dir: PathBuf,
}

impl Storage for ManagerSettings {
    fn file_path() -> PathBuf {
        APP_DATA_DIR.get().unwrap().join(SETTINGS_FILE_NAME)
    }
}

impl ManagerSettings {
    pub fn setup_with_app(&mut self, app: &AppHandle) -> BDMResult<()> {
        if self.download_dir.as_os_str().is_empty()
            || fs::create_dir_all(&self.download_dir).is_err()
        {
            self.download_dir = app.path().resolve("", BaseDirectory::Download)?;
            fs::create_dir_all(&self.download_dir)?;
        }

        Ok(())
    }
}

#[tauri::command]
pub fn retrieve_manager_settings(app: AppHandle) -> BDMResult<ManagerSettings> {
    let binding = app.state::<Mutex<ManagerSettings>>();
    let state = binding.lock()?;
    Ok(state.clone())
}
