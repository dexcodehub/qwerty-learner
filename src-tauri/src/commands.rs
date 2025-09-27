use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, Window};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: String,
    pub version: String,
    pub arch: String,
}

/// 获取应用数据目录
#[tauri::command]
pub fn get_app_data_dir(app_handle: AppHandle) -> Result<String, String> {
    match app_handle.path_resolver().app_data_dir() {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err("无法获取应用数据目录".to_string()),
    }
}

/// 保存用户数据到本地文件
#[tauri::command]
pub async fn save_user_data(
    app_handle: AppHandle,
    filename: String,
    data: String,
) -> Result<(), String> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("无法获取应用数据目录")?;

    // 确保目录存在
    fs::create_dir_all(&app_data_dir).map_err(|e| format!("创建目录失败: {}", e))?;

    let file_path = app_data_dir.join(filename);
    fs::write(file_path, data).map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(())
}

/// 从本地文件加载用户数据
#[tauri::command]
pub async fn load_user_data(app_handle: AppHandle, filename: String) -> Result<String, String> {
    let app_data_dir = app_handle
        .path_resolver()
        .app_data_dir()
        .ok_or("无法获取应用数据目录")?;

    let file_path = app_data_dir.join(filename);
    
    if !file_path.exists() {
        return Ok(String::new());
    }

    fs::read_to_string(file_path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 导出数据到指定文件
#[tauri::command]
pub async fn export_data_to_file(file_path: String, data: String) -> Result<(), String> {
    let path = PathBuf::from(file_path);
    
    // 确保父目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    fs::write(path, data).map_err(|e| format!("导出文件失败: {}", e))?;
    Ok(())
}

/// 从指定文件导入数据
#[tauri::command]
pub async fn import_data_from_file(file_path: String) -> Result<String, String> {
    let path = PathBuf::from(file_path);
    
    if !path.exists() {
        return Err("文件不存在".to_string());
    }

    fs::read_to_string(path).map_err(|e| format!("导入文件失败: {}", e))
}

/// 显示系统通知
#[tauri::command]
pub async fn show_notification(
    app_handle: AppHandle,
    title: String,
    body: String,
) -> Result<(), String> {
    // 暂时使用控制台输出替代系统通知，避免 macOS 编译问题
    println!("通知: {} - {}", title, body);
    Ok(())
}

/// 切换窗口可见性
#[tauri::command]
pub async fn toggle_window_visibility(window: Window) -> Result<(), String> {
    if window.is_visible().map_err(|e| format!("检查窗口状态失败: {}", e))? {
        window.hide().map_err(|e| format!("隐藏窗口失败: {}", e))?;
    } else {
        window.show().map_err(|e| format!("显示窗口失败: {}", e))?;
        window.set_focus().map_err(|e| format!("设置窗口焦点失败: {}", e))?;
    }
    Ok(())
}

/// 设置窗口置顶状态
#[tauri::command]
pub async fn set_window_always_on_top(window: Window, always_on_top: bool) -> Result<(), String> {
    window
        .set_always_on_top(always_on_top)
        .map_err(|e| format!("设置窗口置顶失败: {}", e))?;
    Ok(())
}

/// 获取系统信息
#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        platform: std::env::consts::OS.to_string(),
        version: "1.0.0".to_string(), // 可以从系统获取实际版本
        arch: std::env::consts::ARCH.to_string(),
    })
}