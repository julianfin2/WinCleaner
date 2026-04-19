use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;

pub async fn run_dism_cleanup() -> Result<String, String> {
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("dism.exe")
        .args(["/online", "/Cleanup-Image", "/StartComponentCleanup"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("系统组件清理完成。".into())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub async fn clean_thumbnails() -> Result<String, String> {
    let local_app_data = std::env::var("LOCALAPPDATA").map_err(|_| "无法获取 LocalAppData 路径")?;
    let thumb_path = Path::new(&local_app_data).join("Microsoft\\Windows\\Explorer");

    let mut count = 0;
    if thumb_path.exists() {
        if let Ok(entries) = fs::read_dir(thumb_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name.starts_with("thumbcache_") && name.ends_with(".db") {
                    if fs::remove_file(entry.path()).is_ok() {
                        count += 1;
                    }
                }
            }
        }
    }

    if count > 0 {
        Ok(format!("成功清理 {} 个缩略图缓存文件。", count))
    } else {
        Ok("未发现可清理的缩略图缓存，或文件正被系统占用。".into())
    }
}

pub async fn disable_hibernation() -> Result<String, String> {
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let output = Command::new("powercfg.exe")
        .args(["-h", "off"])
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok("休眠模式已关闭，hiberfil.sys 已移除。".into())
    } else {
        Err("执行失败，请确保以管理员身份运行。".into())
    }
}
