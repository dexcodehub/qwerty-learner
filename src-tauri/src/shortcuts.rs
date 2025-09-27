use tauri::{AppHandle, GlobalShortcutManager, Manager};

/// 注册全局快捷键
pub fn register_global_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let mut shortcut_manager = app.global_shortcut_manager();

    // 注册 Cmd+Shift+Q 快捷键来切换窗口显示/隐藏
    let app_handle = app.clone();
    shortcut_manager.register("Cmd+Shift+Q", move || {
        if let Some(window) = app_handle.get_window("main") {
            if let Ok(is_visible) = window.is_visible() {
                if is_visible {
                    let _ = window.hide();
                } else {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        }
    })?;

    // 注册 Cmd+Shift+F 快捷键来切换全屏模式
    let app_handle = app.clone();
    shortcut_manager.register("Cmd+Shift+F", move || {
        if let Some(window) = app_handle.get_window("main") {
            if let Ok(is_fullscreen) = window.is_fullscreen() {
                let _ = window.set_fullscreen(!is_fullscreen);
            }
        }
    })?;

    // 注册 Cmd+Shift+T 快捷键来切换置顶状态
    let app_handle = app.clone();
    shortcut_manager.register("Cmd+Shift+T", move || {
        if let Some(window) = app_handle.get_window("main") {
            // 这里简单切换置顶状态，实际应用中可能需要保存状态
            let _ = window.set_always_on_top(true);
            // 可以添加一个定时器来取消置顶，或者通过其他方式管理状态
        }
    })?;

    // 注册 Cmd+Shift+R 快捷键来重新加载应用
    let app_handle = app.clone();
    shortcut_manager.register("Cmd+Shift+R", move || {
        if let Some(window) = app_handle.get_window("main") {
            let _ = window.eval("location.reload()");
        }
    })?;

    println!("全局快捷键注册成功:");
    println!("  Cmd+Shift+Q - 切换窗口显示/隐藏");
    println!("  Cmd+Shift+F - 切换全屏模式");
    println!("  Cmd+Shift+T - 窗口置顶");
    println!("  Cmd+Shift+R - 重新加载应用");

    Ok(())
}

/// 注销所有全局快捷键
pub fn unregister_all_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let mut shortcut_manager = app.global_shortcut_manager();
    shortcut_manager.unregister_all()?;
    println!("所有全局快捷键已注销");
    Ok(())
}