# KeedaVault Core

[中文文档](./README_CN.md)

🦀 Cross-platform KDBX password vault core library - Written in Rust

## 📋 Overview

`keedavault-core` is the core library for the KeedaVault password manager, providing secure password database management capabilities fully compatible with the KeePass KDBX format.

### Features

- ✅ **KDBX Support**: Fully compatible with KeePass database format (KDBX3/KDBX4)
- 🔐 **Strong Encryption**: Argon2, ChaCha20, AES Encryption
- 🔑 **TOTP Support**: Time-based One-Time Password generation
- 🔍 **Search & Filter**: Fast entry search and tag filtering
- 📱 **Cross-platform**: Support for Desktop (macOS/Windows/Ubuntu) and iOS
- 🦺 **Type Safe**: Rust memory safety and performance guarantees

### Platform Support

| Platform | Integration Method | Status |
|----------|-------------------|--------|
| **macOS Desktop** | Tauri (Direct Rust API) | ✅ Supported |
| **Windows Desktop** | Tauri (Direct Rust API) | ✅ Supported |
| **Ubuntu Desktop** | Tauri (Direct Rust API) | ✅ Supported |
| **iOS** | UniFFI (Swift bindings) | ✅ Supported |

## 🚀 Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
keedavault-core = "0.1.0"
```

### Basic Usage

#### 🖥️ Desktop (macOS/Windows/Ubuntu) - Tauri Integration

Use Rust API directly in your Tauri project:

```rust
use keedavault_core::{Vault, VaultConfig, Entry};

// Open existing vault
let vault = Vault::open("my_passwords.kdbx", "master_password")?;

// Get all entries
let entries = vault.get_entries()?;

// Create new vault
let mut vault = Vault::create(
    "new_vault.kdbx",
    "master_password",
    VaultConfig::default()
)?;

// Add entry
let mut entry = Entry::new("GitHub".to_string(), "root_group_id".to_string());
entry.username = "user@example.com".to_string();
entry.password = "secure_password".to_string();
entry.url = "https://github.com".to_string();

let entry_id = vault.add_entry(entry)?;

// Save vault
vault.save()?;

// Lock vault
vault.lock();
```

#### 📱 iOS - Swift Integration (via UniFFI)

> **Note**: iOS integration requires compiling XCFramework first. See [iOS Integration Guide](./docs/ios-integration.md).

```swift
import KeedavaultCore

// Open vault
let vault = try Vault.open(path: "my_passwords.kdbx", password: "master_password")

// Get entries
let entries = try vault.getEntries()

// Create new entry
var entry = Entry.new(title: "GitHub", groupId: "root_group_id")
entry.username = "user@example.com"
entry.password = "secure_password"
entry.url = "https://github.com"

let entryId = try vault.addEntry(entry: entry)

// Save
try vault.save()

// Lock
vault.lock()
```

## 📚 API Documentation

### Core Types

#### `Vault`

Main structure for vault management.

**Methods**:

| Method | Description | Desktop | iOS |
|--------|-------------|---------|-----|
| `open(path, password)` | Open existing vault | ✅ | ✅ |
| `create(path, password, config)` | Create new vault | ✅ | ✅ |
| `save()` | Save vault | ✅ | ✅ |
| `lock()` | Lock vault | ✅ | ✅ |
| `is_locked()` | Check if locked | ✅ | ✅ |
| `get_entries()` | Get all entries | ✅ | ✅ |
| `get_entry(id)` | Get single entry | ✅ | ✅ |
| `add_entry(entry)` | Add entry | ✅ | ✅ |
| `update_entry(id, entry)` | Update entry | ✅ | ✅ |
| `delete_entry(id)` | Delete entry | ✅ | ✅ |
| `get_groups()` | Get all groups | ✅ | ✅ |

#### `Entry`

Represents a password entry.

**Fields**:

```rust
pub struct Entry {
    pub id: String,              // Unique Identifier
    pub group_id: String,        // Parent Group ID
    pub title: String,           // Title
    pub username: String,        // Username
    pub password: String,        // Password
    pub url: String,             // URL
    pub notes: String,           // Notes
    pub tags: Vec<String>,       // Tags
    pub totp_secret: Option<String>, // TOTP Secret
    pub custom_fields: Vec<CustomField>, // Custom Fields
    pub created_at: DateTime<Utc>,   // Creation Time
    pub modified_at: DateTime<Utc>,  // Modification Time
    pub accessed_at: DateTime<Utc>,  // Access Time
    pub expires_at: Option<DateTime<Utc>>, // Expiry Time
    pub is_favorite: bool,       // Is Favorite
}
```

**Methods**:

| Method | Description | Desktop | iOS |
|--------|-------------|---------|-----|
| `new(title, group_id)` | Create new entry | ✅ | ✅ |
| `touch()` | Update modification time | ✅ | ✅ |
| `mark_accessed()` | Mark as accessed | ✅ | ✅ |
| `is_expired()` | Check if expired | ✅ | ✅ |

#### `Group`

Represents a group/folder.

**Fields**:

```rust
pub struct Group {
    pub id: String,              // Unique Identifier
    pub parent_id: Option<String>, // Parent Group ID
    pub name: String,            // Name
    pub icon_id: u32,            // Icon ID
    pub notes: String,           // Notes
    pub is_recycle_bin: bool,    // Is Recycle Bin
    pub is_expanded: bool,       // Is Expanded
}
```

**Methods**:

| Method | Description | Desktop | iOS |
|--------|-------------|---------|-----|
| `new(name, parent_id)` | Create new group | ✅ | ✅ |
| `new_recycle_bin()` | Create recycle bin | ✅ | ✅ |
| `is_root()` | Is root group | ✅ | ✅ |

#### `VaultConfig`

Configuration for vault encryption parameters.

```rust
pub struct VaultConfig {
    pub kdf_iterations: u64,      // KDF Iterations
    pub argon2_memory: u64,       // Argon2 Memory (KB)
    pub argon2_parallelism: u32,  // Argon2 Parallelism
}
```

**Defaults**:
- `kdf_iterations`: 2
- `argon2_memory`: 65536 (64 MB)
- `argon2_parallelism`: 2

### Search and Filter

```rust
use keedavault_core::search;

// Full text search
let results = search::search_entries(&entries, "github");

// Filter by tag
let tagged = search::filter_by_tag(&entries, "work");

// Get favorites
let favorites = search::get_favorites(&entries);
```

| Function | Description | Desktop | iOS |
|----------|-------------|---------|-----|
| `search_entries(entries, query)` | Full text search | ✅ | ✅ |
| `filter_by_tag(entries, tag)` | Filter by tag | ✅ | ✅ |
| `get_favorites(entries)` | Get favorites | ✅ | ✅ |

### TOTP

```rust
use keedavault_core::totp;

// Generate TOTP code
let code = totp::generate_totp("JBSWY3DPEHPK3PXP")?;

// Validate TOTP code
let is_valid = totp::validate_totp("JBSWY3DPEHPK3PXP", "123456")?;
```

### Error Handling

```rust
use keedavault_core::{VaultError, Result};

match Vault::open("vault.kdbx", "wrong_password") {
    Ok(vault) => { /* Success */ },
    Err(VaultError::InvalidPassword) => {
        println!("Invalid password");
    },
    Err(VaultError::OpenError(msg)) => {
        println!("Failed to open: {}", msg);
    },
    Err(e) => {
        println!("Other error: {}", e);
    }
}
```

**Error Types**:

- `OpenError(String)` - Failed to open vault
- `SaveError(String)` - Failed to save vault
- `InvalidPassword` - Invalid password
- `VaultLocked` - Vault is locked
- `EntryNotFound(String)` - Entry not found
- `GroupNotFound(String)` - Group not found
- `EncryptionError(String)` - Encryption error
- `DecryptionError(String)` - Decryption error

## 🔧 Build

### Desktop Build

```bash
# Standard build
cargo build --release

# Run tests
cargo test

# Generate docs
cargo doc --open
```

### iOS Build

> **Note**: iOS build requires macOS environment and Xcode.

```bash
# Add iOS targets
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim

# Build iOS library (enable UniFFI)
cargo build --target aarch64-apple-ios --release --features uniffi

# Generate Swift bindings
cargo run --bin uniffi-bindgen generate src/keedavault.udl --language swift

# Package XCFramework
./scripts/build-xcframework.sh
```

See [iOS Integration Guide](./docs/ios-integration.md) for detailed steps.

## 🎯 Platform Differences

### Desktop (Tauri) vs iOS (UniFFI)

| Feature | Desktop (Tauri) | iOS (UniFFI) |
|---------|-----------------|--------------|
| **Language** | Rust | Swift |
| **Integration** | Direct Rust API Call | Via FFI Call |
| **Type Conversion** | None needed | Auto-generated Swift types |
| **Performance** | Native | FFI Overhead (Minimal) |
| **Async Support** | Rust async/await | Swift async/await |
| **Error Handling** | Rust Result | Swift throws |
| **Memory** | Rust Ownership | ARC + Rust Ownership |

### Platform Specific Features

#### macOS Desktop Specific

```rust
// Tauri Command Example
#[tauri::command]
fn unlock_vault(path: String, password: String) -> Result<VaultHandle, String> {
    let vault = Vault::open(&path, &password)
        .map_err(|e| e.to_string())?;
    Ok(VaultHandle::new(vault))
}
```

#### iOS Specific

```swift
// Store master password in Keychain
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

// Unlock with Face ID
import LocalAuthentication

func unlockWithBiometrics() async throws -> Vault {
    let context = LAContext()
    try await context.evaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, 
                                     localizedReason: "Unlock Vault")
    let password = getPasswordFromKeychain()
    return try Vault.open(path: vaultPath, password: password)
}
```

## 📖 Detailed Documentation

- [API Reference](./docs/api-reference.md)
- [Desktop Integration Guide](./docs/desktop-integration.md)
- [iOS Integration Guide](./docs/ios-integration.md)
- [Architecture](./docs/architecture.md)
- [Security Best Practices](./docs/security.md)

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_create_vault
```

**Current Test Coverage**:
- ✅ Vault Creation and Locking
- ✅ Entry Creation and Expiry Check
- ✅ Group Creation and Hierarchy
- ✅ Search Functionality
- ✅ TOTP Placeholders

## 📊 Project Status

### Completed ✅

- [x] Project Initialization
- [x] Basic Data Structures (Entry, Group, Vault)
- [x] Error Handling System
- [x] KDBX File Open/Create/Save
- [x] Search and Filter Functionality
- [x] Unit Tests
- [x] Entry/Group CRUD Complete Implementation
- [x] TOTP Generation and Validation
- [x] UniFFI iOS Bindings
- [x] Performance Optimization

### Ongoing 🚧

*(None)*

### Planned 📋

- [ ] Cloud Sync Support (WebDAV/S3)
- [ ] Password Strength Detection
- [ ] Auto-lock
- [ ] Security Audit Log

## 🤝 Contribution

Part of the KeedaVault project. Contributions welcome!

## 📄 License

MIT License

## 🔗 Related Projects

- [keedavault-app](https://github.com/daocatt/keedavault-app) - Desktop and iOS Application
- [keedavault](https://github.com/daocatt/keedavault) - Original Implementation (Reference)

## 📞 Support

- Docs: [docs/](./docs/)
- Issues: [GitHub Issues](https://github.com/daocatt/keedavault-core/issues)
- Discussions: [GitHub Discussions](https://github.com/daocatt/keedavault-core/discussions)
