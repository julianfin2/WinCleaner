use std::fs;
use std::path::{Path, PathBuf};

use crate::backend::fast_clean::clean_directory_contents;
use crate::backend::models::{
    BrowserProfile, BrowserScanResult, BrowserType, CleanResult, ProjectCleanProgress,
};
use crate::backend::utils::{format_size, get_dir_size_simple};
use tauri::Emitter;

const BROWSER_CACHE_DIRS: &[&str] = &[
    "Cache",
    "Code Cache",
    "GPUCache",
    "Media Cache",
    "Service Worker/CacheStorage",
    // "Service Worker/ScriptCache",
    // "GrShaderCache",
    // "DawnCache",
    // "File System",
    // "blob_storage",
];

impl BrowserType {
    fn get_user_data_path(&self) -> Result<PathBuf, String> {
        let local_app_data = std::env::var("LOCALAPPDATA").map_err(|_| "无法获取 LocalAppData 路径")?;
        let base = Path::new(&local_app_data);

        match self {
            BrowserType::Chrome => Ok(base.join("Google\\Chrome\\User Data")),
            BrowserType::Edge => Ok(base.join("Microsoft\\Edge\\User Data")),
        }
    }
}

pub async fn run_browser_scan(browser: BrowserType) -> Result<BrowserScanResult, String> {
    let user_data_path = browser.get_user_data_path()?;
    let local_state_path = user_data_path.join("Local State");

    let mut profiles = Vec::new();
    let mut total_bytes = 0;

    if local_state_path.exists() {
        let content = fs::read_to_string(local_state_path).map_err(|e| e.to_string())?;
        let value: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

        if let Some(info_cache) = value
            .get("profile")
            .and_then(|profile| profile.get("info_cache"))
            .and_then(|info| info.as_object())
        {
            for (dir_name, info) in info_cache {
                let profile_display_name = info.get("name").and_then(|name| name.as_str()).unwrap_or(dir_name);
                let profile_path = user_data_path.join(dir_name);

                if profile_path.exists() {
                    let mut size = 0;
                    for sub_dir in BROWSER_CACHE_DIRS {
                        let target = profile_path.join(sub_dir);
                        if target.exists() {
                            size += get_dir_size_simple(&target);
                        }
                    }

                    total_bytes += size;
                    profiles.push(BrowserProfile {
                        name: profile_display_name.to_string(),
                        path_name: dir_name.clone(),
                        cache_size: size,
                        cache_size_str: format_size(size),
                    });
                }
            }
        }
    }

    Ok(BrowserScanResult {
        profiles,
        total_size: format_size(total_bytes),
    })
}

pub async fn run_browser_clean(
    browser: BrowserType,
    profile_paths: Vec<String>,
    app_handle: tauri::AppHandle,
) -> Result<CleanResult, String> {
    let user_data_path = browser.get_user_data_path()?;
    let mut total_freed = 0;
    let mut success_count = 0;
    let mut fail_count = 0;
    let mut approx_completed_bytes = 0;
    let total_items = profile_paths.len() as u32;

    for (index, profile_dir) in profile_paths.into_iter().enumerate() {
        let profile_path = user_data_path.join(&profile_dir);
        let mut profile_estimated_size = 0;
        if profile_path.exists() {
            for sub_dir in BROWSER_CACHE_DIRS {
                let target = profile_path.join(sub_dir);
                if target.exists() {
                    profile_estimated_size += get_dir_size_simple(&target);
                    let (freed, success, fail) = clean_directory_contents(&target, None);
                    total_freed += freed;
                    success_count += success;
                    fail_count += fail;
                }
            }
        }

        approx_completed_bytes += profile_estimated_size;
        let _ = app_handle.emit(
            "browser-clean-progress",
            ProjectCleanProgress {
                completed_items: (index + 1) as u32,
                total_items,
                current_item: profile_dir,
                approx_completed_bytes,
            },
        );
    }

    Ok(CleanResult {
        total_freed: format_size(total_freed),
        success_count,
        fail_count,
    })
}
