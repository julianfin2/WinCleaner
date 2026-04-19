use std::collections::HashMap;
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

use tauri::Emitter;

use crate::backend::models::{FileTreeNode, ScanProgress};
use crate::backend::state::DiskState;
use crate::backend::utils::format_size;

pub async fn run_full_scan(root_path: String, state: &DiskState, app_handle: tauri::AppHandle) {
    use jwalk::WalkDir;

    let mut dir_sizes = HashMap::new();
    let root = Path::new(&root_path);
    let mut file_count = 0;

    for entry in WalkDir::new(root).skip_hidden(false).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type.is_file() {
            file_count += 1;

            if file_count % 2000 == 0 {
                let _ = app_handle.emit(
                    "scan-progress",
                    ScanProgress {
                        file_count,
                        current_path: entry.parent_path().to_string_lossy().to_string(),
                    },
                );
            }

            let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
            let mut current_path = entry.parent_path().to_path_buf();
            while current_path.starts_with(root) {
                let path_str = current_path.to_string_lossy().to_string();
                *dir_sizes.entry(path_str).or_insert(0) += size;
                if current_path == root {
                    break;
                }
                if let Some(parent) = current_path.parent() {
                    current_path = parent.to_path_buf();
                } else {
                    break;
                }
            }
        }
    }

    let mut state_dirs = state.dir_sizes.lock().unwrap();
    *state_dirs = dir_sizes;
}

pub fn get_children(parent_path: String, state: &DiskState) -> Vec<FileTreeNode> {
    let dir_sizes = state.dir_sizes.lock().unwrap();
    let mut results = Vec::new();
    let parent_size = *dir_sizes.get(&parent_path).unwrap_or(&1);

    if let Ok(entries) = fs::read_dir(Path::new(&parent_path)) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            let path_str = path.to_string_lossy().to_string();
            let is_dir = path.is_dir();
            let name = entry.file_name().to_string_lossy().to_string();

            let size = if is_dir {
                *dir_sizes.get(&path_str).unwrap_or(&0)
            } else {
                entry.metadata().map(|m| m.len()).unwrap_or(0)
            };

            if size > 0 || !is_dir {
                results.push(FileTreeNode {
                    name,
                    path: path_str,
                    is_dir,
                    size,
                    size_str: format_size(size),
                    percent: (size as f64 / parent_size as f64 * 100.0) as f32,
                    file_count: 0,
                    has_children: is_dir,
                });
            }
        }
    }

    results.sort_by(|a, b| b.size.cmp(&a.size));
    results
}

pub async fn open_explorer(path: String) -> Result<(), String> {
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    Command::new("explorer.exe")
        .arg("/select,")
        .arg(&path)
        .creation_flags(CREATE_NO_WINDOW)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}
