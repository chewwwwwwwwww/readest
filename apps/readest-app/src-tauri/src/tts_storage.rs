use std::sync::Mutex;
use tauri::Manager;

// Serialize preparations within the app before any caller opens SQLite.
static PREPARATION: Mutex<()> = Mutex::new(());

#[tauri::command]
pub fn prepare_tts_storage(app: tauri::AppHandle) -> Result<String, String> {
    let _guard = PREPARATION.lock().map_err(|error| error.to_string())?;
    let legacy = app
        .path()
        .app_cache_dir()
        .map_err(|error| error.to_string())?
        .join("tts-cache");
    let durable = app
        .path()
        .app_local_data_dir()
        .map_err(|error| error.to_string())?
        .join("tts-cache");
    let root = crate::tts_storage_dir::prepare_directory(&legacy, &durable)?;
    #[cfg(target_os = "ios")]
    exclude_from_backup(&root)?;
    root.to_str()
        .map(str::to_owned)
        .ok_or_else(|| "TTS storage path is not UTF-8".to_owned())
}

#[cfg(target_os = "ios")]
fn exclude_from_backup(path: &std::path::Path) -> Result<(), String> {
    use objc2_foundation::{NSNumber, NSString, NSURLIsExcludedFromBackupKey, NSURL};
    let path = path.to_str().ok_or("TTS storage path is not UTF-8")?;
    let url = NSURL::fileURLWithPath_isDirectory(&NSString::from_str(path), true);
    let excluded = NSNumber::numberWithBool(true);
    // SAFETY: NSURLIsExcludedFromBackupKey requires an NSNumber boolean.
    unsafe { url.setResourceValue_forKey_error(Some(&excluded), NSURLIsExcludedFromBackupKey) }
        .map_err(|error| format!("Cannot exclude durable TTS audio from iOS backups: {error}"))
}
