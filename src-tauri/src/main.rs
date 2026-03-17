#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod injection;
mod tray;

use tauri::{Event, Listener, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_notification::NotificationExt;
use tray::TrayIconState;

const GOOGLE_CHAT_URL: &str = "https://mail.google.com/chat/u/0";

fn is_internal_url(url: &str) -> bool {
    url.starts_with("https://mail.google.com/chat")
        || url.starts_with("https://chat.google.com")
        || url.starts_with("https://accounts.google.com")
        || url.starts_with("https://accounts.youtube.com")
        || url.starts_with("https://myaccount.google.com")
        || url.starts_with("https://meet.google.com")
}

fn create_splash_window(app: &tauri::AppHandle) -> tauri::WebviewWindow {
    WebviewWindowBuilder::new(app, "splash", WebviewUrl::App("index.html".into()))
        .title("GoChat")
        .inner_size(400.0, 300.0)
        .center()
        .resizable(false)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .build()
        .expect("Failed to create splash window")
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
                let _ = window.show();
            }
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tray::build_tray(app.handle())?;
            
            let app_handle = app.handle().clone();
            app.listen("favicon-changed", move |event: Event| {
                let payload = event.payload();
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
                    if let Some(state_str) = data.get("state").and_then(|s| s.as_str()) {
                        let state = match state_str {
                            "badge" => TrayIconState::Badge,
                            "offline" => TrayIconState::Offline,
                            _ => TrayIconState::Normal,
                        };
                        let _ = tray::update_tray_icon(&app_handle, state);
                    }
                }
            });

            let app_handle_for_notif = app.handle().clone();
            app.listen("desktop-notification", move |event: Event| {
                let payload = event.payload();
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
                    let title = data.get("title").and_then(|s| s.as_str()).unwrap_or("GoChat");
                    let body = data.get("body").and_then(|s| s.as_str()).unwrap_or("");
                    
                    if let Err(e) = app_handle_for_notif
                        .notification()
                        .builder()
                        .title(title)
                        .body(body)
                        .show()
                    {
                        eprintln!("Failed to show notification: {}", e);
                    }
                }
            });

            let _app_handle = app.handle().clone();
            app.listen("notification-permission", move |event: Event| {
                let payload = event.payload();
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
                    if data.get("status").and_then(|s| s.as_str()) == Some("denied") {
                        eprintln!("GoChat: Notification permission denied by user");
                    }
                }
            });

            let app_handle = app.handle().clone();
            app.listen("unread-count", move |event: Event| {
                let payload = event.payload();
                if let Ok(data) = serde_json::from_str::<serde_json::Value>(payload) {
                    if let Some(count) = data.get("count").and_then(|c| c.as_u64()) {
                        if count > 0 {
                            let _ = tray::update_tray_icon(&app_handle, TrayIconState::Badge);
                        }
                    }
                }
            });
            
            let splash = create_splash_window(app.handle());
            let splash_handle = splash.clone();
            let favicon_script = injection::get_favicon_monitor_script();
            let notification_script = injection::get_notification_script();
            
            WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External(GOOGLE_CHAT_URL.parse().unwrap()),
            )
            .title("GoChat")
            .inner_size(1200.0, 800.0)
            .center()
            .resizable(true)
            .visible(false)
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
            .on_page_load(move |window, _payload| {
                let _ = window.eval(&favicon_script);
                let _ = window.eval(&notification_script);
                let _ = window.show();
                let _ = window.set_focus();
                let _ = splash_handle.close();
            })
            .on_navigation(move |url| {
                let url_str = url.as_str();
                if !is_internal_url(url_str) {
                    let _ = tauri_plugin_opener::open_url(url_str, None::<&str>);
                    return false;
                }
                true
            })
            .build()
            .expect("Failed to create main window");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
