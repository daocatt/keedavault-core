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

4.  **Configure Search Paths** (Crucial Step):
    This tells Xcode where to find the Rust module map.
    
    1.  Select the **KeedaVaultDemo** project (blue icon) in the Project Navigator.
    2.  Select the **KeedaVaultDemo** *Target* (not the Project) in the main editor list.
    3.  Select the **Build Settings** tab.
    4.  **Vital**: Click **"All"** (instead of "Basic") in the filter bar to reveal hidden settings.
    5.  In the search bar, type `SWIFT_INCLUDE_PATHS` directly.
        - This ensures you find the setting regardless of its display name (usually **"Import Paths"** under *Swift Compiler - Search Paths*).
    6.  Double-click the empty value area and add:
        - `$(PROJECT_DIR)/../Bindings`
    
    *If compilation still fails with "module not found", also add the same path to **"Header Search Paths"** (`HEADER_SEARCH_PATHS`).*


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

- **"Target has Swift tasks not blocking downstream targets"**:
    - This is usually a harmless warning in Xcode.
    - **Check Build Phases**: Go to your Target's **Build Phases** > **Compile Sources**. Ensure `keedavault_coreFFI.h` is **NOT** listed there. Headers should not be compiled directly. If it is there, remove it (select and click `-`).
    - Try **Product > Clean Build Folder**.

## Rebuilding the Core

If you modify the Rust code, rebuild the library:

```bash
# For Simulator (M1/M2)
cargo build --release --target aarch64-apple-ios-sim --features uniffi

# For Physical Device
cargo build --release --target aarch64-apple-ios --features uniffi
```

Then copy the new `.a` file to `Libs/`.
