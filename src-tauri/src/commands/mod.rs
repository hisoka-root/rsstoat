use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

use crate::config::{AppConfig, ConfigState};
use crate::discord_rpc::DiscordRpc;

#[derive(Serialize, Deserialize)]
pub struct Versions {
    pub tauri: String,
    pub desktop: String,
}

#[tauri::command]
pub fn get_versions() -> Versions {
    Versions {
        tauri: "2".to_string(),
        desktop: "1.4.1".to_string(),
    }
}

#[tauri::command]
pub fn minimise(window: tauri::WebviewWindow) -> Result<(), String> {
    window.minimize().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn maximise(window: tauri::WebviewWindow) -> Result<(), String> {
    if window.is_maximized().unwrap_or(false) {
        window.unmaximize().map_err(|e| e.to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn close_window(window: tauri::WebviewWindow) -> Result<(), String> {
    window.close().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_config(config: tauri::State<'_, ConfigState>) -> Result<AppConfig, String> {
    let cfg = config.read().await;
    Ok(cfg.clone())
}

#[tauri::command]
pub async fn set_config(
    app: AppHandle,
    config: tauri::State<'_, ConfigState>,
    new_config: AppConfig,
) -> Result<(), String> {
    let old_discord_rpc = {
        let cfg = config.read().await;
        cfg.discord_rpc
    };
    let new_discord_rpc = new_config.discord_rpc;

    // save config
    let cfg_snapshot;
    {
        let mut cfg = config.write().await;
        *cfg = new_config;
        cfg_snapshot = cfg.clone();
    }
    crate::config::save_config(&app, &cfg_snapshot).await?;

    // toggle discord rpc if changed
    if old_discord_rpc != new_discord_rpc {
        if let Some(rpc_mutex) = app.try_state::<std::sync::Mutex<DiscordRpc>>() {
            if new_discord_rpc {
                rpc_mutex.lock().unwrap().start();
            } else {
                rpc_mutex.lock().unwrap().stop();
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_autostart() -> Result<bool, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let launcher = auto_launch::AutoLaunchBuilder::new()
        .set_app_name("rsStoat")
        .set_app_path(&exe.to_string_lossy())
        .build()
        .map_err(|e| e.to_string())?;
    launcher.is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_autostart(enabled: bool) -> Result<bool, String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let launcher = auto_launch::AutoLaunchBuilder::new()
        .set_app_name("rsStoat")
        .set_app_path(&exe.to_string_lossy())
        .build()
        .map_err(|e| e.to_string())?;
    if enabled {
        launcher.enable().map_err(|e| e.to_string())?;
    } else {
        launcher.disable().map_err(|e| e.to_string())?;
    }
    launcher.is_enabled().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_badge_count(window: tauri::WebviewWindow, count: i32) -> Result<(), String> {
    crate::badges::set_badge_count(&window, count);
    Ok(())
}

#[tauri::command]
pub async fn quit_app(app: AppHandle) -> Result<(), String> {
    app.exit(0);
    Ok(())
}

#[tauri::command]
pub fn is_window_visible(window: tauri::WebviewWindow) -> Result<bool, String> {
    window.is_visible().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn show_window(window: tauri::WebviewWindow) -> Result<(), String> {
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn hide_window(window: tauri::WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn check_update(app: AppHandle) -> Result<String, String> {
    let updater = app
        .updater()
        .map_err(|e| format!("Updater not available: {}", e))?;
    match updater.check().await {
        Ok(Some(update)) => Ok(format!(
            "Update available: {} -> {}",
            update.current_version, update.version
        )),
        Ok(None) => Ok("No update available".into()),
        Err(e) => Err(format!("Update check failed: {}", e)),
    }
}

#[tauri::command]
pub async fn open_devtools(window: tauri::WebviewWindow) -> Result<(), String> {
    window.open_devtools();
    Ok(())
}
