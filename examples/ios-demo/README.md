# KeedaVault Core - iOS Demo

This is a minimal SwiftUI demo application showcasing how to integrate `keedavault-core` using UniFFI bindings.

## Requirements

- Xcode 14+
- Rust Toolchain (for rebuilding core)

## Quick Start (No Rust experience needed)

We have pre-included a **Universal Fat Library** for iOS Simulators.
It supports both **Apple Silicon (arm64)** and **Intel (x86_64)** Macs.

1.  **Create a Project**:
    - Open Xcode.
    - Create a new "App" project named `KeedaVaultDemo`.
    - Choose "SwiftUI" and "Swift".

2.  **Add Files**:
    - Right-click on the project folder in Xcode Navigator.
    - **Add `VaultViewModel.swift`**:
        - Select **"Add Files to..."**.
        - Navigate to `examples/ios-demo/Sources/`.
        - Select `VaultViewModel.swift`.
        - **Option**: Check "Copy items if needed".
        - Add it to the main "KeedaVaultDemo" group.
    
    - **Add Bridging Header**:
        - Select `KeedavaultDemo-Bridging-Header.h` from `examples/ios-demo/Sources/`.
        - Add it to the main group.

    - **Update `KeedaVaultDemoApp.swift` and `ContentView.swift`**:
        - Replace the content of the Xcode-created files with the code from `Sources/`.

    - **Add Bindings & Libs**:
        - Select the `Bindings` folder (Check **"Create groups"**).
        - Select the `Libs` folder (Check **"Create groups"**).

3.  **Configure Build Settings**:
    - **Link Binary with Libraries**: Ensure `libkeedavault_core.a` is present.
    - **Add System Library**: Click `+` and add `libresolv.tbd`.
    
4.  **Configure C Bindings (Bridging Header)**:
    This is the most reliable method for local static libraries.
    
    1.  Go to **Build Settings** > Search for `Bridging Header`.
    2.  Set **Objective-C Bridging Header** to: `KeedaVaultDemo/KeedaVaultDemo-Bridging-Header.h`
        *(Adjust path if you placed the file elsewhere. It must be relative to the `.xcodeproj`)*.
    3.  Search for `HEADER_SEARCH_PATHS`.
    4.  Add: `$(PROJECT_DIR)/../Bindings`
    5.  **Important**: Open `Bindings/keedavault_core.swift` and ensure the `import keedavault_coreFFI` block is commented out (we did this for you, but check if you regenerate).

5.  **Run**:
    - Select any Simulator (iPhone 16, etc.).
    - Cmd+R.

## Troubleshooting

- **"Module 'keedavault_coreFFI' not found"**:
    - This means Xcode can't find the `.modulemap` file. Check your **Import Paths** setting. It must be the *directory* containing the modulemap.

- **"Undefined symbol: ..."**:
    - Ensure `libkeedavault_core.a` is linked.
    - Ensure you are running on the correct architecture (Simulator / Apple Silicon). If running on a physical device, you need to rebuild the library for `aarch64-apple-ios`.

- **"Target has Swift tasks not blocking downstream targets"**:
    - This is usually a harmless warning in Xcode.
    - **Check Build Phases**: Go to your Target's **Build Phases** > **Compile Sources**. Ensure `keedavault_coreFFI.h` is **NOT** listed there. Headers should not be compiled directly. If it is there, remove it (select and click `-`).
    - Try **Product > Clean Build Folder**.

## Rebuilding the Core (Advanced)

If you modify the Rust code, you need to rebuild the static library.
Since we support both Intel and M1/M2 Macs, we create a **Fat Library**.

```bash
# 1. Build for Apple Silicon Simulator
cargo build --release --target aarch64-apple-ios-sim --features uniffi

# 2. Build for Intel Simulator
cargo build --release --target x86_64-apple-ios --features uniffi

# 3. Combine into a Fat Library
lipo -create \
  target/aarch64-apple-ios-sim/release/libkeedavault_core.a \
  target/x86_64-apple-ios/release/libkeedavault_core.a \
  -output examples/ios-demo/Libs/libkeedavault_core.a
```

*(Note: In `Cargo.toml`, we use `strip = false` and `lto = false` to ensure symbols are preserved for static linking.)*
