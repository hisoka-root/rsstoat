use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::AppHandle;
use tauri::Manager;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowState {
    #[serde(default)]
    pub x: i32,
    #[serde(default)]
    pub y: i32,
    #[serde(default)]
    pub width: i32,
    #[serde(default)]
    pub height: i32,
    #[serde(default)]
    pub is_maximised: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 1280,
            height: 720,
            is_maximised: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_true")]
    pub first_launch: bool,
    #[serde(default = "default_true")]
    pub custom_frame: bool,
    #[serde(default = "default_true")]
    pub minimise_to_tray: bool,
    #[serde(default)]
    pub start_minimised_to_tray: bool,
    #[serde(default = "default_true")]
    pub spellchecker: bool,
    #[serde(default = "default_true")]
    pub hardware_acceleration: bool,
    #[serde(default = "default_true")]
    pub discord_rpc: bool,
    #[serde(default)]
    pub window_state: WindowState,
}

fn default_true() -> bool {
    true
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            first_launch: true,
            custom_frame: true,
            minimise_to_tray: true,
            start_minimised_to_tray: false,
            spellchecker: true,
            hardware_acceleration: true,
            discord_rpc: true,
            window_state: WindowState::default(),
        }
    }
}

pub type ConfigState = Arc<RwLock<AppConfig>>;

pub fn config_path(app: &AppHandle) -> PathBuf {
    let config_dir = app
        .path()
        .app_config_dir()
        .expect("failed to get config dir");
    config_dir.join("config.json")
}

pub async fn load_config(app: &AppHandle) -> AppConfig {
    let path = config_path(app);
    if let Ok(data) = fs::read_to_string(&path) {
        if let Ok(config) = serde_json::from_str(&data) {
            return config;
        }
    }
    AppConfig::default()
}

pub async fn save_config(app: &AppHandle, config: &AppConfig) -> Result<(), String> {
    let path = config_path(app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, &data).map_err(|e| e.to_string())?;
    Ok(())
}
