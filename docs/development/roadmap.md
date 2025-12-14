# KeedaVault 项目路线图

**当前版本**: v0.3.1
**更新日期**: 2025-12-14

---

## 📍 当前状态

### ✅ 已完成 (Phase 1)

1. **核心功能** (100%)
   - ✅ Vault 基础操作 (create, open, lock, save)
   - ✅ Entry CRUD (完整实现)
   - ✅ Group CRUD (完整实现)
   - ✅ 回收站机制 (KeePass 标准)
   - ✅ TOTP 支持 (RFC 6238)
   - ✅ 搜索和过滤

2. **安全性** (100%)
   - ✅ Zeroize 内存保护
   - ✅ 代码审计完成
   - ✅ 安全文档

3. **测试** (100%)
   - ✅ 18/18 测试通过
   - ✅ 100% 核心功能覆盖

---

## 🎯 接下来的步骤

### Phase 1.5: UniFFI iOS 绑定 (优先级: 🔴 最高)

**目标**: 让 Rust core 可以被 Swift/iOS 调用

**预计时间**: 2-3 小时

#### 任务清单

1. **创建 UniFFI 接口定义** (30 min)
   ```bash
   # 创建 uniffi/vault.udl
   ```
   - [ ] 定义 Vault 接口
   - [ ] 定义 Entry/Group 类型
   - [ ] 定义错误类型

2. **配置 Cargo.toml** (15 min)
   - [ ] 启用 uniffi 特性
   - [ ] 配置 build.rs

3. **生成 Swift 绑定** (30 min)
   - [ ] 运行 uniffi-bindgen
   - [ ] 验证生成的 Swift 代码

4. **创建 iOS 集成示例** (45 min)
   - [ ] 简单的 Swift 调用示例
   - [ ] 测试基本 CRUD 操作

5. **文档** (30 min)
   - [ ] 更新 ios-integration.md
   - [ ] 添加使用示例

**输出**:
- `uniffi/vault.udl` - UniFFI 接口定义
- `build.rs` - 构建脚本
- Swift 绑定代码
- iOS 集成文档

---

### Phase 2: Desktop 集成 (优先级: 🟡 高)

**目标**: 集成到 Tauri 应用

**预计时间**: 3-4 小时

#### 任务清单

1. **创建 Tauri Commands** (1 hour)
   - [ ] 实现 Vault 操作命令
   - [ ] 实现 Entry/Group CRUD 命令
   - [ ] 错误处理

2. **状态管理** (1 hour)
   - [ ] Vault 状态管理
   - [ ] 锁定/解锁机制

3. **前端适配** (1.5 hours)
   - [ ] 更新 TypeScript 类型
   - [ ] 适配新的 API

4. **测试** (30 min)
   - [ ] 端到端测试
   - [ ] 性能测试

---

### Phase 3: 功能增强 (优先级: 🟢 中)

**预计时间**: 持续进行

#### 3.1 API 文档完善 (1 hour)
- [ ] 为所有公共 API 添加 Rustdoc 示例
- [ ] 创建 API 使用指南
- [ ] 添加常见问题解答

#### 3.2 性能优化 (2 hours)
- [ ] 搜索索引机制
- [ ] 增量保存
- [ ] 大型数据库性能测试

#### 3.3 代码重构 (2 hours)
- [ ] 拆分 vault.rs (目前 ~1000 行)
- [ ] 提取 helper methods 到单独模块
- [ ] 改进代码组织

---

## 🚀 推荐执行顺序

### 本周 (2025-12-14 ~ 2025-12-20)

**Day 1-2: UniFFI iOS 绑定**
```bash
1. 创建 uniffi/vault.udl
2. 配置构建系统
3. 生成并测试 Swift 绑定
4. 编写集成文档
```

**Day 3-4: Desktop 集成**
```bash
1. 实现 Tauri Commands
2. 更新前端代码
3. 端到端测试
```

**Day 5: 文档和优化**
```bash
1. 完善 API 文档
2. 性能测试
3. 代码审查
```

---

## 📋 详细步骤：UniFFI iOS 绑定

### Step 1: 创建 UniFFI 接口定义

创建 `uniffi/vault.udl`:

```idl
namespace keedavault {
    // 工厂函数
    Vault create_vault(string path, string password);
    Vault open_vault(string path, string password);
};

interface Vault {
    // 基础操作
    void lock();
    void save();
    
    // Entry CRUD
    string add_entry(Entry entry);
    Entry get_entry(string id);
    sequence<Entry> get_entries();
    void update_entry(string id, Entry entry);
    void delete_entry(string id);
    
    // Group CRUD
    string add_group(Group group);
    sequence<Group> get_groups();
    void update_group(string id, Group group);
    void delete_group(string id);
    
    // 回收站
    void empty_recycle_bin();
};

dictionary Entry {
    string id;
    string group_id;
    string title;
    string username;
    string password;
    string url;
    string notes;
    sequence<string> tags;
    string? totp_secret;
};

dictionary Group {
    string id;
    string? parent_id;
    string name;
    string notes;
};

[Error]
enum VaultError {
    "VaultLocked",
    "InvalidPassword",
    "EntryNotFound",
    "GroupNotFound",
    "IoError",
    "KeePassError",
};
```

### Step 2: 配置 Cargo.toml

```toml
[lib]
crate-type = ["lib", "cdylib", "staticlib"]

[dependencies]
uniffi = { version = "0.25", optional = true }

[build-dependencies]
uniffi = { version = "0.25", features = ["build"] }

[features]
default = []
uniffi = ["dep:uniffi"]
```

### Step 3: 创建 build.rs

```rust
fn main() {
    #[cfg(feature = "uniffi")]
    uniffi::generate_scaffolding("./uniffi/vault.udl").unwrap();
}
```

### Step 4: 生成 Swift 绑定

```bash
# 安装 uniffi-bindgen
cargo install uniffi-bindgen

# 生成 Swift 代码
cargo build --features uniffi
uniffi-bindgen generate uniffi/vault.udl --language swift --out-dir ./generated
```

---

## 🎓 学习资源

如果需要了解 UniFFI：
- [UniFFI Book](https://mozilla.github.io/uniffi-rs/)
- [UniFFI Examples](https://github.com/mozilla/uniffi-rs/tree/main/examples)

---

## ❓ 决策点

在开始之前，需要确认：

1. **iOS 优先还是 Desktop 优先？**
   - 建议：iOS (因为 Phase 1 文档中提到这是下一步)

2. **是否需要完整的 UniFFI 绑定？**
   - 建议：先实现核心功能，后续迭代

3. **测试策略？**
   - 建议：先手动测试，后续添加自动化

---

## 📊 进度追踪

| Phase | 状态 | 完成度 |
|-------|------|--------|
| Phase 1: Core CRUD | ✅ 完成 | 100% |
| Phase 1.5: UniFFI | 📋 待开始 | 0% |
| Phase 2: Desktop | 📋 待开始 | 0% |
| Phase 3: 增强 | 📋 待开始 | 0% |

---

**建议**: 从 **Phase 1.5: UniFFI iOS 绑定** 开始，这是最符合项目规划的下一步。
