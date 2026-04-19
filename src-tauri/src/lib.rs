use std::collections::HashMap;
use std::sync::Mutex;
use tauri::State;

mod backend;

#[tauri::command]
async fn start_fast_scan() -> backend::models::FastScanResult {
    backend::fast_clean::run_fast_scan().await
}

#[tauri::command]
async fn start_fast_clean(
    selected_paths: Vec<String>,
    app_handle: tauri::AppHandle,
) -> Result<backend::models::CleanResult, String> {
    backend::fast_clean::run_fast_clean(selected_paths, app_handle).await
}

#[tauri::command]
async fn start_full_disk_scan(
    state: State<'_, backend::state::DiskState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    backend::disk_analysis::run_full_scan("C:\\".to_string(), &state, app_handle).await;
    Ok(())
}

#[tauri::command]
async fn get_tree_children(
    path: String,
    state: State<'_, backend::state::DiskState>,
) -> Result<Vec<backend::models::FileTreeNode>, String> {
    Ok(backend::disk_analysis::get_children(path, &state))
}

#[tauri::command]
async fn open_in_explorer(path: String) -> Result<(), String> {
    backend::disk_analysis::open_explorer(path).await
}

#[tauri::command]
async fn clean_system_components() -> Result<String, String> {
    backend::advanced_clean::run_dism_cleanup().await
}

#[tauri::command]
async fn clean_thumbnails() -> Result<String, String> {
    backend::advanced_clean::clean_thumbnails().await
}

#[tauri::command]
async fn disable_hibernation() -> Result<String, String> {
    backend::advanced_clean::disable_hibernation().await
}

#[tauri::command]
async fn start_browser_scan(browser: String) -> Result<backend::models::BrowserScanResult, String> {
    let browser_type = if browser == "chrome" {
        backend::models::BrowserType::Chrome
    } else {
        backend::models::BrowserType::Edge
    };

    backend::browser_clean::run_browser_scan(browser_type).await
}

#[tauri::command]
async fn start_browser_clean(
    browser: String,
    profiles: Vec<String>,
    app_handle: tauri::AppHandle,
) -> Result<backend::models::CleanResult, String> {
    let browser_type = if browser == "chrome" {
        backend::models::BrowserType::Chrome
    } else {
        backend::models::BrowserType::Edge
    };

    backend::browser_clean::run_browser_clean(browser_type, profiles, app_handle).await
}

#[tauri::command]
async fn get_memory_stats() -> backend::models::MemoryStats {
    backend::memory_clean::get_memory_stats()
}

#[tauri::command]
async fn run_memory_clean() -> Result<u64, String> {
    backend::memory_clean::run_memory_clean().await
}

#[tauri::command]
async fn run_deep_memory_clean() -> Result<u64, String> {
    backend::memory_clean::run_deep_memory_clean().await
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(backend::state::DiskState {
            dir_sizes: Mutex::new(HashMap::new()),
        })
        .invoke_handler(tauri::generate_handler![
            start_fast_scan,
            start_fast_clean,
            start_full_disk_scan,
            get_tree_children,
            open_in_explorer,
            clean_system_components,
            clean_thumbnails,
            disable_hibernation,
            start_browser_scan,
            start_browser_clean,
            get_memory_stats,
            run_memory_clean,
            run_deep_memory_clean
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
