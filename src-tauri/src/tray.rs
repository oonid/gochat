use tauri::{
    image::Image,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager, Runtime,
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TrayIconState {
    Normal,
    Badge,
    Offline,
}

pub struct TrayState {
    pub current_icon: TrayIconState,
    pub auth_mode: bool,
}

impl Default for TrayState {
    fn default() -> Self {
        Self {
            current_icon: TrayIconState::Normal,
            auth_mode: false,
        }
    }
}

pub fn build_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    let show_hide = MenuItem::with_id(app, "show_hide", "Show", true, None::<&str>)?;
    let reload = MenuItem::with_id(app, "reload", "Reload", true, None::<&str>)?;
    let auth_toggle = MenuItem::with_id(
        app,
        "auth_toggle",
        "Third-party Auth: Off",
        true,
        None::<&str>,
    )?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    let menu = Menu::with_items(app, &[&show_hide, &reload, &auth_toggle, &quit])?;

    let icon = load_tray_icon(TrayIconState::Normal)?;

    let tray = TrayIconBuilder::with_id("main")
        .icon(icon)
        .menu(&menu)
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
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
                "auth_toggle" => {
                    // TODO: implement auth toggle with config persistence
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
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

fn load_tray_icon(state: TrayIconState) -> Result<Image<'static>, Box<dyn std::error::Error>> {
    let icon_path = match state {
        TrayIconState::Normal => "icons/default/normal.png",
        TrayIconState::Badge => "icons/default/badge.png",
        TrayIconState::Offline => "icons/default/offline.png",
    };

    Image::from_path(icon_path).map_err(Into::into)
}

pub fn update_tray_icon<R: Runtime>(
    app: &AppHandle<R>,
    state: TrayIconState,
) -> Result<(), Box<dyn std::error::Error>> {
    let tray = app.tray_by_id("main").ok_or("Tray not found")?;
    let icon = load_tray_icon(state)?;
    tray.set_icon(Some(icon))?;
    Ok(())
}
