//! 配線台帳 `registry.json`（§8-5）。
//!
//! 「現状の記録」ではなく **取り消し手順の記録**。真実ではなく再生成可能なキャッシュ（不変条件1）—
//! init は現実（§9-1 検出）を列挙源に reconcile し、registry は出力としてのみ書く。判定には使わない（§9-2）。
//! prompt は自分の痕跡（isCreated 付き）、connection は他者共有（キーのみ削除ゆえ isCreated なし）。
use crate::error::Result;
use crate::paths;
use crate::wiring::prompt::MARKER;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Registry {
    pub prompt: PromptGroup,
    pub connection: ConnectionGroup,
}

#[derive(Serialize, Deserialize)]
pub struct PromptGroup {
    #[serde(rename = "type")]
    pub kind: String,
    pub marker: String,
    pub targets: BTreeMap<String, PromptTarget>,
}

#[derive(Serialize, Deserialize)]
pub struct PromptTarget {
    /// 解決済み実パス（undo 同一性保証のため焼き付ける。§8-5）。
    pub path: String,
    /// kage が新規作成したか（true→ファイルごと削除 / false→マーカー範囲のみ削除。§9-3）。
    #[serde(rename = "isCreated")]
    pub is_created: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionGroup {
    #[serde(rename = "type")]
    pub kind: String,
    /// 削除対象キー（常に `mcpServers.<key>` のみ操作。§8-5）。
    pub key: String,
    pub targets: BTreeMap<String, ConnectionTarget>,
}

#[derive(Serialize, Deserialize)]
pub struct ConnectionTarget {
    pub path: String,
}

impl Registry {
    pub fn new() -> Self {
        Registry {
            prompt: PromptGroup {
                kind: "markdown".to_string(),
                marker: MARKER.to_string(),
                targets: BTreeMap::new(),
            },
            connection: ConnectionGroup {
                kind: "json".to_string(),
                key: "kage".to_string(),
                targets: BTreeMap::new(),
            },
        }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

/// registry.json を `~/.kage/` に書く（kage 自身の領域。整形自由）。
pub fn write(reg: &Registry) -> Result<()> {
    let path = paths::registry_path()?;
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(&path, serde_json::to_string_pretty(reg)?)?;
    Ok(())
}

/// registry.json を読む（uninstall の undo 手順源。§8-5）。無ければ `None`。
pub fn read() -> Result<Option<Registry>> {
    let path = paths::registry_path()?;
    if !path.exists() {
        return Ok(None);
    }
    let reg = serde_json::from_str(&fs::read_to_string(&path)?)?;
    Ok(Some(reg))
}

/// registry.json を削除する（undo 消費後。§9-2: 削除は冪等性にとって非イベント）。
pub fn delete() -> Result<()> {
    let path = paths::registry_path()?;
    if path.exists() {
        fs::remove_file(&path)?;
    }
    Ok(())
}
