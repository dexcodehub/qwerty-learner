# macOS 代码签名构建指南

本文档说明如何为 Qwerty Learner 应用配置和使用代码签名。

## 前提条件

1. **Apple 开发者账户**: 需要有效的 Apple 开发者账户
2. **开发者证书**: 在 Keychain Access 中安装相应的开发者证书
3. **create-dmg 工具**: 用于创建 DMG 安装包

```bash
# 安装 create-dmg
brew install create-dmg
```

## 查看可用的签名身份

```bash
security find-identity -v -p codesigning
```

输出示例：
```
1) 1234567890ABCDEF1234567890ABCDEF12345678 "Apple Development: Your Name (XXXXXXXXXX)"
2) ABCDEF1234567890ABCDEF1234567890ABCDEF12 "Apple Development: Another Name (YYYYYYYYYY)"
```

## 构建方式

### 方式一：使用构建脚本（推荐）

#### 签名构建
```bash
# 使用完整的签名身份名称
./scripts/build-signed.sh "Apple Development: Your Name (XXXXXXXXXX)"
```

#### 无签名构建（开发环境）
```bash
./scripts/build-unsigned.sh
```

### 方式二：使用环境变量

```bash
# 设置环境变量
export APPLE_SIGNING_IDENTITY="Apple Development: Your Name (XXXXXXXXXX)"

# 运行构建
npm run tauri:build
```

### 方式三：临时修改配置文件（不推荐）

直接在 `src-tauri/tauri.conf.json` 中修改 `signingIdentity` 字段，但这种方式会将敏感信息提交到版本控制中。

## 证书类型说明

- **Apple Development**: 用于开发和测试，只能在开发者设备和注册的测试设备上运行
- **Developer ID Application**: 用于 App Store 外分发，可在任何 macOS 设备上运行
- **Apple Distribution**: 用于 App Store 分发

## 构建产物

成功构建后，产物位于：

- **应用程序**: `src-tauri/target/release/bundle/macos/Qwerty Learner.app`
- **DMG 安装包**: `src-tauri/target/release/bundle/dmg/`

## 验证签名

```bash
# 验证应用程序签名
codesign -dv "src-tauri/target/release/bundle/macos/Qwerty Learner.app"

# 验证签名有效性
codesign --verify --verbose "src-tauri/target/release/bundle/macos/Qwerty Learner.app"

# 验证 DMG 签名（如果已签名）
codesign -dv "path/to/your.dmg"
```

## 故障排除

### 常见问题

1. **"Resource busy" 错误**
   ```bash
   # 强制卸载挂载的磁盘映像
   hdiutil detach -force "/Volumes/Qwerty Learner"
   ```

2. **找不到签名身份**
   - 确保证书已正确安装在 Keychain Access 中
   - 检查证书是否过期
   - 使用 `security find-identity -v -p codesigning` 确认可用身份

3. **构建失败**
   - 检查 Xcode Command Line Tools 是否已安装
   - 确保 `create-dmg` 工具已安装
   - 查看详细的构建日志

## 安全注意事项

- ❌ 不要将签名身份硬编码在配置文件中
- ✅ 使用环境变量或构建脚本传递签名身份
- ✅ 将敏感信息添加到 `.gitignore` 中
- ✅ 定期更新开发者证书

## CI/CD 集成

在 CI/CD 环境中，可以通过以下方式配置签名：

```yaml
# GitHub Actions 示例
- name: Build with signing
  env:
    APPLE_SIGNING_IDENTITY: ${{ secrets.APPLE_SIGNING_IDENTITY }}
  run: npm run tauri:build
```

确保在 CI/CD 系统中安全地存储签名身份信息。