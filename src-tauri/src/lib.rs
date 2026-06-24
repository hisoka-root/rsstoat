mod badges;
mod commands;
mod config;
mod discord_rpc;
mod tray;

use config::ConfigState;
use discord_rpc::DiscordRpc;
use std::time::Instant;
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            let handle = app.handle().clone();

            let build_url = std::env::args()
                .find(|a| a.starts_with("--force-server="))
                .map(|a| a.trim_start_matches("--force-server=").to_string())
                .unwrap_or_else(|| "https://stoat.chat/app".to_string());

            let start_hidden = std::env::args().any(|a| a == "--hidden");

            // load config
            let cfg = tauri::async_runtime::block_on(config::load_config(&handle));
            let config_state: ConfigState = std::sync::Arc::new(tokio::sync::RwLock::new(cfg));
            app.manage(config_state.clone());

            // discord rpc state
            let discord_rpc = DiscordRpc::new();
            app.manage(std::sync::Mutex::new(discord_rpc));

            // enable auto-launch on first launch
            {
                let first = tauri::async_runtime::block_on(config_state.read()).first_launch;
                if first {
                    let _ = try_enable_autostart(&handle);
                }
            }
            if cfg!(target_os = "windows") || cfg!(target_os = "macos") {
                let snapshot = {
                    let mut cfg = tauri::async_runtime::block_on(config_state.write());
                    if cfg.first_launch {
                        cfg.first_launch = false;
                        Some(cfg.clone())
                    } else {
                        None
                    }
                };
                if let Some(cfg) = snapshot {
                    let _ = tauri::async_runtime::block_on(config::save_config(&handle, &cfg));
                }
            }

            // init discord rpc if enabled
            {
                let discord_enabled =
                    tauri::async_runtime::block_on(config_state.read()).discord_rpc;
                if discord_enabled {
                    if let Some(rpc_mutex) = app.try_state::<std::sync::Mutex<DiscordRpc>>() {
                        rpc_mutex.lock().unwrap().start();
                    }
                }
            }

            // create main webview window
            let window = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External(build_url.parse().expect("invalid build URL")),
            )
            .title("rsStoat")
            .inner_size(1280.0, 720.0)
            .min_inner_size(300.0, 300.0)
            .resizable(true)
            .visible(!start_hidden)
            .initialization_script(include_str!("../bridge.js"))
            .build()
            .expect("failed to create main window");
            let _ = window.show();

            // restore window state
            {
                let cfg = tauri::async_runtime::block_on(config_state.read());
                let ws = &cfg.window_state;
                if ws.width > 0 && ws.height > 0 {
                    let _ =
                        window.set_size(tauri::LogicalSize::new(ws.width as f64, ws.height as f64));
                }
                if ws.x > 0 || ws.y > 0 {
                    let _ =
                        window.set_position(tauri::LogicalPosition::new(ws.x as f64, ws.y as f64));
                }
                if ws.is_maximised {
                    let _ = window.maximize();
                }
            }

            // save window state on events (throttled to 300ms)
            let cs = config_state.clone();
            let app_h = handle.clone();
            let last_save = std::sync::Mutex::new(Instant::now());
            window.on_window_event(move |event| {
                use tauri::WindowEvent;
                let cs = cs.clone();
                let app_h = app_h.clone();
                let should_save = || {
                    let mut last = last_save.lock().unwrap();
                    let now = Instant::now();
                    if now.duration_since(*last).as_millis() >= 300 {
                        *last = now;
                        true
                    } else {
                        false
                    }
                };
                match event {
                    WindowEvent::Moved(pos) => {
                        if !should_save() {
                            return;
                        }
                        let x = pos.x;
                        let y = pos.y;
                        tauri::async_runtime::spawn(async move {
                            let snapshot;
                            {
                                let mut cfg = cs.write().await;
                                cfg.window_state.x = x;
                                cfg.window_state.y = y;
                                snapshot = cfg.clone();
                            }
                            let _ = config::save_config(&app_h, &snapshot).await;
                        });
                    }
                    WindowEvent::Resized(size) => {
                        if !should_save() {
                            return;
                        }
                        let w = size.width as i32;
                        let h = size.height as i32;
                        tauri::async_runtime::spawn(async move {
                            let snapshot;
                            {
                                let mut cfg = cs.write().await;
                                cfg.window_state.width = w;
                                cfg.window_state.height = h;
                                snapshot = cfg.clone();
                            }
                            let _ = config::save_config(&app_h, &snapshot).await;
                        });
                    }
                    _ => {}
                }
            });

            // intercept close -> minimize to tray
            let cs2 = config_state.clone();
            let w2 = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                    let cfg = tauri::async_runtime::block_on(cs2.read());
                    if cfg.minimise_to_tray {
                        api.prevent_close();
                        let _ = w2.hide();
                    }
                }
            });

            // init tray
            let _ = tray::init_tray(&handle);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_versions,
            commands::minimise,
            commands::maximise,
            commands::close_window,
            commands::get_config,
            commands::set_config,
            commands::get_autostart,
            commands::set_autostart,
            commands::set_badge_count,
            commands::quit_app,
            commands::is_window_visible,
            commands::show_window,
            commands::hide_window,
        ])
        .run(tauri::generate_context!())
        .expect("error while running rsStoat");
}

fn try_enable_autostart(_app: &tauri::AppHandle) -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let launcher = auto_launch::AutoLaunchBuilder::new()
        .set_app_name("rsStoat")
        .set_app_path(&exe.to_string_lossy())
        .build()
        .map_err(|e| e.to_string())?;
    launcher.enable().map_err(|e| e.to_string())
}
