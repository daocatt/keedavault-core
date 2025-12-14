# API 完整参考

本文档详细说明了 `keedavault-core` 的所有公共 API。

## 目录

- [Vault API](#vault-api)
- [Entry API](#entry-api)
- [Group API](#group-api)
- [Search API](#search-api)
- [TOTP API](#totp-api)
- [Error Types](#error-types)
- [平台差异](#平台差异)

---

## Vault API

### `Vault::open`

打开现有的 KDBX 数据库。

**签名**:

```rust
pub fn open<P: AsRef<Path>>(path: P, password: &str) -> Result<Self>
```

**参数**:
- `path`: 数据库文件路径
- `password`: 主密码

**返回**: `Result<Vault, VaultError>`

**示例**:

```rust
// Desktop (Rust)
let vault = Vault::open("my_vault.kdbx", "master_password")?;
```

```swift
// iOS (Swift)
let vault = try Vault.open(path: "my_vault.kdbx", password: "master_password")
```

**错误**:
- `VaultError::OpenError` - 文件不存在或无法读取
- `VaultError::InvalidPassword` - 密码错误
- `VaultError::KeePassError` - KDBX 格式错误

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Vault::create`

创建新的 KDBX 数据库。

**签名**:

```rust
pub fn create<P: AsRef<Path>>(
    path: P,
    password: &str,
    config: VaultConfig,
) -> Result<Self>
```

**参数**:
- `path`: 数据库文件路径
- `password`: 主密码
- `config`: 数据库配置（KDF 参数）

**返回**: `Result<Vault, VaultError>`

**示例**:

```rust
// Desktop (Rust)
let vault = Vault::create(
    "new_vault.kdbx",
    "master_password",
    VaultConfig::default()
)?;
```

```swift
// iOS (Swift)
let config = VaultConfig.default()
let vault = try Vault.create(
    path: "new_vault.kdbx",
    password: "master_password",
    config: config
)
```

**错误**:
- `VaultError::SaveError` - 无法创建文件
- `VaultError::KeePassError` - 数据库创建失败

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Vault::save`

保存数据库到磁盘。

**签名**:

```rust
pub fn save(&mut self) -> Result<()>
```

**返回**: `Result<(), VaultError>`

**示例**:

```rust
// Desktop (Rust)
vault.save()?;
```

```swift
// iOS (Swift)
try vault.save()
```

**错误**:
- `VaultError::VaultLocked` - 数据库已锁定
- `VaultError::SaveError` - 保存失败

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Vault::lock`

锁定数据库，清除内存中的敏感数据。

**签名**:

```rust
pub fn lock(&mut self)
```

**示例**:

```rust
// Desktop (Rust)
vault.lock();
```

```swift
// iOS (Swift)
vault.lock()
```

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Vault::is_locked`

检查数据库是否已锁定。

**签名**:

```rust
pub fn is_locked(&self) -> bool
```

**返回**: `bool` - 是否锁定

**示例**:

```rust
// Desktop (Rust)
if vault.is_locked() {
    println!("数据库已锁定");
}
```

```swift
// iOS (Swift)
if vault.isLocked() {
    print("数据库已锁定")
}
```

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Vault::get_entries`

获取所有密码条目。

**签名**:

```rust
pub fn get_entries(&self) -> Result<Vec<Entry>>
```

**返回**: `Result<Vec<Entry>, VaultError>`

**示例**:

```rust
// Desktop (Rust)
let entries = vault.get_entries()?;
for entry in entries {
    println!("{}: {}", entry.title, entry.username);
}
```

```swift
// iOS (Swift)
let entries = try vault.getEntries()
for entry in entries {
    print("\(entry.title): \(entry.username)")
}
```

**错误**:
- `VaultError::VaultLocked` - 数据库已锁定

**平台支持**: ✅ Desktop | ✅ iOS

**状态**: 🚧 当前返回空数组，Phase 1 将实现完整功能

---

### `Vault::get_entry`

根据 ID 获取单个条目。

**签名**:

```rust
pub fn get_entry(&self, id: &str) -> Result<Entry>
```

**参数**:
- `id`: 条目 ID

**返回**: `Result<Entry, VaultError>`

**示例**:

```rust
// Desktop (Rust)
let entry = vault.get_entry("entry-uuid-123")?;
```

```swift
// iOS (Swift)
let entry = try vault.getEntry(id: "entry-uuid-123")
```

**错误**:
- `VaultError::VaultLocked` - 数据库已锁定
- `VaultError::EntryNotFound` - 条目不存在

**平台支持**: ✅ Desktop | ✅ iOS

**状态**: 🚧 Phase 1 实现中

---

### `Vault::add_entry`

添加新条目到数据库。

**签名**:

```rust
pub fn add_entry(&mut self, entry: Entry) -> Result<String>
```

**参数**:
- `entry`: 要添加的条目

**返回**: `Result<String, VaultError>` - 新条目的 ID

**示例**:

```rust
// Desktop (Rust)
let mut entry = Entry::new("GitHub".to_string(), "root".to_string());
entry.username = "user@example.com".to_string();
entry.password = "secure_password".to_string();

let entry_id = vault.add_entry(entry)?;
vault.save()?;
```

```swift
// iOS (Swift)
var entry = Entry.new(title: "GitHub", groupId: "root")
entry.username = "user@example.com"
entry.password = "secure_password"

let entryId = try vault.addEntry(entry: entry)
try vault.save()
```

**错误**:
- `VaultError::VaultLocked` - 数据库已锁定

**平台支持**: ✅ Desktop | ✅ iOS

**状态**: 🚧 Phase 1 实现中

---

### `Vault::update_entry`

更新现有条目。

**签名**:

```rust
pub fn update_entry(&mut self, id: &str, entry: Entry) -> Result<()>
```

**参数**:
- `id`: 要更新的条目 ID
- `entry`: 新的条目数据

**返回**: `Result<(), VaultError>`

**示例**:

```rust
// Desktop (Rust)
let mut entry = vault.get_entry("entry-id")?;
entry.password = "new_password".to_string();
entry.touch(); // 更新修改时间

vault.update_entry("entry-id", entry)?;
vault.save()?;
```

```swift
// iOS (Swift)
var entry = try vault.getEntry(id: "entry-id")
entry.password = "new_password"
entry.touch()

try vault.updateEntry(id: "entry-id", entry: entry)
try vault.save()
```

**错误**:
- `VaultError::VaultLocked` - 数据库已锁定
- `VaultError::EntryNotFound` - 条目不存在

**平台支持**: ✅ Desktop | ✅ iOS

**状态**: 🚧 Phase 1 实现中

---

### `Vault::delete_entry`

删除条目。

**签名**:

```rust
pub fn delete_entry(&mut self, id: &str) -> Result<()>
```

**参数**:
- `id`: 要删除的条目 ID

**返回**: `Result<(), VaultError>`

**示例**:

```rust
// Desktop (Rust)
vault.delete_entry("entry-id")?;
vault.save()?;
```

```swift
// iOS (Swift)
try vault.deleteEntry(id: "entry-id")
try vault.save()
```

**错误**:
- `VaultError::VaultLocked` - 数据库已锁定
- `VaultError::EntryNotFound` - 条目不存在

**平台支持**: ✅ Desktop | ✅ iOS

**状态**: 🚧 Phase 1 实现中

---

### `Vault::get_groups`

获取所有分组。

**签名**:

```rust
pub fn get_groups(&self) -> Result<Vec<Group>>
```

**返回**: `Result<Vec<Group>, VaultError>`

**示例**:

```rust
// Desktop (Rust)
let groups = vault.get_groups()?;
```

```swift
// iOS (Swift)
let groups = try vault.getGroups()
```

**错误**:
- `VaultError::VaultLocked` - 数据库已锁定

**平台支持**: ✅ Desktop | ✅ iOS

**状态**: 🚧 Phase 1 实现中

---

## Entry API

### `Entry::new`

创建新的密码条目。

**签名**:

```rust
pub fn new(title: String, group_id: String) -> Self
```

**参数**:
- `title`: 条目标题
- `group_id`: 所属分组 ID

**返回**: `Entry`

**示例**:

```rust
// Desktop (Rust)
let entry = Entry::new("My Account".to_string(), "group-id".to_string());
```

```swift
// iOS (Swift)
let entry = Entry.new(title: "My Account", groupId: "group-id")
```

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Entry::touch`

更新条目的修改时间戳。

**签名**:

```rust
pub fn touch(&mut self)
```

**示例**:

```rust
// Desktop (Rust)
entry.touch();
```

```swift
// iOS (Swift)
entry.touch()
```

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Entry::mark_accessed`

标记条目为已访问，更新访问时间戳。

**签名**:

```rust
pub fn mark_accessed(&mut self)
```

**示例**:

```rust
// Desktop (Rust)
entry.mark_accessed();
```

```swift
// iOS (Swift)
entry.markAccessed()
```

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Entry::is_expired`

检查条目是否已过期。

**签名**:

```rust
pub fn is_expired(&self) -> bool
```

**返回**: `bool` - 是否过期

**示例**:

```rust
// Desktop (Rust)
if entry.is_expired() {
    println!("此条目已过期");
}
```

```swift
// iOS (Swift)
if entry.isExpired() {
    print("此条目已过期")
}
```

**平台支持**: ✅ Desktop | ✅ iOS

---

## Group API

### `Group::new`

创建新分组。

**签名**:

```rust
pub fn new(name: String, parent_id: Option<String>) -> Self
```

**参数**:
- `name`: 分组名称
- `parent_id`: 父分组 ID（None 表示根分组）

**返回**: `Group`

**示例**:

```rust
// Desktop (Rust)
let group = Group::new("Work".to_string(), None);
let subgroup = Group::new("Projects".to_string(), Some(group.id.clone()));
```

```swift
// iOS (Swift)
let group = Group.new(name: "Work", parentId: nil)
let subgroup = Group.new(name: "Projects", parentId: group.id)
```

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Group::new_recycle_bin`

创建回收站分组。

**签名**:

```rust
pub fn new_recycle_bin() -> Self
```

**返回**: `Group`

**示例**:

```rust
// Desktop (Rust)
let recycle_bin = Group::new_recycle_bin();
```

```swift
// iOS (Swift)
let recycleBin = Group.newRecycleBin()
```

**平台支持**: ✅ Desktop | ✅ iOS

---

### `Group::is_root`

检查是否为根分组。

**签名**:

```rust
pub fn is_root(&self) -> bool
```

**返回**: `bool` - 是否为根分组

**示例**:

```rust
// Desktop (Rust)
if group.is_root() {
    println!("这是根分组");
}
```

```swift
// iOS (Swift)
if group.isRoot() {
    print("这是根分组")
}
```

**平台支持**: ✅ Desktop | ✅ iOS

---

## Search API

### `search_entries`

在条目列表中进行全文搜索。

**签名**:

```rust
pub fn search_entries(entries: &[Entry], query: &str) -> Vec<Entry>
```

**参数**:
- `entries`: 条目列表
- `query`: 搜索关键词

**返回**: `Vec<Entry>` - 匹配的条目

**搜索范围**: title, username, url, notes

**示例**:

```rust
// Desktop (Rust)
use keedavault_core::search;

let entries = vault.get_entries()?;
let results = search::search_entries(&entries, "github");
```

```swift
// iOS (Swift - 需要在 Swift 侧实现)
let entries = try vault.getEntries()
let results = entries.filter { entry in
    entry.title.lowercased().contains("github") ||
    entry.username.lowercased().contains("github") ||
    entry.url.lowercased().contains("github")
}
```

**平台支持**: ✅ Desktop | ⚠️ iOS (需要 Swift 侧实现)

---

### `filter_by_tag`

按标签过滤条目。

**签名**:

```rust
pub fn filter_by_tag(entries: &[Entry], tag: &str) -> Vec<Entry>
```

**参数**:
- `entries`: 条目列表
- `tag`: 标签名

**返回**: `Vec<Entry>` - 包含该标签的条目

**示例**:

```rust
// Desktop (Rust)
let work_entries = search::filter_by_tag(&entries, "work");
```

```swift
// iOS (Swift - 需要在 Swift 侧实现)
let workEntries = entries.filter { $0.tags.contains("work") }
```

**平台支持**: ✅ Desktop | ⚠️ iOS (需要 Swift 侧实现)

---

### `get_favorites`

获取所有收藏的条目。

**签名**:

```rust
pub fn get_favorites(entries: &[Entry]) -> Vec<Entry>
```

**参数**:
- `entries`: 条目列表

**返回**: `Vec<Entry>` - 收藏的条目

**示例**:

```rust
// Desktop (Rust)
let favorites = search::get_favorites(&entries);
```

```swift
// iOS (Swift - 需要在 Swift 侧实现)
let favorites = entries.filter { $0.isFavorite }
```

**平台支持**: ✅ Desktop | ⚠️ iOS (需要 Swift 侧实现)

---

## TOTP API

> **状态**: 🚧 Phase 1 实现中

### `generate_totp`

生成 TOTP 代码。

**签名**:

```rust
pub fn generate_totp(secret: &str) -> Result<String>
```

**参数**:
- `secret`: Base32 编码的 TOTP 密钥

**返回**: `Result<String, VaultError>` - 6 位数字代码

**示例**:

```rust
// Desktop (Rust)
use keedavault_core::totp;

let code = totp::generate_totp("JBSWY3DPEHPK3PXP")?;
println!("TOTP: {}", code);
```

```swift
// iOS (Swift)
let code = try generateTotp(secret: "JBSWY3DPEHPK3PXP")
print("TOTP: \(code)")
```

**平台支持**: 🚧 Desktop | 🚧 iOS

---

### `validate_totp`

验证 TOTP 代码。

**签名**:

```rust
pub fn validate_totp(secret: &str, code: &str) -> Result<bool>
```

**参数**:
- `secret`: Base32 编码的 TOTP 密钥
- `code`: 要验证的代码

**返回**: `Result<bool, VaultError>` - 是否有效

**示例**:

```rust
// Desktop (Rust)
let is_valid = totp::validate_totp("JBSWY3DPEHPK3PXP", "123456")?;
```

```swift
// iOS (Swift)
let isValid = try validateTotp(secret: "JBSWY3DPEHPK3PXP", code: "123456")
```

**平台支持**: 🚧 Desktop | 🚧 iOS

---

## Error Types

### `VaultError`

所有操作的错误类型。

**变体**:

| 错误 | 描述 | 常见原因 |
|------|------|---------|
| `OpenError(String)` | 打开数据库失败 | 文件不存在、权限不足 |
| `SaveError(String)` | 保存数据库失败 | 磁盘空间不足、权限不足 |
| `InvalidPassword` | 密码错误 | 用户输入错误的密码 |
| `VaultLocked` | 数据库已锁定 | 尝试操作已锁定的数据库 |
| `EntryNotFound(String)` | 条目不存在 | 使用了无效的条目 ID |
| `GroupNotFound(String)` | 分组不存在 | 使用了无效的分组 ID |
| `InvalidEntry(String)` | 无效的条目数据 | 数据验证失败 |
| `EncryptionError(String)` | 加密错误 | 加密过程失败 |
| `DecryptionError(String)` | 解密错误 | 解密过程失败 |
| `IoError` | IO 错误 | 文件系统错误 |
| `SerializationError` | 序列化错误 | JSON 序列化失败 |
| `KeePassError(String)` | KeePass 库错误 | KDBX 格式问题 |
| `Unknown(String)` | 未知错误 | 其他错误 |

**示例**:

```rust
// Desktop (Rust)
match vault.open("vault.kdbx", "password") {
    Ok(v) => { /* 成功 */ },
    Err(VaultError::InvalidPassword) => {
        eprintln!("密码错误");
    },
    Err(e) => {
        eprintln!("错误: {}", e);
    }
}
```

```swift
// iOS (Swift)
do {
    let vault = try Vault.open(path: "vault.kdbx", password: "password")
} catch VaultError.invalidPassword {
    print("密码错误")
} catch {
    print("错误: \(error)")
}
```

---

## 平台差异

### Desktop (Tauri) 特性

#### 直接 Rust API 调用

```rust
// 在 Tauri Command 中直接使用
#[tauri::command]
async fn unlock_vault(
    path: String,
    password: String,
    state: State<'_, AppState>
) -> Result<VaultInfo, String> {
    let vault = Vault::open(&path, &password)
        .map_err(|e| e.to_string())?;
    
    let info = VaultInfo {
        path: vault.path().to_string_lossy().to_string(),
        entry_count: vault.get_entries()?.len(),
    };
    
    state.add_vault(vault);
    Ok(info)
}
```

#### 异步支持

```rust
// Desktop 可以使用 Rust 的 async/await
use tokio::fs;

async fn save_vault_async(vault: &mut Vault) -> Result<()> {
    vault.save()?;
    Ok(())
}
```

### iOS (UniFFI) 特性

#### Swift 类型映射

| Rust 类型 | Swift 类型 |
|-----------|-----------|
| `String` | `String` |
| `bool` | `Bool` |
| `u32` | `UInt32` |
| `u64` | `UInt64` |
| `Vec<T>` | `[T]` |
| `Option<T>` | `T?` |
| `Result<T, E>` | `throws` |
| `DateTime<Utc>` | `Date` |

#### iOS 特定集成

```swift
// 使用 Keychain 存储密码
import Security

class VaultManager {
    func savePassword(_ password: String, for vaultPath: String) {
        let data = password.data(using: .utf8)!
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: "com.bsdev.keedavault",
            kSecAttrAccount as String: vaultPath,
            kSecValueData as String: data
        ]
        SecItemAdd(query as CFDictionary, nil)
    }
    
    func getPassword(for vaultPath: String) -> String? {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: "com.bsdev.keedavault",
            kSecAttrAccount as String: vaultPath,
            kSecReturnData as String: true
        ]
        
        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)
        
        guard status == errSecSuccess,
              let data = result as? Data,
              let password = String(data: data, encoding: .utf8) else {
            return nil
        }
        
        return password
    }
}

// 使用 Face ID 解锁
import LocalAuthentication

func unlockWithBiometrics(vaultPath: String) async throws -> Vault {
    let context = LAContext()
    var error: NSError?
    
    guard context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error) else {
        throw VaultError.biometricsNotAvailable
    }
    
    try await context.evaluatePolicy(
        .deviceOwnerAuthenticationWithBiometrics,
        localizedReason: "解锁密码库"
    )
    
    guard let password = getPassword(for: vaultPath) else {
        throw VaultError.passwordNotFound
    }
    
    return try Vault.open(path: vaultPath, password: password)
}
```

#### 内存管理

```swift
// UniFFI 对象需要手动管理生命周期
class VaultViewController: UIViewController {
    private var vault: Vault?
    
    override func viewDidDisappear(_ animated: Bool) {
        super.viewDidDisappear(animated)
        
        // 锁定并释放 vault
        vault?.lock()
        vault = nil
    }
}
```

---

## 性能考虑

### Desktop

- ✅ 零开销抽象
- ✅ 原生性能
- ✅ 直接内存访问

### iOS

- ⚠️ FFI 调用开销（通常 < 1μs）
- ⚠️ 数据需要跨 FFI 边界复制
- ✅ 批量操作性能良好

**建议**:
- 批量获取数据而不是逐个获取
- 在 Swift 侧缓存常用数据
- 避免频繁的小型 FFI 调用

---

## 版本兼容性

| keedavault-core | keepass-rs | Rust | Swift | iOS |
|----------------|------------|------|-------|-----|
| 0.1.0 | 0.8.16 | 1.70+ | 5.9+ | 15.0+ |

---

## 更新日志

### v0.1.0 (2025-12-14)

**新增**:
- ✅ 基础 Vault 操作 (open, create, save, lock)
- ✅ Entry 和 Group 数据结构
- ✅ 搜索和过滤功能
- ✅ 错误处理系统

**进行中**:
- 🚧 Entry/Group CRUD 完整实现
- 🚧 TOTP 支持
- 🚧 iOS UniFFI 绑定

---

## 相关文档

- [README](../README.md)
- [Desktop 集成指南](./desktop-integration.md)
- [iOS 集成指南](./ios-integration.md)
- [架构设计](./architecture.md)
