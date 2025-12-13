# KeedaVault Core

🦀 A cross-platform KDBX vault core library written in Rust.

## Overview

`keedavault-core` is the heart of the KeedaVault password manager. It provides secure password database management with support for the KDBX file format (KeePass compatible).

## Features

- ✅ **KDBX Support**: Full compatibility with KeePass database format
- 🔐 **Strong Encryption**: Argon2, ChaCha20, and AES encryption
- 🔑 **TOTP Generation**: Time-based one-time password support
- 🔍 **Search & Filter**: Fast entry search and filtering
- 📱 **Cross-Platform**: Works on Desktop (via Tauri) and iOS (via UniFFI)
- 🦺 **Type-Safe**: Written in Rust for memory safety and performance

## Architecture

```
┌─────────────────────────────────────────┐
│         keedavault-core (Rust)          │
│  ┌───────────────────────────────────┐  │
│  │   Core Business Logic             │  │
│  │   - KDBX parsing                  │  │
│  │   - Encryption/Decryption         │  │
│  │   - Entry/Group management        │  │
│  │   - TOTP generation               │  │
│  │   - Search & Filter               │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
            │
    ┌───────┴────────┐
    ▼                ▼
┌─────────┐    ┌──────────┐
│ Desktop │    │   iOS    │
│ (Tauri) │    │ (UniFFI) │
└─────────┘    └──────────┘
```

## Usage

### Desktop (Tauri)

```rust
use keedavault_core::{Vault, VaultConfig};

// Open an existing vault
let vault = Vault::open("my_passwords.kdbx", "master_password")?;

// Get all entries
let entries = vault.get_entries()?;

// Create a new vault
let vault = Vault::create(
    "new_vault.kdbx",
    "master_password",
    VaultConfig::default()
)?;
```

### iOS (Swift via UniFFI)

```swift
// UniFFI bindings will be generated
let vault = try Vault.open(path: "my_passwords.kdbx", password: "master_password")
let entries = try vault.getEntries()
```

## Building

### Standard Build

```bash
cargo build --release
```

### iOS Build

```bash
# Install iOS targets
rustup target add aarch64-apple-ios x86_64-apple-ios

# Build for iOS with UniFFI
cargo build --target aarch64-apple-ios --release --features uniffi
```

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_create_vault
```

## Project Structure

```
keedavault-core/
├── src/
│   ├── lib.rs          # Library entry point
│   ├── error.rs        # Error types
│   ├── vault.rs        # Vault operations
│   ├── entry.rs        # Entry data structure
│   ├── group.rs        # Group data structure
│   ├── crypto.rs       # Encryption utilities
│   ├── totp.rs         # TOTP generation
│   └── search.rs       # Search & filter
├── tests/              # Integration tests
├── benches/            # Performance benchmarks
├── uniffi/             # UniFFI interface definitions
└── Cargo.toml          # Dependencies
```

## Dependencies

- **keepass**: KDBX file format support
- **argon2**: Key derivation function
- **chacha20poly1305**: ChaCha20 encryption
- **aes**: AES encryption
- **thiserror**: Error handling
- **serde**: Serialization
- **totp-lite**: TOTP generation
- **uniffi** (optional): iOS bindings

## Development Status

🚧 **Phase 1: Core Foundation** (In Progress)

- [x] Project setup
- [x] Basic data structures (Entry, Group, Vault)
- [x] Error handling
- [ ] KDBX file operations
- [ ] Entry/Group CRUD
- [ ] TOTP implementation
- [ ] Search functionality
- [ ] Unit tests
- [ ] Documentation

See [implementation-plan.md](../keedavault/docs/refactoring/implementation-plan.md) for detailed roadmap.

## Contributing

This is part of the KeedaVault project. See the main repository for contribution guidelines.

## License

MIT License - see LICENSE file for details.

## Related Projects

- [keedavault-app](https://github.com/daocatt/keedavault-app) - Desktop and iOS applications
- [keedavault](https://github.com/daocatt/keedavault) - Original implementation (reference)

## Documentation

- [Architecture Overview](../keedavault/docs/refactoring/intro.md)
- [Project Structure](../keedavault/docs/refactoring/project-structure.md)
- [Implementation Plan](../keedavault/docs/refactoring/implementation-plan.md)
