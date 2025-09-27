use std::path::PathBuf;
use dirs;

/// 获取用户文档目录
pub fn get_documents_dir() -> Option<PathBuf> {
    dirs::document_dir()
}

/// 获取用户下载目录
pub fn get_downloads_dir() -> Option<PathBuf> {
    dirs::download_dir()
}

/// 获取用户桌面目录
pub fn get_desktop_dir() -> Option<PathBuf> {
    dirs::desktop_dir()
}

/// 获取应用配置目录
pub fn get_config_dir() -> Option<PathBuf> {
    dirs::config_dir().map(|dir| dir.join("qwerty-learner"))
}

/// 获取应用缓存目录
pub fn get_cache_dir() -> Option<PathBuf> {
    dirs::cache_dir().map(|dir| dir.join("qwerty-learner"))
}

/// 确保目录存在，如果不存在则创建
pub fn ensure_dir_exists(path: &PathBuf) -> Result<(), std::io::Error> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// 格式化文件大小
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", size as u64, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// 检查文件是否存在且可读
pub fn is_file_readable(path: &PathBuf) -> bool {
    path.exists() && path.is_file() && path.metadata().map(|m| !m.permissions().readonly()).unwrap_or(false)
}

/// 生成时间戳文件名
pub fn generate_timestamp_filename(prefix: &str, extension: &str) -> String {
    let now = chrono::Utc::now();
    format!("{}_{}.{}", prefix, now.format("%Y%m%d_%H%M%S"), extension)
}