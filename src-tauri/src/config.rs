use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

const MIN_WINDOW_WIDTH: u32 = 400;
const MIN_WINDOW_HEIGHT: u32 = 300;
const MAX_WINDOW_WIDTH: u32 = 3840;
const MAX_WINDOW_HEIGHT: u32 = 2160;
const VALID_ICON_THEMES: &[&str] = &["default", "colored", "mono"];

const CUSTOM_CSS_TEMPLATE: &str = r#"/* GoChat Custom CSS
 * 
 * This file allows you to customize the appearance of Google Chat.
 * Edit this file and reload the app to see your changes.
 * 
 * Example customizations:
 */

/* Darker sidebar 
body {
  --sidebar-bg: #1a1a1a !important;
}
*/

/* Hide specific elements
 selector {
  display: none !important;
}
*/
"#;

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

impl Config {
    pub fn validate(&mut self) {
        self.bounds.width = self.bounds.width.clamp(MIN_WINDOW_WIDTH, MAX_WINDOW_WIDTH);
        self.bounds.height = self
            .bounds
            .height
            .clamp(MIN_WINDOW_HEIGHT, MAX_WINDOW_HEIGHT);

        if !VALID_ICON_THEMES.contains(&self.icon_theme.as_str()) {
            self.icon_theme = default_icon_theme();
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
            Ok(content) => match serde_json::from_str::<Config>(&content) {
                Ok(mut config) => {
                    config.validate();
                    return config;
                }
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

pub fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("gochat")
}

pub fn ensure_config_dir() -> PathBuf {
    let config_dir = get_config_dir();

    if !config_dir.exists() {
        if let Err(e) = fs::create_dir_all(&config_dir) {
            eprintln!("Failed to create config directory: {}", e);
        }
    }

    config_dir
}

pub fn get_custom_css_path() -> PathBuf {
    ensure_config_dir().join("custom.css")
}

pub fn load_custom_css() -> Option<String> {
    let css_path = get_custom_css_path();

    if css_path.exists() {
        match fs::read_to_string(&css_path) {
            Ok(css) => {
                if css.trim().is_empty() {
                    None
                } else {
                    Some(css)
                }
            }
            Err(e) => {
                eprintln!("Failed to read custom.css: {}", e);
                None
            }
        }
    } else {
        None
    }
}

pub fn create_custom_css_template() {
    let css_path = get_custom_css_path();

    if !css_path.exists() {
        if let Err(e) = fs::write(&css_path, CUSTOM_CSS_TEMPLATE) {
            eprintln!("Failed to create custom.css template: {}", e);
        } else {
            println!("Created custom.css template at {:?}", css_path);
        }
    }
}
