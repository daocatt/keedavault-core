# KeedaVault Core - iOS Demo

This is a minimal SwiftUI demo application showcasing how to integrate `keedavault-core` using UniFFI bindings.

## Requirements

- Xcode 14+
- Rust Toolchain (for rebuilding core)

## Quick Start (No Rust experience needed)

We have pre-included a compiled static library for **iOS Simulator (Apple Silicon)**.

1.  **Create a Project**:
    - Open Xcode.
    - Create a new "App" project named `KeedaVaultDemo`.
    - Choose "SwiftUI" and "Swift".

2.  **Add Files**:
    - Right-click on the project folder in Xcode Navigator.
    - Select **"Add Files to..."**.
    - Select the `Sources` folder from this directory (add the files inside, not the folder itself, or create logical groups).
    - Select the `Bindings` folder (make sure to create groups).
    - Select the `Libs` folder.

3.  **Configure Build Settings**:
    - Go to **Build Phases** > **Link Binary with Libraries**.
    - Ensure `libkeedavault_core.a` is added.
    - Click `+` and add `libresolv.tbd` (required for some Rust std lib functions).

4.  **Configure Import Paths**:
    - Go to **Build Settings**.
    - Search for **"Import Paths"** (Swift Compiler - Search Paths).
    - Add the path to the `example/ios-demo/Bindings` folder (e.g., `$(PROJECT_DIR)/../Bindings` depending on where you put the Xcode project).
    - This allows Swift to find `keedavault_coreFFI.modulemap`.

5.  **Run**:
    - Select an **iPhone Simulator** (e.g., iPhone 15 Pro).
    - Hit **Run** (Cmd+R).
    - The app should launch. Click "Create Demo Vault" to test core functionality!

## Troubleshooting

- **"Module 'keedavault_coreFFI' not found"**:
    - This means Xcode can't find the `.modulemap` file. Check your **Import Paths** setting. It must be the *directory* containing the modulemap.

- **"Undefined symbol: ..."**:
    - Ensure `libkeedavault_core.a` is linked.
    - Ensure you are running on the correct architecture (Simulator / Apple Silicon). If running on a physical device, you need to rebuild the library for `aarch64-apple-ios`.

## Rebuilding the Core

If you modify the Rust code, rebuild the library:

```bash
# For Simulator (M1/M2)
cargo build --release --target aarch64-apple-ios-sim --features uniffi

# For Physical Device
cargo build --release --target aarch64-apple-ios --features uniffi
```

Then copy the new `.a` file to `Libs/`.
