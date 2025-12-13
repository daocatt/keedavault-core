# KeedaVault Core

🦀 跨平台 KDBX 密码库核心库 - 使用 Rust 编写

## 📋 概述

`keedavault-core` 是 KeedaVault 密码管理器的核心库，提供安全的密码数据库管理功能，完全兼容 KeePass KDBX 格式。

### 特性

- ✅ **KDBX 支持**: 完全兼容 KeePass 数据库格式（KDBX3/KDBX4）
- 🔐 **强加密**: Argon2, ChaCha20, AES 加密
- 🔑 **TOTP 支持**: 时间基准一次性密码生成
- 🔍 **搜索过滤**: 快速条目搜索和标签过滤
- 📱 **跨平台**: 支持 Desktop (macOS/Windows/Ubuntu) 和 iOS
- 🦺 **类型安全**: Rust 内存安全和性能保证

### 平台支持

| 平台 | 集成方式 | 状态 |
|------|---------|------|
| **macOS Desktop** | Tauri (直接 Rust API) | ✅ 支持 |
| **Windows Desktop** | Tauri (直接 Rust API) | ✅ 支持 |
| **Ubuntu Desktop** | Tauri (直接 Rust API) | ✅ 支持 |
| **iOS** | UniFFI (Swift bindings) | 🚧 计划中 |

## 🚀 快速开始

### 安装

添加到你的 `Cargo.toml`:

```toml
[dependencies]
keedavault-core = "0.1.0"
```

### 基础使用

#### 🖥️ Desktop (macOS/Windows/Ubuntu) - Tauri 集成

在 Tauri 项目中直接使用 Rust API：

```rust
use keedavault_core::{Vault, VaultConfig, Entry};

// 打开现有数据库
let vault = Vault::open("my_passwords.kdbx", "master_password")?;

// 获取所有条目
let entries = vault.get_entries()?;

// 创建新数据库
let mut vault = Vault::create(
    "new_vault.kdbx",
    "master_password",
    VaultConfig::default()
)?;

// 添加条目
let mut entry = Entry::new("GitHub".to_string(), "root_group_id".to_string());
entry.username = "user@example.com".to_string();
entry.password = "secure_password".to_string();
entry.url = "https://github.com".to_string();

let entry_id = vault.add_entry(entry)?;

// 保存数据库
vault.save()?;

// 锁定数据库
vault.lock();
```

#### 📱 iOS - Swift 集成 (通过 UniFFI)

> **注意**: iOS 集成需要先编译 XCFramework，详见 [iOS 集成指南](./docs/ios-integration.md)

```swift
import KeedavaultCore

// 打开数据库
let vault = try Vault.open(path: "my_passwords.kdbx", password: "master_password")

// 获取条目
let entries = try vault.getEntries()

// 创建新条目
var entry = Entry.new(title: "GitHub", groupId: "root_group_id")
entry.username = "user@example.com"
entry.password = "secure_password"
entry.url = "https://github.com"

let entryId = try vault.addEntry(entry: entry)

// 保存
try vault.save()

// 锁定
vault.lock()
```

## 📚 API 文档

### 核心类型

#### `Vault` - 密码库

主要的密码库管理结构。

**方法**:

| 方法 | 描述 | Desktop | iOS |
|------|------|---------|-----|
| `open(path, password)` | 打开现有数据库 | ✅ | ✅ |
| `create(path, password, config)` | 创建新数据库 | ✅ | ✅ |
| `save()` | 保存数据库 | ✅ | ✅ |
| `lock()` | 锁定数据库 | ✅ | ✅ |
| `is_locked()` | 检查是否锁定 | ✅ | ✅ |
| `get_entries()` | 获取所有条目 | ✅ | ✅ |
| `get_entry(id)` | 获取单个条目 | ✅ | ✅ |
| `add_entry(entry)` | 添加条目 | ✅ | ✅ |
| `update_entry(id, entry)` | 更新条目 | ✅ | ✅ |
| `delete_entry(id)` | 删除条目 | ✅ | ✅ |
| `get_groups()` | 获取所有分组 | ✅ | ✅ |

#### `Entry` - 密码条目

表示一个密码条目。

**字段**:

```rust
pub struct Entry {
    pub id: String,              // 唯一标识符
    pub group_id: String,        // 所属分组 ID
    pub title: String,           // 标题
    pub username: String,        // 用户名
    pub password: String,        // 密码
    pub url: String,             // URL
    pub notes: String,           // 备注
    pub tags: Vec<String>,       // 标签
    pub totp_secret: Option<String>, // TOTP 密钥
    pub custom_fields: Vec<CustomField>, // 自定义字段
    pub created_at: DateTime<Utc>,   // 创建时间
    pub modified_at: DateTime<Utc>,  // 修改时间
    pub accessed_at: DateTime<Utc>,  // 访问时间
    pub expires_at: Option<DateTime<Utc>>, // 过期时间
    pub is_favorite: bool,       // 是否收藏
}
```

**方法**:

| 方法 | 描述 | Desktop | iOS |
|------|------|---------|-----|
| `new(title, group_id)` | 创建新条目 | ✅ | ✅ |
| `touch()` | 更新修改时间 | ✅ | ✅ |
| `mark_accessed()` | 标记为已访问 | ✅ | ✅ |
| `is_expired()` | 检查是否过期 | ✅ | ✅ |

#### `Group` - 分组

表示一个分组/文件夹。

**字段**:

```rust
pub struct Group {
    pub id: String,              // 唯一标识符
    pub parent_id: Option<String>, // 父分组 ID
    pub name: String,            // 名称
    pub icon_id: u32,            // 图标 ID
    pub notes: String,           // 备注
    pub is_recycle_bin: bool,    // 是否为回收站
    pub is_expanded: bool,       // 是否展开
}
```

**方法**:

| 方法 | 描述 | Desktop | iOS |
|------|------|---------|-----|
| `new(name, parent_id)` | 创建新分组 | ✅ | ✅ |
| `new_recycle_bin()` | 创建回收站 | ✅ | ✅ |
| `is_root()` | 是否为根分组 | ✅ | ✅ |

#### `VaultConfig` - 数据库配置

配置数据库的加密参数。

```rust
pub struct VaultConfig {
    pub kdf_iterations: u64,      // KDF 迭代次数
    pub argon2_memory: u64,       // Argon2 内存 (KB)
    pub argon2_parallelism: u32,  // Argon2 并行度
}
```

**默认值**:
- `kdf_iterations`: 2
- `argon2_memory`: 65536 (64 MB)
- `argon2_parallelism`: 2

### 搜索和过滤

```rust
use keedavault_core::search;

// 全文搜索
let results = search::search_entries(&entries, "github");

// 按标签过滤
let tagged = search::filter_by_tag(&entries, "work");

// 获取收藏
let favorites = search::get_favorites(&entries);
```

| 函数 | 描述 | Desktop | iOS |
|------|------|---------|-----|
| `search_entries(entries, query)` | 全文搜索 | ✅ | ✅ |
| `filter_by_tag(entries, tag)` | 标签过滤 | ✅ | ✅ |
| `get_favorites(entries)` | 获取收藏 | ✅ | ✅ |

### TOTP (计划中)

```rust
use keedavault_core::totp;

// 生成 TOTP 代码
let code = totp::generate_totp("JBSWY3DPEHPK3PXP")?;

// 验证 TOTP 代码
let is_valid = totp::validate_totp("JBSWY3DPEHPK3PXP", "123456")?;
```

### 错误处理

```rust
use keedavault_core::{VaultError, Result};

match Vault::open("vault.kdbx", "wrong_password") {
    Ok(vault) => { /* 成功 */ },
    Err(VaultError::InvalidPassword) => {
        println!("密码错误");
    },
    Err(VaultError::OpenError(msg)) => {
        println!("打开失败: {}", msg);
    },
    Err(e) => {
        println!("其他错误: {}", e);
    }
}
```

**错误类型**:

- `OpenError(String)` - 打开数据库失败
- `SaveError(String)` - 保存数据库失败
- `InvalidPassword` - 密码错误
- `VaultLocked` - 数据库已锁定
- `EntryNotFound(String)` - 条目不存在
- `GroupNotFound(String)` - 分组不存在
- `EncryptionError(String)` - 加密错误
- `DecryptionError(String)` - 解密错误

## 🔧 构建

### Desktop 构建

```bash
# 标准构建
cargo build --release

# 运行测试
cargo test

# 生成文档
cargo doc --open
```

### iOS 构建

> **注意**: iOS 构建需要 macOS 环境和 Xcode

```bash
# 安装 iOS 目标
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim

# 构建 iOS 库（启用 UniFFI）
cargo build --target aarch64-apple-ios --release --features uniffi

# 生成 Swift 绑定
cargo run --bin uniffi-bindgen generate src/keedavault.udl --language swift

# 打包 XCFramework
./scripts/build-xcframework.sh
```

详细步骤请参考 [iOS 集成指南](./docs/ios-integration.md)

## 🎯 平台差异说明

### Desktop (Tauri) vs iOS (UniFFI)

| 特性 | Desktop (Tauri) | iOS (UniFFI) |
|------|----------------|--------------|
| **语言** | Rust | Swift |
| **集成方式** | 直接调用 Rust API | 通过 FFI 调用 |
| **类型转换** | 无需转换 | 自动生成 Swift 类型 |
| **性能** | 原生性能 | FFI 开销（极小） |
| **异步支持** | Rust async/await | Swift async/await |
| **错误处理** | Rust Result | Swift throws |
| **内存管理** | Rust 所有权 | ARC + Rust 所有权 |

### 特定平台功能

#### macOS Desktop 特有

```rust
// Tauri Command 示例
#[tauri::command]
fn unlock_vault(path: String, password: String) -> Result<VaultHandle, String> {
    let vault = Vault::open(&path, &password)
        .map_err(|e| e.to_string())?;
    Ok(VaultHandle::new(vault))
}
```

#### iOS 特有

```swift
// 使用 Keychain 存储主密码
import Security

func savePasswordToKeychain(password: String) {
    let data = password.data(using: .utf8)!
    let query: [String: Any] = [
        kSecClass as String: kSecClassGenericPassword,
        kSecAttrAccount as String: "vault_password",
        kSecValueData as String: data
    ]
    SecItemAdd(query as CFDictionary, nil)
}

// 使用 Face ID 解锁
import LocalAuthentication

func unlockWithBiometrics() async throws -> Vault {
    let context = LAContext()
    try await context.evaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, 
                                     localizedReason: "解锁密码库")
    let password = getPasswordFromKeychain()
    return try Vault.open(path: vaultPath, password: password)
}
```

## 📖 详细文档

- [API 完整参考](./docs/api-reference.md)
- [Desktop 集成指南](./docs/desktop-integration.md)
- [iOS 集成指南](./docs/ios-integration.md)
- [架构设计](./docs/architecture.md)
- [安全最佳实践](./docs/security.md)

## 🧪 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_create_vault

# 带输出的测试
cargo test -- --nocapture

# 性能测试
cargo bench
```

**当前测试覆盖**:
- ✅ Vault 创建和锁定
- ✅ Entry 创建和过期检查
- ✅ Group 创建和层级
- ✅ 搜索功能
- ✅ TOTP 占位符

## 📊 项目状态

### 已完成 ✅

- [x] 项目初始化
- [x] 基础数据结构 (Entry, Group, Vault)
- [x] 错误处理系统
- [x] KDBX 文件打开/创建/保存
- [x] 搜索和过滤功能
- [x] 单元测试

### 进行中 🚧

- [ ] Entry/Group CRUD 完整实现
- [ ] TOTP 生成和验证
- [ ] UniFFI iOS 绑定
- [ ] 性能优化

### 计划中 📋

- [ ] 云同步支持 (WebDAV/S3)
- [ ] 密码强度检测
- [ ] 自动锁定
- [ ] 安全审计日志

## 🤝 贡献

这是 KeedaVault 项目的一部分。欢迎贡献！

## 📄 许可证

MIT License

## 🔗 相关项目

- [keedavault-app](https://github.com/daocatt/keedavault-app) - Desktop 和 iOS 应用
- [keedavault](https://github.com/daocatt/keedavault) - 原始实现（参考）

## 📞 支持

- 文档: [docs/](./docs/)
- Issues: [GitHub Issues](https://github.com/daocatt/keedavault-core/issues)
- 讨论: [GitHub Discussions](https://github.com/daocatt/keedavault-core/discussions)
