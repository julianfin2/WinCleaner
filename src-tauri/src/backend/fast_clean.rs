use std::fs;
use std::path::Path;
use std::time::{Duration, SystemTime};

use tauri::Emitter;

use crate::backend::models::{CleanResult, CleaningConfig, FastScanResult, ProjectCleanProgress, ScanItem};
use crate::backend::utils::format_size;

fn get_fast_cleaning_configs() -> Vec<CleaningConfig> {
    let mut configs = Vec::new();

    if let Ok(temp) = std::env::var("TEMP") {
        configs.push(CleaningConfig::new("用户临时文件", &temp, None, true));
    }

    configs.push(CleaningConfig::new("系统临时文件", "C:\\Windows\\Temp", None, true));
    configs.push(CleaningConfig::new(
        "Windows 更新残留",
        "C:\\Windows\\SoftwareDistribution\\Download",
        Some(10),
        true,
    ));
    configs.push(CleaningConfig::new("回收站", "C:\\$Recycle.Bin", None, true));
    configs.push(CleaningConfig::new(
        "内核转储文件",
        "C:\\Windows\\LiveKernelReports",
        None,
        false,
    ));

    configs
}

pub async fn run_fast_scan() -> FastScanResult {
    let configs = get_fast_cleaning_configs();
    let mut items = Vec::new();
    let mut total_bytes = 0;
    let mut total_count = 0;

    for config in configs {
        let (size, count) = get_dir_stats(Path::new(&config.path), config.filter_days);
        items.push(ScanItem {
            name: config.name,
            path: config.path,
            size,
            count,
            enabled: config.default_enabled,
        });
        total_bytes += size;
        total_count += count;
    }

    FastScanResult {
        items,
        total_size: format_size(total_bytes),
        total_count,
    }
}

fn get_dir_stats(path: &Path, filter_days: Option<u64>) -> (u64, u32) {
    if !path.exists() {
        return (0, 0);
    }

    let mut size = 0;
    let mut count = 0;
    let now = SystemTime::now();
    let dur = filter_days.map(|days| Duration::from_secs(days * 24 * 3600));

    for entry in walkdir::WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let mut allowed = true;
            if let (Some(filter_duration), Ok(metadata)) = (dur, entry.metadata()) {
                if let Ok(modified_time) = metadata.modified() {
                    if let Ok(elapsed) = now.duration_since(modified_time) {
                        if elapsed < filter_duration {
                            allowed = false;
                        }
                    }
                }
            }

            if allowed {
                size += entry.metadata().map(|m| m.len()).unwrap_or(0);
                count += 1;
            }
        }
    }

    (size, count)
}

pub async fn run_fast_clean(
    selected_paths: Vec<String>,
    app_handle: tauri::AppHandle,
) -> Result<CleanResult, String> {
    let selected_configs: Vec<CleaningConfig> = get_fast_cleaning_configs()
        .into_iter()
        .filter(|config| selected_paths.contains(&config.path))
        .collect();

    let mut success_count = 0;
    let mut fail_count = 0;
    let mut total_freed = 0;
    let mut approx_completed_bytes = 0;
    let total_items = selected_configs.len() as u32;

    for (index, config) in selected_configs.into_iter().enumerate() {
        let path = Path::new(&config.path);
        let item_size = get_dir_stats(path, config.filter_days).0;

        if path.exists() {
            let (freed, success, fail) = clean_directory_contents(path, config.filter_days);
            total_freed += freed;
            success_count += success;
            fail_count += fail;
        }

        approx_completed_bytes += item_size;
        let _ = app_handle.emit(
            "fast-clean-progress",
            ProjectCleanProgress {
                completed_items: (index + 1) as u32,
                total_items,
                current_item: config.name,
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

pub fn clean_directory_contents(path: &Path, filter_days: Option<u64>) -> (u64, u32, u32) {
    let mut freed = 0;
    let mut success = 0;
    let mut fail = 0;
    let now = SystemTime::now();
    let dur = filter_days.map(|days| Duration::from_secs(days * 24 * 3600));

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            let metadata = entry.metadata();
            let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);

            if let (Some(filter_duration), Ok(metadata)) = (dur, &metadata) {
                if let Ok(modified_time) = metadata.modified() {
                    if let Ok(elapsed) = now.duration_since(modified_time) {
                        if elapsed < filter_duration {
                            continue;
                        }
                    }
                }
            }

            if entry_path.is_file() {
                if fs::remove_file(&entry_path).is_ok() {
                    freed += size;
                    success += 1;
                } else {
                    fail += 1;
                }
            } else if entry_path.is_dir() {
                let (dir_freed, dir_success, dir_fail) =
                    clean_directory_contents(&entry_path, filter_days);
                freed += dir_freed;
                success += dir_success;
                fail += dir_fail;

                if fs::remove_dir(&entry_path).is_ok() {
                    success += 1;
                }
            }
        }
    }

    (freed, success, fail)
}
