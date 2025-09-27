use tauri::{
    AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu,
    SystemTrayMenuItem,
};

/// 创建系统托盘菜单
pub fn create_tray_menu() -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_item(CustomMenuItem::new("show".to_string(), "显示窗口"))
        .add_item(CustomMenuItem::new("hide".to_string(), "隐藏窗口"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("focus_mode".to_string(), "专注模式"))
        .add_item(CustomMenuItem::new("settings".to_string(), "设置"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("about".to_string(), "关于"))
        .add_item(CustomMenuItem::new("quit".to_string(), "退出"))
}

/// 处理系统托盘事件
pub fn handle_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            // 左键点击托盘图标，切换窗口显示状态
            if let Some(window) = app.get_window("main") {
                if let Ok(is_visible) = window.is_visible() {
                    if is_visible {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            // 右键点击显示菜单（默认行为）
            println!("右键点击托盘图标");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            // 双击托盘图标，显示并聚焦窗口
            if let Some(window) = app.get_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            handle_menu_item_click(app, &id);
        }
        _ => {}
    }
}

/// 处理托盘菜单项点击事件
fn handle_menu_item_click(app: &AppHandle, menu_id: &str) {
    match menu_id {
        "show" => {
            if let Some(window) = app.get_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "hide" => {
            if let Some(window) = app.get_window("main") {
                let _ = window.hide();
            }
        }
        "focus_mode" => {
            // 专注模式：隐藏其他应用，窗口置顶
            if let Some(window) = app.get_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
                let _ = window.set_always_on_top(true);
                // 可以添加更多专注模式的功能
            }
        }
        "settings" => {
            // 打开设置页面
            if let Some(window) = app.get_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
                // 导航到设置页面
                let _ = window.eval("window.location.hash = '#/setting'");
            }
        }
        "about" => {
            // 显示关于信息
            if let Some(window) = app.get_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
                // 可以显示关于对话框或导航到关于页面
                let _ = window.eval("console.log('关于 Qwerty Learner')");
            }
        }
        "quit" => {
            // 退出应用
            std::process::exit(0);
        }
        _ => {
            println!("未知的菜单项: {}", menu_id);
        }
    }
}

/// 更新托盘图标状态
pub fn update_tray_icon(app: &AppHandle, is_learning: bool) {
    // 根据学习状态更新托盘图标
    // 这里可以根据需要切换不同的图标
    if is_learning {
        println!("更新托盘图标为学习状态");
        // 可以设置不同的图标来表示正在学习
    } else {
        println!("更新托盘图标为空闲状态");
    }
}