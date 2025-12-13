# KeedaVault Core 文档

欢迎查阅 `keedavault-core` 的完整文档。

## 📚 文档导航

### 快速开始

- **[README](../README.md)** - 项目概述、快速开始、基础使用
  - 特性介绍
  - 平台支持
  - 安装方法
  - 基础示例（Desktop 和 iOS）

### API 文档

- **[API 完整参考](./api-reference.md)** - 所有公共 API 的详细说明
  - Vault API - 数据库操作
  - Entry API - 条目管理
  - Group API - 分组管理
  - Search API - 搜索和过滤
  - TOTP API - 一次性密码
  - Error Types - 错误类型
  - **平台差异对照表** - Desktop vs iOS

### 集成指南

#### Desktop (macOS/Windows/Ubuntu)

- **[Desktop 集成指南](./desktop-integration.md)** - Tauri 应用集成
  - 项目设置
  - Tauri Commands 实现
  - 状态管理
  - 前端集成（TypeScript/React）
  - 最佳实践

#### iOS

- **[iOS 集成指南](./ios-integration.md)** - iOS 应用集成
  - 环境准备
  - 构建 XCFramework
  - Xcode 集成
  - SwiftUI 使用示例
  - iOS 特定功能（Keychain、Face ID）
  - 故障排查

### 高级主题

- **[架构设计](./architecture.md)** - 系统架构和设计决策（待创建）
- **[安全最佳实践](./security.md)** - 安全指南和建议（待创建）
- **[性能优化](./performance.md)** - 性能调优指南（待创建）

---

## 🎯 按使用场景查找

### 我想...

#### 开始使用

- 📖 [快速开始](../README.md#快速开始)
- 🖥️ [Desktop 项目设置](./desktop-integration.md#项目设置)
- 📱 [iOS 环境准备](./ios-integration.md#环境准备)

#### 理解 API

- 📋 [完整 API 列表](./api-reference.md)
- 🔍 [Vault 操作](./api-reference.md#vault-api)
- 📝 [Entry 管理](./api-reference.md#entry-api)
- 🔐 [TOTP 使用](./api-reference.md#totp-api)

#### 集成到应用

- 🖥️ [Tauri Commands](./desktop-integration.md#tauri-commands)
- 📱 [SwiftUI 示例](./ios-integration.md#swift-使用示例)
- 🔧 [状态管理](./desktop-integration.md#状态管理)

#### 平台特定功能

- 🍎 [macOS 特性](./desktop-integration.md#最佳实践)
- 📱 [iOS Keychain](./ios-integration.md#1-keychain-集成)
- 👤 [Face ID 集成](./ios-integration.md#2-face-id--touch-id)
- 📂 [文件选择器](./ios-integration.md#3-文件选择器)

#### 解决问题

- ❌ [错误处理](./api-reference.md#error-types)
- 🐛 [iOS 故障排查](./ios-integration.md#故障排查)
- 💡 [最佳实践](./desktop-integration.md#最佳实践)

---

## 📊 API 状态

### 已实现 ✅

| 功能 | Desktop | iOS | 文档 |
|------|---------|-----|------|
| Vault 打开/创建 | ✅ | ✅ | [API](./api-reference.md#vault-api) |
| Vault 保存/锁定 | ✅ | ✅ | [API](./api-reference.md#vault-api) |
| Entry 数据结构 | ✅ | ✅ | [API](./api-reference.md#entry-api) |
| Group 数据结构 | ✅ | ✅ | [API](./api-reference.md#group-api) |
| 搜索和过滤 | ✅ | ⚠️ | [API](./api-reference.md#search-api) |

### 进行中 🚧

| 功能 | Desktop | iOS | 预计完成 |
|------|---------|-----|----------|
| Entry CRUD | 🚧 | 🚧 | Phase 1 |
| Group CRUD | 🚧 | 🚧 | Phase 1 |
| TOTP 生成 | 🚧 | 🚧 | Phase 1 |

### 计划中 📋

| 功能 | Desktop | iOS | 预计完成 |
|------|---------|-----|----------|
| 云同步 | 📋 | 📋 | Phase 4 |
| 密码生成器 | 📋 | 📋 | Phase 4 |
| 自动锁定 | 📋 | 📋 | Phase 4 |

---

## 🔍 平台对比

### Desktop (Tauri) vs iOS (UniFFI)

| 特性 | Desktop | iOS | 说明 |
|------|---------|-----|------|
| **集成方式** | 直接 Rust API | FFI (UniFFI) | Desktop 无额外开销 |
| **语言** | Rust | Swift | - |
| **类型转换** | 无需转换 | 自动生成 | UniFFI 处理类型映射 |
| **性能** | 原生 | FFI 开销 < 1μs | 实际使用中差异可忽略 |
| **异步支持** | Rust async/await | Swift async/await | 两者都支持 |
| **错误处理** | Result<T, E> | throws | Swift 使用异常 |
| **搜索功能** | ✅ 内置 | ⚠️ Swift 侧实现 | iOS 需要在 Swift 层过滤 |

### 何时使用哪个平台

#### 选择 Desktop (Tauri) 如果：

- ✅ 需要跨 macOS/Windows/Ubuntu
- ✅ 已有 Web 前端代码
- ✅ 需要最佳性能
- ✅ 团队熟悉 Rust/TypeScript

#### 选择 iOS (UniFFI) 如果：

- ✅ 需要原生 iOS 体验
- ✅ 需要 Face ID/Touch ID
- ✅ 需要 App Store 分发
- ✅ 团队熟悉 Swift/SwiftUI

---

## 📖 代码示例索引

### Desktop (Rust + TypeScript)

- [Tauri Commands 实现](./desktop-integration.md#tauri-commands)
- [状态管理](./desktop-integration.md#状态管理)
- [TypeScript API 封装](./desktop-integration.md#api-封装)
- [React 组件示例](./desktop-integration.md#react-示例)

### iOS (Swift)

- [VaultManager 实现](./ios-integration.md#基础使用)
- [SwiftUI 视图](./ios-integration.md#swiftui-视图)
- [Keychain 集成](./ios-integration.md#1-keychain-集成)
- [Face ID 认证](./ios-integration.md#2-face-id--touch-id)

---

## 🔗 外部资源

### 相关项目

- [keedavault-app](https://github.com/daocatt/keedavault-app) - Desktop 和 iOS 应用
- [keedavault](https://github.com/daocatt/keedavault) - 原始实现（参考）

### 依赖库文档

- [keepass-rs](https://docs.rs/keepass/) - KDBX 文件格式库
- [Tauri](https://tauri.app/v2/guides/) - Desktop 框架
- [UniFFI](https://mozilla.github.io/uniffi-rs/) - Rust-Swift 绑定

### 标准和规范

- [KeePass KDBX Format](https://keepass.info/help/kb/kdbx_4.html) - KDBX4 格式规范
- [TOTP RFC 6238](https://datatracker.ietf.org/doc/html/rfc6238) - TOTP 标准

---

## 🤝 贡献文档

发现文档问题或想要改进？

1. Fork 项目
2. 编辑 Markdown 文件
3. 提交 Pull Request

文档源文件位置：
- `README.md` - 项目根目录
- `docs/*.md` - 文档目录

---

## 📝 更新日志

### v0.1.0 (2025-12-14)

**文档**:
- ✅ 完整的 README
- ✅ API 参考文档
- ✅ Desktop 集成指南
- ✅ iOS 集成指南
- ✅ 平台差异对照

**代码**:
- ✅ 基础 Vault 操作
- ✅ Entry/Group 数据结构
- ✅ 搜索和过滤
- ✅ 错误处理系统

---

## 💬 获取帮助

- 📖 先查看相关文档
- 🐛 [提交 Issue](https://github.com/daocatt/keedavault-core/issues)
- 💬 [参与讨论](https://github.com/daocatt/keedavault-core/discussions)

---

**最后更新**: 2025-12-14  
**版本**: 0.1.0
