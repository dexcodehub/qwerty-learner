import { sendNotification } from '@tauri-apps/api/notification'
import { platform } from '@tauri-apps/api/os'
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from '@tauri-apps/api/window'

// 检查是否在Tauri环境中运行
export const isTauri = () => {
  return typeof window !== 'undefined' && '__TAURI__' in window
}

// 系统信息接口
export interface SystemInfo {
  platform: string
  version: string
  arch: string
}

// 获取应用数据目录
export const getAppDataDir = async (): Promise<string> => {
  if (!isTauri()) return ''
  try {
    return await invoke<string>('get_app_data_dir')
  } catch (error) {
    console.error('获取应用数据目录失败:', error)
    return ''
  }
}

// 保存用户数据
export const saveUserData = async (filename: string, data: string): Promise<boolean> => {
  if (!isTauri()) return false
  try {
    await invoke('save_user_data', { filename, data })
    return true
  } catch (error) {
    console.error('保存用户数据失败:', error)
    return false
  }
}

// 加载用户数据
export const loadUserData = async (filename: string): Promise<string> => {
  if (!isTauri()) return ''
  try {
    return await invoke<string>('load_user_data', { filename })
  } catch (error) {
    console.error('加载用户数据失败:', error)
    return ''
  }
}

// 导出数据到文件
export const exportDataToFile = async (filePath: string, data: string): Promise<boolean> => {
  if (!isTauri()) return false
  try {
    await invoke('export_data_to_file', { filePath, data })
    return true
  } catch (error) {
    console.error('导出数据失败:', error)
    return false
  }
}

// 从文件导入数据
export const importDataFromFile = async (filePath: string): Promise<string> => {
  if (!isTauri()) return ''
  try {
    return await invoke<string>('import_data_from_file', { filePath })
  } catch (error) {
    console.error('导入数据失败:', error)
    return ''
  }
}

// 显示系统通知
export const showDesktopNotification = async (title: string, body: string): Promise<boolean> => {
  if (!isTauri()) return false
  try {
    await invoke('show_notification', { title, body })
    return true
  } catch (error) {
    console.error('显示通知失败:', error)
    return false
  }
}

// 切换窗口可见性
export const toggleWindowVisibility = async (): Promise<boolean> => {
  if (!isTauri()) return false
  try {
    await invoke('toggle_window_visibility')
    return true
  } catch (error) {
    console.error('切换窗口可见性失败:', error)
    return false
  }
}

// 设置窗口置顶
export const setWindowAlwaysOnTop = async (alwaysOnTop: boolean): Promise<boolean> => {
  if (!isTauri()) return false
  try {
    await invoke('set_window_always_on_top', { alwaysOnTop })
    return true
  } catch (error) {
    console.error('设置窗口置顶失败:', error)
    return false
  }
}

// 获取系统信息
export const getSystemInfo = async (): Promise<SystemInfo | null> => {
  if (!isTauri()) return null
  try {
    return await invoke<SystemInfo>('get_system_info')
  } catch (error) {
    console.error('获取系统信息失败:', error)
    return null
  }
}

// 窗口控制函数
export const windowControls = {
  // 最小化窗口
  minimize: async () => {
    if (!isTauri()) return false
    try {
      await appWindow.minimize()
      return true
    } catch (error) {
      console.error('最小化窗口失败:', error)
      return false
    }
  },

  // 最大化窗口
  maximize: async () => {
    if (!isTauri()) return false
    try {
      await appWindow.maximize()
      return true
    } catch (error) {
      console.error('最大化窗口失败:', error)
      return false
    }
  },

  // 关闭窗口
  close: async () => {
    if (!isTauri()) return false
    try {
      await appWindow.close()
      return true
    } catch (error) {
      console.error('关闭窗口失败:', error)
      return false
    }
  },

  // 切换全屏
  toggleFullscreen: async () => {
    if (!isTauri()) return false
    try {
      const isFullscreen = await appWindow.isFullscreen()
      await appWindow.setFullscreen(!isFullscreen)
      return true
    } catch (error) {
      console.error('切换全屏失败:', error)
      return false
    }
  },

  // 居中窗口
  center: async () => {
    if (!isTauri()) return false
    try {
      await appWindow.center()
      return true
    } catch (error) {
      console.error('居中窗口失败:', error)
      return false
    }
  },
}

// 获取平台信息
export const getPlatform = async (): Promise<string> => {
  if (!isTauri()) return 'web'
  try {
    return await platform()
  } catch (error) {
    console.error('获取平台信息失败:', error)
    return 'unknown'
  }
}

// 检查是否为macOS
export const isMacOS = async (): Promise<boolean> => {
  const platformName = await getPlatform()
  return platformName === 'darwin'
}

// 桌面端特有的学习数据管理
export const desktopDataManager = {
  // 保存学习记录
  saveLearningRecord: async (record: any): Promise<boolean> => {
    return await saveUserData('learning_records.json', JSON.stringify(record))
  },

  // 加载学习记录
  loadLearningRecord: async (): Promise<any> => {
    const data = await loadUserData('learning_records.json')
    try {
      return data ? JSON.parse(data) : null
    } catch (error) {
      console.error('解析学习记录失败:', error)
      return null
    }
  },

  // 保存用户设置
  saveSettings: async (settings: any): Promise<boolean> => {
    return await saveUserData('settings.json', JSON.stringify(settings))
  },

  // 加载用户设置
  loadSettings: async (): Promise<any> => {
    const data = await loadUserData('settings.json')
    try {
      return data ? JSON.parse(data) : null
    } catch (error) {
      console.error('解析用户设置失败:', error)
      return null
    }
  },
}
