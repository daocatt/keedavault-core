# iOS 集成指南 (UniFFI)

本指南说明如何在 iOS 应用中通过 UniFFI 集成 `keedavault-core`。

## 目录

- [概述](#概述)
- [环境准备](#环境准备)
- [构建 XCFramework](#构建-xcframework)
- [Xcode 集成](#xcode-集成)
- [Swift 使用示例](#swift-使用示例)
- [iOS 特定功能](#ios-特定功能)
- [最佳实践](#最佳实践)

---

## 概述

### 架构

```
┌─────────────────────────────────────┐
│        SwiftUI / UIKit              │
│                                     │
│   Swift Code                        │
└──────────────┬──────────────────────┘
               │ Swift API
               ▼
┌─────────────────────────────────────┐
│   UniFFI Generated Bindings         │
│                                     │
│   KeedavaultCore.swift              │
└──────────────┬──────────────────────┘
               │ FFI (C ABI)
               ▼
┌─────────────────────────────────────┐
│   keedavault_core.xcframework       │
│                                     │
│   Rust Library (静态库)              │
└─────────────────────────────────────┘
```

### 平台支持

| 目标 | 架构 | 用途 |
|------|------|------|
| `aarch64-apple-ios` | ARM64 | 真机 (iPhone/iPad) |
| `x86_64-apple-ios` | x86_64 | 模拟器 (Intel Mac) |
| `aarch64-apple-ios-sim` | ARM64 | 模拟器 (Apple Silicon Mac) |

---

## 环境准备

### 1. 安装 Rust 工具链

```bash
# 安装 Rust (如果还没有)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 添加 iOS 目标
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim
```

### 2. 安装 UniFFI 工具

```bash
cargo install uniffi-bindgen-go
```

### 3. 验证 Xcode

```bash
xcodebuild -version
# 应该显示 Xcode 14.0 或更高版本
```

---

## 构建 XCFramework

### 方法 1: 使用构建脚本（推荐）

创建 `scripts/build-xcframework.sh`:

```bash
#!/bin/bash

set -e

# 配置
PROJECT_NAME="keedavault_core"
FRAMEWORK_NAME="KeedavaultCore"
BUILD_DIR="target"
XCFRAMEWORK_DIR="$BUILD_DIR/xcframework"

# 清理旧构建
rm -rf "$XCFRAMEWORK_DIR"
mkdir -p "$XCFRAMEWORK_DIR"

echo "🔨 Building for iOS devices (ARM64)..."
cargo build --target aarch64-apple-ios --release --features uniffi

echo "🔨 Building for iOS Simulator (x86_64)..."
cargo build --target x86_64-apple-ios --release --features uniffi

echo "🔨 Building for iOS Simulator (ARM64)..."
cargo build --target aarch64-apple-ios-sim --release --features uniffi

echo "📦 Creating fat library for simulator..."
lipo -create \
    "$BUILD_DIR/x86_64-apple-ios/release/lib$PROJECT_NAME.a" \
    "$BUILD_DIR/aarch64-apple-ios-sim/release/lib$PROJECT_NAME.a" \
    -output "$XCFRAMEWORK_DIR/lib${PROJECT_NAME}_sim.a"

echo "🔧 Generating Swift bindings..."
cargo run --bin uniffi-bindgen generate \
    src/${PROJECT_NAME}.udl \
    --language swift \
    --out-dir "$XCFRAMEWORK_DIR/swift"

echo "📦 Creating XCFramework..."
xcodebuild -create-xcframework \
    -library "$BUILD_DIR/aarch64-apple-ios/release/lib$PROJECT_NAME.a" \
    -headers "$XCFRAMEWORK_DIR/swift" \
    -library "$XCFRAMEWORK_DIR/lib${PROJECT_NAME}_sim.a" \
    -headers "$XCFRAMEWORK_DIR/swift" \
    -output "$XCFRAMEWORK_DIR/$FRAMEWORK_NAME.xcframework"

echo "✅ XCFramework created at: $XCFRAMEWORK_DIR/$FRAMEWORK_NAME.xcframework"
echo "✅ Swift bindings at: $XCFRAMEWORK_DIR/swift/${FRAMEWORK_NAME}.swift"
```

赋予执行权限并运行:

```bash
chmod +x scripts/build-xcframework.sh
./scripts/build-xcframework.sh
```

### 方法 2: 手动构建

```bash
# 1. 构建各个目标
cargo build --target aarch64-apple-ios --release --features uniffi
cargo build --target x86_64-apple-ios --release --features uniffi
cargo build --target aarch64-apple-ios-sim --release --features uniffi

# 2. 创建模拟器 fat library
lipo -create \
    target/x86_64-apple-ios/release/libkeedavault_core.a \
    target/aarch64-apple-ios-sim/release/libkeedavault_core.a \
    -output target/libkeedavault_core_sim.a

# 3. 生成 Swift 绑定
cargo run --bin uniffi-bindgen generate \
    src/keedavault_core.udl \
    --language swift \
    --out-dir target/swift

# 4. 创建 XCFramework
xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/libkeedavault_core.a \
    -headers target/swift \
    -library target/libkeedavault_core_sim.a \
    -headers target/swift \
    -output target/KeedavaultCore.xcframework
```

---

## Xcode 集成

### 1. 创建 iOS 项目

在 Xcode 中创建新的 iOS App 项目。

### 2. 添加 XCFramework

1. 将 `KeedavaultCore.xcframework` 拖入项目
2. 在 **General** → **Frameworks, Libraries, and Embedded Content** 中确认已添加
3. 设置为 **Embed & Sign**

### 3. 添加 Swift 绑定文件

1. 将生成的 `KeedavaultCore.swift` 文件添加到项目
2. 确保在 **Target Membership** 中勾选

### 4. 配置 Build Settings

在 **Build Settings** 中:

- **Header Search Paths**: 添加 XCFramework 的 headers 路径
- **Library Search Paths**: 添加 XCFramework 路径
- **Other Linker Flags**: 添加 `-lkeedavault_core`

### 5. 项目结构

```
KeedavaultApp/
├── KeedavaultApp/
│   ├── App/
│   │   └── KeedavaultApp.swift
│   ├── Views/
│   │   ├── UnlockView.swift
│   │   ├── EntryListView.swift
│   │   └── EntryDetailView.swift
│   ├── ViewModels/
│   │   ├── VaultViewModel.swift
│   │   └── EntryViewModel.swift
│   ├── Models/
│   │   └── VaultState.swift
│   └── Utilities/
│       ├── KeychainHelper.swift
│       └── BiometricsHelper.swift
├── Frameworks/
│   └── KeedavaultCore.xcframework
└── Generated/
    └── KeedavaultCore.swift
```

---

## Swift 使用示例

### 基础使用

```swift
import KeedavaultCore

class VaultManager: ObservableObject {
    @Published var vault: Vault?
    @Published var entries: [Entry] = []
    @Published var isLocked = true
    
    // 打开数据库
    func openVault(path: String, password: String) throws {
        vault = try Vault.open(path: path, password: password)
        isLocked = false
        try loadEntries()
    }
    
    // 创建新数据库
    func createVault(path: String, password: String) throws {
        let config = VaultConfig.default()
        vault = try Vault.create(path: path, password: password, config: config)
        isLocked = false
    }
    
    // 加载条目
    func loadEntries() throws {
        guard let vault = vault else { return }
        entries = try vault.getEntries()
    }
    
    // 添加条目
    func addEntry(title: String, username: String, password: String) throws {
        guard let vault = vault else { return }
        
        var entry = Entry.new(title: title, groupId: "root")
        entry.username = username
        entry.password = password
        
        _ = try vault.addEntry(entry: entry)
        try vault.save()
        try loadEntries()
    }
    
    // 更新条目
    func updateEntry(_ entry: Entry) throws {
        guard let vault = vault else { return }
        
        try vault.updateEntry(id: entry.id, entry: entry)
        try vault.save()
        try loadEntries()
    }
    
    // 删除条目
    func deleteEntry(id: String) throws {
        guard let vault = vault else { return }
        
        try vault.deleteEntry(id: id)
        try vault.save()
        try loadEntries()
    }
    
    // 搜索条目
    func searchEntries(query: String) -> [Entry] {
        return entries.filter { entry in
            entry.title.lowercased().contains(query.lowercased()) ||
            entry.username.lowercased().contains(query.lowercased()) ||
            entry.url.lowercased().contains(query.lowercased())
        }
    }
    
    // 锁定数据库
    func lockVault() {
        vault?.lock()
        vault = nil
        entries = []
        isLocked = true
    }
}
```

### SwiftUI 视图

**UnlockView.swift**:

```swift
import SwiftUI

struct UnlockView: View {
    @StateObject private var vaultManager = VaultManager()
    @State private var password = ""
    @State private var showError = false
    @State private var errorMessage = ""
    
    let vaultPath: String
    
    var body: some View {
        VStack(spacing: 20) {
            Image(systemName: "lock.shield")
                .font(.system(size: 80))
                .foregroundColor(.blue)
            
            Text("解锁密码库")
                .font(.title)
            
            SecureField("主密码", text: $password)
                .textFieldStyle(.roundedBorder)
                .padding(.horizontal)
            
            Button("解锁") {
                unlockVault()
            }
            .buttonStyle(.borderedProminent)
            .disabled(password.isEmpty)
            
            Button("使用 Face ID") {
                unlockWithBiometrics()
            }
            .buttonStyle(.bordered)
        }
        .padding()
        .alert("错误", isPresented: $showError) {
            Button("确定", role: .cancel) { }
        } message: {
            Text(errorMessage)
        }
    }
    
    private func unlockVault() {
        do {
            try vaultManager.openVault(path: vaultPath, password: password)
            // 导航到主界面
        } catch {
            errorMessage = error.localizedDescription
            showError = true
        }
    }
    
    private func unlockWithBiometrics() {
        Task {
            do {
                let savedPassword = try await BiometricsHelper.authenticate(
                    reason: "解锁密码库"
                )
                try vaultManager.openVault(path: vaultPath, password: savedPassword)
            } catch {
                errorMessage = error.localizedDescription
                showError = true
            }
        }
    }
}
```

**EntryListView.swift**:

```swift
import SwiftUI

struct EntryListView: View {
    @ObservedObject var vaultManager: VaultManager
    @State private var searchText = ""
    @State private var showingAddEntry = false
    
    var filteredEntries: [Entry] {
        if searchText.isEmpty {
            return vaultManager.entries
        } else {
            return vaultManager.searchEntries(query: searchText)
        }
    }
    
    var body: some View {
        NavigationView {
            List {
                ForEach(filteredEntries, id: \.id) { entry in
                    NavigationLink(destination: EntryDetailView(
                        entry: entry,
                        vaultManager: vaultManager
                    )) {
                        EntryRow(entry: entry)
                    }
                }
                .onDelete(perform: deleteEntries)
            }
            .searchable(text: $searchText, prompt: "搜索条目")
            .navigationTitle("密码")
            .toolbar {
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button(action: { showingAddEntry = true }) {
                        Image(systemName: "plus")
                    }
                }
                
                ToolbarItem(placement: .navigationBarLeading) {
                    Button("锁定") {
                        vaultManager.lockVault()
                    }
                }
            }
            .sheet(isPresented: $showingAddEntry) {
                AddEntryView(vaultManager: vaultManager)
            }
        }
    }
    
    private func deleteEntries(at offsets: IndexSet) {
        for index in offsets {
            let entry = filteredEntries[index]
            do {
                try vaultManager.deleteEntry(id: entry.id)
            } catch {
                print("Failed to delete entry: \(error)")
            }
        }
    }
}

struct EntryRow: View {
    let entry: Entry
    
    var body: some View {
        VStack(alignment: .leading, spacing: 4) {
            Text(entry.title)
                .font(.headline)
            
            if !entry.username.isEmpty {
                Text(entry.username)
                    .font(.subheadline)
                    .foregroundColor(.secondary)
            }
            
            if !entry.url.isEmpty {
                Text(entry.url)
                    .font(.caption)
                    .foregroundColor(.blue)
            }
        }
        .padding(.vertical, 4)
    }
}
```

---

## iOS 特定功能

### 1. Keychain 集成

**KeychainHelper.swift**:

```swift
import Security
import Foundation

class KeychainHelper {
    static let shared = KeychainHelper()
    private let service = "com.bsdev.keedavault"
    
    // 保存密码到 Keychain
    func savePassword(_ password: String, for vaultPath: String) throws {
        let data = password.data(using: .utf8)!
        
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: vaultPath,
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly
        ]
        
        // 删除旧的
        SecItemDelete(query as CFDictionary)
        
        // 添加新的
        let status = SecItemAdd(query as CFDictionary, nil)
        guard status == errSecSuccess else {
            throw KeychainError.saveFailed(status)
        }
    }
    
    // 从 Keychain 获取密码
    func getPassword(for vaultPath: String) throws -> String {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: vaultPath,
            kSecReturnData as String: true
        ]
        
        var result: AnyObject?
        let status = SecItemCopyMatching(query as CFDictionary, &result)
        
        guard status == errSecSuccess,
              let data = result as? Data,
              let password = String(data: data, encoding: .utf8) else {
            throw KeychainError.notFound
        }
        
        return password
    }
    
    // 删除密码
    func deletePassword(for vaultPath: String) throws {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrService as String: service,
            kSecAttrAccount as String: vaultPath
        ]
        
        let status = SecItemDelete(query as CFDictionary)
        guard status == errSecSuccess || status == errSecItemNotFound else {
            throw KeychainError.deleteFailed(status)
        }
    }
}

enum KeychainError: Error {
    case saveFailed(OSStatus)
    case notFound
    case deleteFailed(OSStatus)
}
```

### 2. Face ID / Touch ID

**BiometricsHelper.swift**:

```swift
import LocalAuthentication

class BiometricsHelper {
    // 检查生物识别是否可用
    static func isBiometricsAvailable() -> Bool {
        let context = LAContext()
        var error: NSError?
        return context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: &error)
    }
    
    // 获取生物识别类型
    static func biometricType() -> LABiometryType {
        let context = LAContext()
        _ = context.canEvaluatePolicy(.deviceOwnerAuthenticationWithBiometrics, error: nil)
        return context.biometryType
    }
    
    // 使用生物识别认证并获取密码
    static func authenticate(reason: String) async throws -> String {
        let context = LAContext()
        
        // 验证生物识别
        try await context.evaluatePolicy(
            .deviceOwnerAuthenticationWithBiometrics,
            localizedReason: reason
        )
        
        // 从 Keychain 获取密码
        // 注意：这里需要知道 vaultPath
        // 实际使用时应该从 UserDefaults 或其他地方获取
        let vaultPath = UserDefaults.standard.string(forKey: "lastVaultPath") ?? ""
        return try KeychainHelper.shared.getPassword(for: vaultPath)
    }
}
```

### 3. 文件选择器

```swift
import UniformTypeIdentifiers

struct DocumentPicker: UIViewControllerRepresentable {
    @Binding var selectedPath: String?
    
    func makeUIViewController(context: Context) -> UIDocumentPickerViewController {
        let picker = UIDocumentPickerViewController(
            forOpeningContentTypes: [UTType(filenameExtension: "kdbx")!],
            asCopy: false
        )
        picker.delegate = context.coordinator
        return picker
    }
    
    func updateUIViewController(_ uiViewController: UIDocumentPickerViewController, context: Context) {}
    
    func makeCoordinator() -> Coordinator {
        Coordinator(self)
    }
    
    class Coordinator: NSObject, UIDocumentPickerDelegate {
        let parent: DocumentPicker
        
        init(_ parent: DocumentPicker) {
            self.parent = parent
        }
        
        func documentPicker(_ controller: UIDocumentPickerViewController, didPickDocumentsAt urls: [URL]) {
            guard let url = urls.first else { return }
            
            // 获取安全访问
            guard url.startAccessingSecurityScopedResource() else { return }
            defer { url.stopAccessingSecurityScopedResource() }
            
            parent.selectedPath = url.path
        }
    }
}
```

### 4. 后台锁定

```swift
class AppDelegate: NSObject, UIApplicationDelegate {
    var vaultManager: VaultManager?
    
    func applicationDidEnterBackground(_ application: UIApplication) {
        // 进入后台时锁定
        vaultManager?.lockVault()
    }
    
    func applicationWillEnterForeground(_ application: UIApplication) {
        // 返回前台时要求重新认证
        // 在视图中处理
    }
}
```

---

## 最佳实践

### 1. 错误处理

```swift
do {
    try vaultManager.openVault(path: path, password: password)
} catch VaultError.InvalidPassword {
    showAlert("密码错误")
} catch VaultError.OpenError(let message) {
    showAlert("无法打开文件: \(message)")
} catch {
    showAlert("未知错误: \(error.localizedDescription)")
}
```

### 2. 内存管理

```swift
class VaultViewController: UIViewController {
    private var vault: Vault?
    
    deinit {
        // 确保在释放时锁定
        vault?.lock()
        vault = nil
    }
    
    override func viewDidDisappear(_ animated: Bool) {
        super.viewDidDisappear(animated)
        
        // 视图消失时锁定
        if isMovingFromParent {
            vault?.lock()
            vault = nil
        }
    }
}
```

### 3. 性能优化

```swift
// 批量加载而不是逐个加载
func loadAllData() async {
    guard let vault = vault else { return }
    
    async let entries = try? vault.getEntries()
    async let groups = try? vault.getGroups()
    
    self.entries = await entries ?? []
    self.groups = await groups ?? []
}
```

### 4. 安全考虑

```swift
// 1. 使用 Keychain 存储密码
// 2. 启用 Face ID/Touch ID
// 3. 后台自动锁定
// 4. 剪贴板自动清除

class SecurityManager {
    static func copyPassword(_ password: String, clearAfter seconds: TimeInterval = 30) {
        UIPasteboard.general.string = password
        
        DispatchQueue.main.asyncAfter(deadline: .now() + seconds) {
            if UIPasteboard.general.string == password {
                UIPasteboard.general.string = ""
            }
        }
    }
}
```

---

## 故障排查

### 常见问题

#### 1. "Module 'KeedavaultCore' not found"

**解决方案**:
- 确认 XCFramework 已正确添加到项目
- 检查 Build Settings 中的 Framework Search Paths
- 清理构建文件夹 (Cmd+Shift+K)

#### 2. "Undefined symbols for architecture arm64"

**解决方案**:
- 确认已为所有目标架构构建
- 检查 lipo 命令是否正确执行
- 重新构建 XCFramework

#### 3. Swift 绑定文件编译错误

**解决方案**:
- 确认 UniFFI 版本与 Rust crate 版本匹配
- 重新生成 Swift 绑定
- 检查 .udl 文件语法

---

## 相关文档

- [API 参考](./api-reference.md)
- [Desktop 集成指南](./desktop-integration.md)
- [UniFFI 官方文档](https://mozilla.github.io/uniffi-rs/)
- [Apple 开发者文档](https://developer.apple.com/)
