use tauri::{AppHandle, Runtime};
use tauri_plugin_updater::UpdaterExt;

pub async fn check_for_updates<R: Runtime>(
    app: &AppHandle<R>,
    silent: bool,
) -> Result<bool, String> {
    let updater = app.updater().map_err(|e| e.to_string())?;

    match updater.check().await {
        Ok(Some(update)) => {
            eprintln!(
                "Update available: {} -> {}",
                update.current_version,
                update.version
            );

            if !silent {
                show_update_notification(app, &update.version)?;
            }

            return Ok(true);
        }
        Ok(None) => {
            if !silent {
                eprintln!("No updates available");
            }
            return Ok(false);
        }
        Err(e) => {
            if !silent {
                eprintln!("Failed to check for updates: {}", e);
            }
            return Err(e.to_string());
        }
    }
}

fn show_update_notification<R: Runtime>(
    app: &AppHandle<R>,
    version: &str,
) -> Result<(), String> {
    use tauri_plugin_notification::NotificationExt;

    app.notification()
        .builder()
        .title("GoChat Update Available")
        .body(&format!(
            "Version {} is available. Check the tray menu to update.",
            version
        ))
        .show()
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub async fn download_and_install<R: Runtime>(app: &AppHandle<R>) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;

    match updater.check().await {
        Ok(Some(update)) => {
            eprintln!("Downloading update {}...", update.version);

            let mut downloaded = 0u64;

            update
                .download_and_install(
                    |chunk_length, _content_length| {
                        downloaded += chunk_length as u64;
                        eprintln!("Downloaded {} bytes", downloaded);
                    },
                    || {
                        eprintln!("Download complete, installing...");
                    },
                )
                .await
                .map_err(|e| e.to_string())?;

            eprintln!("Update installed. Restart to apply.");
            Ok(())
        }
        Ok(None) => {
            eprintln!("No update available");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to download update: {}", e);
            Err(e.to_string())
        }
    }
}
