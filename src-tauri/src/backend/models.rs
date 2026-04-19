use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct FileTreeNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub size: u64,
    pub size_str: String,
    pub percent: f32,
    pub file_count: u32,
    pub has_children: bool,
}

#[derive(Serialize, Clone)]
pub struct ScanProgress {
    pub file_count: u64,
    pub current_path: String,
}

#[derive(Serialize, Clone)]
pub struct ProjectCleanProgress {
    pub completed_items: u32,
    pub total_items: u32,
    pub current_item: String,
    pub approx_completed_bytes: u64,
}

#[derive(Clone)]
pub struct CleaningConfig {
    pub name: String,
    pub path: String,
    pub filter_days: Option<u64>,
    pub default_enabled: bool,
}

impl CleaningConfig {
    pub fn new(name: &str, path: &str, filter_days: Option<u64>, default_enabled: bool) -> Self {
        Self {
            name: name.into(),
            path: path.into(),
            filter_days,
            default_enabled,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ScanItem {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub count: u32,
    pub enabled: bool,
}

#[derive(Serialize)]
pub struct FastScanResult {
    pub items: Vec<ScanItem>,
    pub total_size: String,
    pub total_count: u32,
}

#[derive(Serialize)]
pub struct CleanResult {
    pub total_freed: String,
    pub success_count: u32,
    pub fail_count: u32,
}

#[derive(Serialize, Clone)]
pub struct BrowserProfile {
    pub name: String,
    pub path_name: String,
    pub cache_size: u64,
    pub cache_size_str: String,
}

#[derive(Serialize)]
pub struct BrowserScanResult {
    pub profiles: Vec<BrowserProfile>,
    pub total_size: String,
}

pub enum BrowserType {
    Chrome,
    Edge,
}

#[derive(Serialize, Clone)]
pub struct MemoryStats {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub percent: f32,
}
