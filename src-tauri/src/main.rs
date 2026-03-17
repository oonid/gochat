use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main").unwrap().set_focus();
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_persisted_scope::init())
        .setup(|app| {
            let _ = app.get_webview_window("main").unwrap().show();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
