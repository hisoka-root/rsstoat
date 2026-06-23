use std::sync::Mutex;
use tauri::image::Image;
use tauri::AppHandle;
use tauri::Manager;

pub struct TrayState {
    pub show_hide_item: tauri::menu::MenuItem<tauri::Wry>,
}

pub fn init_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    use tauri::menu::{MenuBuilder, MenuItemBuilder};
    use tauri::tray::TrayIconBuilder;

    let show_hide = MenuItemBuilder::with_id("show_hide", "Hide App").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit App").build(app)?;
    let title_item = MenuItemBuilder::with_id("title", "rsStoat for Desktop")
        .enabled(false)
        .build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&title_item)
        .separator()
        .item(&show_hide)
        .separator()
        .item(&quit)
        .build()?;

    let tray_state = TrayState {
        show_hide_item: show_hide.clone(),
    };
    app.manage(Mutex::new(tray_state));

    let icon_bytes = include_bytes!("../icons/32x32.png");
    let icon = match Image::from_bytes(icon_bytes) {
        Ok(img) => img,
        Err(_e) => {
            // fallback: try icon.png
            let icon_bytes2 = include_bytes!("../icons/icon.png");
            Image::from_bytes(icon_bytes2).expect("failed to load tray icon (icon.png)")
        }
    };

    TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .on_menu_event(move |app, event| {
            let id = event.id().0.as_str();
            match id {
                "show_hide" => {
                    if let Some(window) = app.get_webview_window("main") {
                        if let Some(state) = app.try_state::<Mutex<TrayState>>() {
                            let state = state.lock().unwrap();
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                                let _ = state.show_hide_item.set_text("Show App");
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                                let _ = state.show_hide_item.set_text("Hide App");
                            }
                        }
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            use tauri::tray::{MouseButton, TrayIconEvent};
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                ..
            } = event
            {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
