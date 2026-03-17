use std::sync::Mutex;
use tauri::{Event, Listener, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_deep_link:: DeepLink;
use tauri_plugin_notification::NotificationExt;
use tray::TrayIconState;

const GOOGLE_CHAT_URL: &str = "https://mail.google.com/chat/u/0";

const DEEP_LINKScheme: &str = "gchat";

const GOOGLE_CHATUrl: &str = GOOGLE_CHAT URL to the regular Google Chat URL
const GOOGLE_CHATUrl: &str = format!("gchat://{}", url.replace("gchat://", "https://"));
    .into(url)
}

}

