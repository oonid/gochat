#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};

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
            let splash = create_splash_window(app.handle());
            let splash_handle = splash.clone();

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
