use sysinfo::{ProcessesToUpdate, System};

use crate::backend::models::MemoryStats;

pub fn get_memory_stats() -> MemoryStats {
    let mut sys = System::new_all();
    sys.refresh_memory();

    let total = sys.total_memory();
    let used = sys.used_memory();
    let free = total.saturating_sub(used);
    let percent = (used as f32 / total as f32) * 100.0;

    MemoryStats {
        total,
        used,
        free,
        percent,
    }
}

pub async fn run_memory_clean() -> Result<u64, String> {
    use windows_sys::Win32::Foundation::CloseHandle;
    use windows_sys::Win32::System::ProcessStatus::EmptyWorkingSet;
    use windows_sys::Win32::System::Threading::{
        OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_SET_QUOTA,
    };

    let before = get_memory_stats().used;

    let mut sys = System::new_all();
    sys.refresh_processes(ProcessesToUpdate::All, true);

    for (pid, _) in sys.processes() {
        let pid_u32 = pid.as_u32();
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_SET_QUOTA, 0, pid_u32);
            if handle != std::ptr::null_mut() {
                EmptyWorkingSet(handle);
                CloseHandle(handle);
            }
        }
    }

    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let after = get_memory_stats().used;
    Ok(before.saturating_sub(after))
}

pub async fn run_deep_memory_clean() -> Result<u64, String> {
    use windows_sys::Win32::System::Memory::SetSystemFileCacheSize;

    let before = get_memory_stats().used;

    unsafe {
        SetSystemFileCacheSize(usize::MAX, usize::MAX, 0);
    }

    let after = get_memory_stats().used;
    Ok(before.saturating_sub(after))
}
