use std::collections::HashMap;
use std::sync::Mutex;

pub struct DiskState {
    pub dir_sizes: Mutex<HashMap<String, u64>>,
}
