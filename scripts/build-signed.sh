#!/bin/bash

# 构建并签名 Tauri 应用的脚本
# 使用方法: ./scripts/build-signed.sh [signing_identity]

set -e

# 检查是否提供了签名身份参数
if [ -z "$1" ]; then
    echo "错误: 请提供签名身份"
    echo "使用方法: $0 \"Apple Development: Your Name (XXXXXXXXXX)\""
    echo ""
    echo "可用的签名身份:"
    security find-identity -v -p codesigning
    exit 1
fi

SIGNING_IDENTITY="$1"

echo "🔧 使用签名身份: $SIGNING_IDENTITY"

# 设置环境变量
export APPLE_SIGNING_IDENTITY="$SIGNING_IDENTITY"

# 运行构建
echo "🚀 开始构建..."
npm run tauri:build

# 检查构建是否成功
if [ $? -eq 0 ]; then
    echo "✅ 构建成功!"
    echo ""
    echo "📦 构建产物位置:"
    echo "  - 应用程序: src-tauri/target/release/bundle/macos/Qwerty Learner.app"
    echo "  - DMG 文件: src-tauri/target/release/bundle/dmg/"
    
    # 验证签名
    echo ""
    echo "🔍 验证应用程序签名..."
    codesign -dv "src-tauri/target/release/bundle/macos/Qwerty Learner.app"
else
    echo "❌ 构建失败"
    exit 1
fi