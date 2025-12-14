# KeedaVault Core - iOS Integration Guide

This guide details how to integrate the `keedavault-core` Rust library into an iOS application using the generated UniFFI bindings.

## Prerequisites

- **Rust Toolchain**: Ensure you have Rust installed (latest stable).
- **Xcode**: Latest version recommended.
- **iOS Device/Simulator**: For running the app.
- **`keedavault-core`**: This repository.

## Step 1: Build the Rust Library for iOS

You need to compile the Rust code into a static library (`.a`) suitable for iOS architectures.

### 1. Add iOS Targets

```bash
rustup target add aarch64-apple-ios        # iOS devices
rustup target add aarch64-apple-ios-sim    # iOS simulator (Apple Silicon)
rustup target add x86_64-apple-ios         # iOS simulator (Intel)
```

### 2. Configure `Cargo.toml` for Static Lib

Ensure your `Cargo.toml` has `staticlib` crate type (if not already):

```toml
[lib]
crate-type = ["lib", "cdylib", "staticlib"]
```

### 3. Build the Library

Build for the target you need (e.g., iOS Simulator on Apple Silicon):

```bash
cargo build --release --target aarch64-apple-ios-sim --features uniffi
```

The output will be at `target/aarch64-apple-ios-sim/release/libkeedavault_core.a`.

> **Note**: For a real project, you'll likely want to create a universal library (`lipo`) or an XCFramework containing slices for both device and simulator.

## Step 2: Generate Swift Bindings

If you haven't already, generate the Swift bindings:

```bash
# Install tool (if needed)
# cargo install uniffi-bindgen --version 0.30

# Enable cli feature to build the bindgen tool
cargo run --features uniffi,cli --bin uniffi-bindgen generate \
    --library target/aarch64-apple-ios-sim/release/libkeedavault_core.a \
    --language swift \
    --out-dir generated/swift
```

You should get:
- `keedavault_core.swift`
- `keedavault_coreFFI.h`
- `keedavault_coreFFI.modulemap`

## Step 3: Integrate into Xcode

### 1. Add Files to Project

1. Open your iOS project in Xcode.
2. Drag and drop the following files into your project navigator:
   - `keedavault_core.swift`
   - `keedavault_coreFFI.h`
   - `keedavault_coreFFI.modulemap`
   - `libkeedavault_core.a` (the compiled Rust library)
3. Ensure "Copy items if needed" is unchecked (reference files) or checked (copy files), depending on your preference.
4. Ensure they are added to your app target.

### 2. Configure Bridging Header

UniFFI 0.30+ generates a `.modulemap`, so you might **not** need a bridging header if you configure the module map correctly. However, a common approach is:

1. Create a `Bridging-Header.h` if you don't have one.
2. Add `#include "keedavault_coreFFI.h"` to it.
3. In Build Settings, set **Objective-C Bridging Header** to the path of your bridging header.

**Alternative (Cleanest):**
1. In Build Settings, find **Import Paths** (Swift configurations) or **Header Search Paths**. Add the directory containing the `.modulemap` and `.h` files.
2. Ensure the `.modulemap` file is actually linking the `.h` file correctly relative to where Xcode sees it.

### 3. Link Libraries

1. Go to **Build Phases** -> **Link Binary With Libraries**.
2. Ensure `libkeedavault_core.a` is listed.
3. You might need to add system libraries that Rust depends on (though often handled automatically):
   - `libresolv.tbd`
   - `libiconv.tbd`

## Step 4: Usage in Swift

Now you can use the Rust code directly in Swift!

```swift
import Foundation

class VaultManager {
    static let shared = VaultManager()
    private var vault: Vault?
    
    func createNewVault(name: String, password: String) {
        let documentsPath = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)[0]
        let vaultPath = documentsPath.appendingPathComponent("\(name).kdbx").path
        
        // Define config (using KDF settings suitable for extensive rounds)
        let config = VaultConfig(
            kdfIterations: 2,       // For testing speed, use higher for production
            argon2Memory: 1024 * 64, // 64MB
            argon2Parallelism: 2
        )
        
        do {
            self.vault = try createVault(
                path: vaultPath, 
                password: password, 
                config: config
            )
            print("Vault created successfully!")
        } catch {
            print("Failed to create vault: \(error)")
        }
    }
    
    func addPassword(title: String, secret: String) {
        guard let vault = vault else { return }
        
        let entry = Entry(
            id: UUID().uuidString,
            groupId: "root", // Or actual group ID
            title: title,
            username: "me",
            password: secret,
            url: "",
            notes: "",
            tags: [],
            totpSecret: nil,
            customFields: [],
            createdAt: Int64(Date().timeIntervalSince1970),
            modifiedAt: Int64(Date().timeIntervalSince1970),
            accessedAt: Int64(Date().timeIntervalSince1970),
            expiresAt: nil,
            isFavorite: false
        )
        
        do {
            try vault.addEntry(entry: entry)
            // Remember to save!
            try vault.save()
            print("Entry added and saved.")
        } catch {
            print("Error adding entry: \(error)")
        }
    }
}
```

## Troubleshooting

- **Architecture Mismatch**: If you get linker errors about missing symbols for architecture `arm64` or `x86_64`, double check you built the Rust lib for the correct target (`cargo build --target ...`) and that Xcode is using that specific library file.
- **Undefined Symbols**: If you see errors about `_uniffi_...`, ensure the `.a` file is in "Link Binary With Libraries".

---

**Happy Coding!** 🚀
