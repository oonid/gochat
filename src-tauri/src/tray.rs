use std::sync::Mutex;
use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    path::BaseDirectory,
    tray::TrayIconBuilder,
    AppHandle, Manager, Runtime,
};

use crate::config;
use crate::updater;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TrayIconState {
    Normal,
    Badge,
    Offline,
}

pub fn build_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    let config_state = app.state::<Mutex<config::Config>>();
    let current_auth_mode = config_state.lock().unwrap().third_party_auth_mode;

    let show_hide = MenuItem::with_id(app, "show_hide", "Show", true, None::<&str>)?;
    let reload = MenuItem::with_id(app, "reload", "Reload", true, None::<&str>)?;
    let check_updates = MenuItem::with_id(
        app,
        "check_updates",
        "Check for Updates",
        true,
        None::<&str>,
    )?;
    let auth_toggle = MenuItem::with_id(
        app,
        "auth_toggle",
        &format!(
            "Third-party Auth: {}",
            if current_auth_mode { "On" } else { "Off" }
        ),
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(
        app,
        &[&show_hide, &reload, &check_updates, &auth_toggle, &quit],
    )?;

    let icon = load_tray_icon(app, TrayIconState::Normal)?;

    let _tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show_hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
            "reload" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.eval("window.location.reload()");
                }
            }
            "check_updates" => {
                let app_handle = app.clone();
                tauri::async_runtime::spawn(async move {
                    match updater::check_for_updates(&app_handle, false).await {
                        Ok(true) => {
                            if let Err(e) = updater::download_and_install(&app_handle).await {
                                eprintln!("Failed to install update: {}", e);
                            }
                        }
                        Ok(false) => {
                            eprintln!("App is up to date");
                        }
                        Err(e) => {
                            eprintln!("Update check failed: {}", e);
                        }
                    }
                });
            }
            "auth_toggle" => {
                toggle_auth_mode(app);
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click { .. } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
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

fn toggle_auth_mode<R: Runtime>(app: &AppHandle<R>) {
    let config_state = app.state::<Mutex<config::Config>>();
    let mut cfg = config_state.lock().unwrap();
    cfg.third_party_auth_mode = !cfg.third_party_auth_mode;
    let new_mode = cfg.third_party_auth_mode;
    config::save_config(&cfg);
    drop(cfg);

    eprintln!(
        "Third-party auth mode: {}",
        if new_mode { "ON" } else { "OFF" }
    );

    if let Some(tray) = app.tray_by_id("main") {
        let _ = tray.set_menu(Some(build_tray_menu(app, new_mode)));
    }

    if let Some(window) = app.get_webview_window("main") {
        let _ = window.eval("window.location.reload()");
    }
}

fn build_tray_menu<R: Runtime>(app: &AppHandle<R>, auth_mode: bool) -> Menu<R> {
    let show_hide = MenuItem::with_id(app, "show_hide", "Show", true, None::<&str>).unwrap();
    let reload = MenuItem::with_id(app, "reload", "Reload", true, None::<&str>).unwrap();
    let check_updates = MenuItem::with_id(app, "check_updates", "Check for Updates", true, None::<&str>).unwrap();
    let auth_toggle = MenuItem::with_id(
        app,
        "auth_toggle",
        &format!("Third-party Auth: {}", if auth_mode { "On" } else { "Off" }),
        true,
        None::<&str>,
    )
    .unwrap();
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();

    Menu::with_items(app, &[&show_hide, &reload, &check_updates, &auth_toggle, &quit]).unwrap()
}

fn load_tray_icon<R: Runtime>(
    app: &AppHandle<R>,
    state: TrayIconState,
) -> Result<Image<'static>, Box<dyn std::error::Error>> {
    let icon_name = match state {
        TrayIconState::Normal => "normal.png",
        TrayIconState::Badge => "badge.png",
        TrayIconState::Offline => "offline.png",
    };

    let icon_path = app
        .path()
        .resolve(format!("icons/default/{}", icon_name), BaseDirectory::Resource)?;

    Image::from_path(icon_path).map_err(Into::into)
}

pub fn update_tray_icon<R: Runtime>(
    app: &AppHandle<R>,
    state: TrayIconState,
) -> Result<(), Box<dyn std::error::Error>> {
    let tray = app.tray_by_id("main").ok_or("Tray not found")?;
    let icon = load_tray_icon(app, state)?;
    tray.set_icon(Some(icon))?;
    Ok(())
}
