use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowBounds {
    fn default() -> Self {
        Self {
            x: 100,
            y: 100,
            width: 1200,
            height: 800,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub bounds: WindowBounds,
    #[serde(default)]
    pub maximized: bool,
    #[serde(default)]
    pub start_hidden: bool,
    #[serde(default = "default_icon_theme")]
    pub icon_theme: String,
    #[serde(default = "default_true")]
    pub use_tray: bool,
    #[serde(default = "default_true")]
    pub auto_update: bool,
    #[serde(default)]
    pub third_party_auth_mode: bool,
}

fn default_icon_theme() -> String {
    "default".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bounds: WindowBounds::default(),
            maximized: false,
            start_hidden: false,
            icon_theme: default_icon_theme(),
            use_tray: true,
            auto_update: true,
            third_party_auth_mode: false,
        }
    }
}

fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    config_dir.join("gochat").join("config.json")
}

pub fn load_config() -> Config {
    let path = get_config_path();

    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => match serde_json::from_str(&content) {
                Ok(config) => return config,
                Err(e) => eprintln!("Failed to parse config: {}", e),
            },
            Err(e) => eprintln!("Failed to read config: {}", e),
        }
    }

    Config::default()
}

pub fn save_config(config: &Config) {
    let path = get_config_path();

    if let Some(parent) = path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("Failed to create config directory: {}", e);
            return;
        }
    }

    match serde_json::to_string_pretty(config) {
        Ok(content) => {
            if let Err(e) = fs::write(&path, content) {
                eprintln!("Failed to write config: {}", e);
            }
        }
        Err(e) => eprintln!("Failed to serialize config: {}", e),
    }
}

pub fn save_window_state<R: tauri::Runtime>(app: &tauri::AppHandle<R>, config: &mut Config) {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(position) = window.outer_position() {
            config.bounds.x = position.x;
            config.bounds.y = position.y;
        }

        if let Ok(size) = window.outer_size() {
            config.bounds.width = size.width;
            config.bounds.height = size.height;
        }

        if let Ok(maximized) = window.is_maximized() {
            config.maximized = maximized;
        }
    }

    save_config(config);
}
