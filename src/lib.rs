//! KeedaVault Core Library
//!
//! A cross-platform KDBX vault core library written in Rust.
//! Provides secure password database management with support for:
//! - KDBX file format (KeePass compatible)
//! - Strong encryption (Argon2, ChaCha20, AES)
//! - TOTP generation
//! - Entry and group management
//!
//! # Platform Support
//! - Desktop: Direct Rust API (via Tauri)
//! - iOS: Swift bindings via UniFFI

pub mod crypto;
pub mod entry;
pub mod error;
pub mod group;
pub mod search;
pub mod totp;
pub mod vault;

// Re-export main types
pub use entry::Entry;
pub use error::{Result, VaultError};
pub use group::Group;
pub use vault::Vault;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Basic smoke test
        assert_eq!(2 + 2, 4);
    }
}
