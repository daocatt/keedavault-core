# Desktop 集成指南 (Tauri)

本指南说明如何在 Tauri 桌面应用中集成 `keedavault-core`。

## 目录

- [概述](#概述)
- [项目设置](#项目设置)
- [Tauri Commands](#tauri-commands)
- [状态管理](#状态管理)
- [前端集成](#前端集成)
- [最佳实践](#最佳实践)

---

## 概述

### 架构

```
┌─────────────────────────────────────┐
│   Frontend (React/Vue/Svelte)      │
│                                     │
│   TypeScript / JavaScript           │
└──────────────┬──────────────────────┘
               │ invoke()
               ▼
┌─────────────────────────────────────┐
│      Tauri Commands (Rust)          │
│                                     │
│   #[tauri::command]                 │
└──────────────┬──────────────────────┘
               │ 直接调用
               ▼
┌─────────────────────────────────────┐
│     keedavault-core (Rust)          │
│                                     │
│   Vault, Entry, Group, etc.         │
└─────────────────────────────────────┘
```

### 优势

- ✅ **零开销**: 直接调用 Rust API，无 FFI 开销
- ✅ **类型安全**: Rust 类型系统保证
- ✅ **性能**: 原生性能
- ✅ **跨平台**: 一套代码支持 macOS/Windows/Ubuntu

---

## 项目设置

### 1. 创建 Tauri 项目

```bash
npm create tauri-app@latest
```

选择:
- Framework: React / Vue / Svelte
- Language: TypeScript
- Package manager: npm / yarn / pnpm

### 2. 添加 keedavault-core 依赖

编辑 `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri = { version = "2.0", features = ["shell-open"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# 添加 keedavault-core
keedavault-core = { path = "../../keedavault-core" }
# 或者使用 git
# keedavault-core = { git = "https://github.com/daocatt/keedavault-core" }
```

### 3. 项目结构

```
keedavault-app/desktop/
├── src/                    # 前端代码
│   ├── components/
│   ├── stores/
│   ├── pages/
│   └── App.tsx
├── src-tauri/              # Tauri 后端
│   ├── src/
│   │   ├── main.rs         # 主入口
│   │   ├── commands/       # Tauri Commands
│   │   │   ├── mod.rs
│   │   │   ├── vault.rs
│   │   │   └── entry.rs
│   │   └── state.rs        # 应用状态
│   ├── Cargo.toml
│   └── tauri.conf.json
├── package.json
└── tsconfig.json
```

---

## Tauri Commands

### 状态管理

首先创建应用状态来管理打开的 Vault：

**`src-tauri/src/state.rs`**:

```rust
use keedavault_core::Vault;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

pub type VaultHandle = String;

pub struct AppState {
    vaults: Mutex<HashMap<VaultHandle, Vault>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            vaults: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_vault(&self, vault: Vault) -> VaultHandle {
        let handle = Uuid::new_v4().to_string();
        self.vaults.lock().unwrap().insert(handle.clone(), vault);
        handle
    }

    pub fn get_vault(&self, handle: &str) -> Option<Vault> {
        self.vaults.lock().unwrap().get(handle).cloned()
    }

    pub fn get_vault_mut<F, R>(&self, handle: &str, f: F) -> Option<R>
    where
        F: FnOnce(&mut Vault) -> R,
    {
        self.vaults.lock().unwrap().get_mut(handle).map(f)
    }

    pub fn remove_vault(&self, handle: &str) -> Option<Vault> {
        self.vaults.lock().unwrap().remove(handle)
    }
}
```

### Vault Commands

**`src-tauri/src/commands/vault.rs`**:

```rust
use keedavault_core::{Vault, VaultConfig, Entry, Group, VaultError};
use crate::state::{AppState, VaultHandle};
use tauri::State;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct VaultInfo {
    pub handle: VaultHandle,
    pub path: String,
    pub entry_count: usize,
    pub group_count: usize,
}

/// 打开数据库
#[tauri::command]
pub async fn open_vault(
    path: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<VaultInfo, String> {
    // 打开 vault
    let vault = Vault::open(&path, &password)
        .map_err(|e| format!("Failed to open vault: {}", e))?;

    // 获取信息
    let entry_count = vault.get_entries()
        .map_err(|e| format!("Failed to get entries: {}", e))?
        .len();
    
    let group_count = vault.get_groups()
        .map_err(|e| format!("Failed to get groups: {}", e))?
        .len();

    // 添加到状态
    let handle = state.add_vault(vault);

    Ok(VaultInfo {
        handle,
        path,
        entry_count,
        group_count,
    })
}

/// 创建新数据库
#[tauri::command]
pub async fn create_vault(
    path: String,
    password: String,
    state: State<'_, AppState>,
) -> Result<VaultInfo, String> {
    let vault = Vault::create(&path, &password, VaultConfig::default())
        .map_err(|e| format!("Failed to create vault: {}", e))?;

    let handle = state.add_vault(vault);

    Ok(VaultInfo {
        handle,
        path,
        entry_count: 0,
        group_count: 1, // Root group
    })
}

/// 保存数据库
#[tauri::command]
pub async fn save_vault(
    handle: VaultHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.get_vault_mut(&handle, |vault| {
        vault.save()
            .map_err(|e| format!("Failed to save vault: {}", e))
    })
    .ok_or_else(|| "Vault not found".to_string())?
}

/// 锁定数据库
#[tauri::command]
pub async fn lock_vault(
    handle: VaultHandle,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.remove_vault(&handle)
        .ok_or_else(|| "Vault not found".to_string())?;
    Ok(())
}

/// 检查是否锁定
#[tauri::command]
pub async fn is_vault_locked(
    handle: VaultHandle,
    state: State<'_, AppState>,
) -> Result<bool, String> {
    Ok(state.get_vault(&handle).is_none())
}
```

### Entry Commands

**`src-tauri/src/commands/entry.rs`**:

```rust
use keedavault_core::{Entry, VaultError};
use crate::state::{AppState, VaultHandle};
use tauri::State;

/// 获取所有条目
#[tauri::command]
pub async fn get_entries(
    handle: VaultHandle,
    state: State<'_, AppState>,
) -> Result<Vec<Entry>, String> {
    state.get_vault(&handle)
        .ok_or_else(|| "Vault not found".to_string())?
        .get_entries()
        .map_err(|e| format!("Failed to get entries: {}", e))
}

/// 获取单个条目
#[tauri::command]
pub async fn get_entry(
    handle: VaultHandle,
    entry_id: String,
    state: State<'_, AppState>,
) -> Result<Entry, String> {
    state.get_vault(&handle)
        .ok_or_else(|| "Vault not found".to_string())?
        .get_entry(&entry_id)
        .map_err(|e| format!("Failed to get entry: {}", e))
}

/// 添加条目
#[tauri::command]
pub async fn add_entry(
    handle: VaultHandle,
    entry: Entry,
    state: State<'_, AppState>,
) -> Result<String, String> {
    state.get_vault_mut(&handle, |vault| {
        vault.add_entry(entry)
            .map_err(|e| format!("Failed to add entry: {}", e))
    })
    .ok_or_else(|| "Vault not found".to_string())?
}

/// 更新条目
#[tauri::command]
pub async fn update_entry(
    handle: VaultHandle,
    entry_id: String,
    entry: Entry,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.get_vault_mut(&handle, |vault| {
        vault.update_entry(&entry_id, entry)
            .map_err(|e| format!("Failed to update entry: {}", e))
    })
    .ok_or_else(|| "Vault not found".to_string())?
}

/// 删除条目
#[tauri::command]
pub async fn delete_entry(
    handle: VaultHandle,
    entry_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.get_vault_mut(&handle, |vault| {
        vault.delete_entry(&entry_id)
            .map_err(|e| format!("Failed to delete entry: {}", e))
    })
    .ok_or_else(|| "Vault not found".to_string())?
}

/// 搜索条目
#[tauri::command]
pub async fn search_entries(
    handle: VaultHandle,
    query: String,
    state: State<'_, AppState>,
) -> Result<Vec<Entry>, String> {
    let entries = state.get_vault(&handle)
        .ok_or_else(|| "Vault not found".to_string())?
        .get_entries()
        .map_err(|e| format!("Failed to get entries: {}", e))?;

    Ok(keedavault_core::search::search_entries(&entries, &query))
}
```

### 主入口

**`src-tauri/src/main.rs`**:

```rust
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use state::AppState;

fn main() {
    tauri::Builder::default()
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::vault::open_vault,
            commands::vault::create_vault,
            commands::vault::save_vault,
            commands::vault::lock_vault,
            commands::vault::is_vault_locked,
            commands::entry::get_entries,
            commands::entry::get_entry,
            commands::entry::add_entry,
            commands::entry::update_entry,
            commands::entry::delete_entry,
            commands::entry::search_entries,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## 前端集成

### TypeScript 类型定义

**`src/types/vault.ts`**:

```typescript
export interface VaultInfo {
  handle: string;
  path: string;
  entryCount: number;
  groupCount: number;
}

export interface Entry {
  id: string;
  groupId: string;
  title: string;
  username: string;
  password: string;
  url: string;
  notes: string;
  tags: string[];
  totpSecret?: string;
  customFields: CustomField[];
  createdAt: string;
  modifiedAt: string;
  accessedAt: string;
  expiresAt?: string;
  isFavorite: boolean;
}

export interface CustomField {
  key: string;
  value: string;
  protected: boolean;
}

export interface Group {
  id: string;
  parentId?: string;
  name: string;
  iconId: number;
  notes: string;
  isRecycleBin: boolean;
  isExpanded: boolean;
}
```

### API 封装

**`src/api/vault.ts`**:

```typescript
import { invoke } from '@tauri-apps/api/core';
import type { VaultInfo, Entry, Group } from '../types/vault';

export class VaultAPI {
  /**
   * 打开数据库
   */
  static async open(path: string, password: string): Promise<VaultInfo> {
    return await invoke<VaultInfo>('open_vault', { path, password });
  }

  /**
   * 创建新数据库
   */
  static async create(path: string, password: string): Promise<VaultInfo> {
    return await invoke<VaultInfo>('create_vault', { path, password });
  }

  /**
   * 保存数据库
   */
  static async save(handle: string): Promise<void> {
    await invoke('save_vault', { handle });
  }

  /**
   * 锁定数据库
   */
  static async lock(handle: string): Promise<void> {
    await invoke('lock_vault', { handle });
  }

  /**
   * 检查是否锁定
   */
  static async isLocked(handle: string): Promise<boolean> {
    return await invoke<boolean>('is_vault_locked', { handle });
  }

  /**
   * 获取所有条目
   */
  static async getEntries(handle: string): Promise<Entry[]> {
    return await invoke<Entry[]>('get_entries', { handle });
  }

  /**
   * 获取单个条目
   */
  static async getEntry(handle: string, entryId: string): Promise<Entry> {
    return await invoke<Entry>('get_entry', { handle, entryId });
  }

  /**
   * 添加条目
   */
  static async addEntry(handle: string, entry: Entry): Promise<string> {
    return await invoke<string>('add_entry', { handle, entry });
  }

  /**
   * 更新条目
   */
  static async updateEntry(
    handle: string,
    entryId: string,
    entry: Entry
  ): Promise<void> {
    await invoke('update_entry', { handle, entryId, entry });
  }

  /**
   * 删除条目
   */
  static async deleteEntry(handle: string, entryId: string): Promise<void> {
    await invoke('delete_entry', { handle, entryId });
  }

  /**
   * 搜索条目
   */
  static async searchEntries(handle: string, query: string): Promise<Entry[]> {
    return await invoke<Entry[]>('search_entries', { handle, query });
  }
}
```

### React 示例

**`src/components/VaultUnlock.tsx`**:

```typescript
import React, { useState } from 'react';
import { VaultAPI } from '../api/vault';
import type { VaultInfo } from '../types/vault';

export function VaultUnlock() {
  const [path, setPath] = useState('');
  const [password, setPassword] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const handleUnlock = async () => {
    setLoading(true);
    setError('');

    try {
      const vaultInfo = await VaultAPI.open(path, password);
      console.log('Vault opened:', vaultInfo);
      // 导航到主界面
    } catch (err) {
      setError(err as string);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="vault-unlock">
      <h2>解锁密码库</h2>
      
      <input
        type="text"
        placeholder="数据库路径"
        value={path}
        onChange={(e) => setPath(e.target.value)}
      />

      <input
        type="password"
        placeholder="主密码"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />

      {error && <div className="error">{error}</div>}

      <button onClick={handleUnlock} disabled={loading}>
        {loading ? '解锁中...' : '解锁'}
      </button>
    </div>
  );
}
```

**`src/components/EntryList.tsx`**:

```typescript
import React, { useEffect, useState } from 'react';
import { VaultAPI } from '../api/vault';
import type { Entry } from '../types/vault';

interface Props {
  vaultHandle: string;
}

export function EntryList({ vaultHandle }: Props) {
  const [entries, setEntries] = useState<Entry[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadEntries();
  }, [vaultHandle]);

  const loadEntries = async () => {
    try {
      const data = await VaultAPI.getEntries(vaultHandle);
      setEntries(data);
    } catch (err) {
      console.error('Failed to load entries:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleSearch = async () => {
    if (!searchQuery) {
      await loadEntries();
      return;
    }

    try {
      const results = await VaultAPI.searchEntries(vaultHandle, searchQuery);
      setEntries(results);
    } catch (err) {
      console.error('Search failed:', err);
    }
  };

  if (loading) return <div>加载中...</div>;

  return (
    <div className="entry-list">
      <div className="search-bar">
        <input
          type="text"
          placeholder="搜索..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
        />
        <button onClick={handleSearch}>搜索</button>
      </div>

      <div className="entries">
        {entries.map((entry) => (
          <div key={entry.id} className="entry-item">
            <h3>{entry.title}</h3>
            <p>{entry.username}</p>
            <small>{entry.url}</small>
          </div>
        ))}
      </div>
    </div>
  );
}
```

---

## 最佳实践

### 1. 错误处理

```rust
// 提供详细的错误信息
#[tauri::command]
pub async fn open_vault(
    path: String,
    password: String,
) -> Result<VaultInfo, String> {
    Vault::open(&path, &password)
        .map_err(|e| match e {
            VaultError::InvalidPassword => "密码错误".to_string(),
            VaultError::OpenError(msg) => format!("无法打开文件: {}", msg),
            _ => format!("未知错误: {}", e),
        })?;
    // ...
}
```

### 2. 自动保存

```typescript
// 使用 debounce 自动保存
import { debounce } from 'lodash';

const autoSave = debounce(async (handle: string) => {
  try {
    await VaultAPI.save(handle);
    console.log('Auto-saved');
  } catch (err) {
    console.error('Auto-save failed:', err);
  }
}, 2000);

// 在条目更新后调用
await VaultAPI.updateEntry(handle, entryId, entry);
autoSave(handle);
```

### 3. 安全退出

```typescript
// 在窗口关闭前锁定所有 vault
window.addEventListener('beforeunload', async () => {
  const handles = getOpenVaultHandles();
  await Promise.all(handles.map(h => VaultAPI.lock(h)));
});
```

### 4. 性能优化

```rust
// 批量操作
#[tauri::command]
pub async fn get_entries_with_groups(
    handle: VaultHandle,
    state: State<'_, AppState>,
) -> Result<(Vec<Entry>, Vec<Group>), String> {
    let vault = state.get_vault(&handle)
        .ok_or_else(|| "Vault not found".to_string())?;
    
    let entries = vault.get_entries()
        .map_err(|e| e.to_string())?;
    let groups = vault.get_groups()
        .map_err(|e| e.to_string())?;
    
    Ok((entries, groups))
}
```

---

## 构建和发布

### 开发模式

```bash
npm run tauri dev
```

### 生产构建

```bash
npm run tauri build
```

这将生成:
- **macOS**: `.dmg`, `.app`
- **Windows**: `.msi`, `.exe`
- **Linux**: `.deb`, `.AppImage`

---

## 相关文档

- [API 参考](./api-reference.md)
- [iOS 集成指南](./ios-integration.md)
- [Tauri 官方文档](https://tauri.app/)
