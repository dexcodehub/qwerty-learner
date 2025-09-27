#!/bin/bash

# 构建 Tauri 应用（无签名）的脚本
# 适用于开发环境或不需要签名的场景

set -e

echo "🚀 开始构建（无签名）..."

# 确保签名身份为空
unset APPLE_SIGNING_IDENTITY

# 运行构建
npm run tauri:build

# 检查构建是否成功
if [ $? -eq 0 ]; then
    echo "✅ 构建成功!"
    echo ""
    echo "📦 构建产物位置:"
    echo "  - 应用程序: src-tauri/target/release/bundle/macos/Qwerty Learner.app"
    echo "  - DMG 文件: src-tauri/target/release/bundle/dmg/"
    
    echo ""
    echo "⚠️  注意: 此应用未签名，可能在其他设备上无法正常运行"
else
    echo "❌ 构建失败"
    exit 1
fi