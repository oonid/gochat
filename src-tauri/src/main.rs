#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod tray;

use tauri::Manager;

const GOOGLE_CHAT_URL: &str = "https://mail.google.com/chat/u/0";

fn is_internal_url(url: &str) -> bool {
    if url == "about:blank" {
        return true;
    }
    url.starts_with("https://mail.google.com/chat")
        || url.starts_with("https://chat.google.com")
        || url.starts_with("https://accounts.google.com")
        || url.starts_with("https://accounts.youtube.com")
        || url.starts_with("https://myaccount.google.com")
        || url.starts_with("https://meet.google.com")
        || url.starts_with("https://ogs.google.com")
        || url.starts_with("https://www.google.com")
        || url.starts_with("https://google.com")
        || url.starts_with("https://contacts.google.com")
        || url.starts_with("https://studio.workspace.google.com")
        || url.starts_with("https://calendar.google.com")
        || url.starts_with("https://tasks.google.com")
}

fn create_splash_window(app: &tauri::AppHandle) -> tauri::WebviewWindow {
    tauri::WebviewWindowBuilder::new(app, "splash", tauri::WebviewUrl::App("index.html".into()))
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

            let splash = create_splash_window(app.handle());
            let splash_handle = splash.clone();

            let _main_window = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External(GOOGLE_CHAT_URL.parse().unwrap()),
            )
            .title("GoChat")
            .inner_size(1200.0, 800.0)
            .center()
            .resizable(true)
            .visible(false)
            .on_page_load(move |window, _payload| {
                let _ = window.show();
                let _ = window.set_focus();
                let _ = splash_handle.close();
            })
            .on_navigation(move |url| {
                let url_str = url.as_str();
                if is_internal_url(url_str) {
                    true
                } else {
                    let _ = tauri_plugin_opener::open_url(url_str, None::<&str>);
                    false
                }
            })
            .build()
            .expect("Failed to create main window");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
