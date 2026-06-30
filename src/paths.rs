//! パス解決（§8-4: `[username]` は実行時に `$HOME` から導出し、保存しない）。
//!
//! Phase 1 は SSoT（`~/.kage/`）のみ。各面（WSL / Windows / Desktop）の実パス解決は
//! Phase 3（init 配線）で追加する。
use crate::error::{KageError, Result};
use std::path::PathBuf;

/// `$HOME` を取得。未設定は stop（§10-1: 一意な自動解決が不能）。
fn home() -> Result<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from).ok_or_else(|| {
        KageError::stop(
            "環境変数 HOME が未設定で ~/.kage を解決できません",
            "HOME を設定して再実行してください",
        )
    })
}

/// `~/.kage/`（SSoT と registry の置き場。§8-4）。
pub fn kage_dir() -> Result<PathBuf> {
    Ok(home()?.join(".kage"))
}

/// SSoT 本体 `~/.kage/ABOUTME.md`（リソース `kage://me` の実体。§8-1）。
pub fn aboutme_path() -> Result<PathBuf> {
    Ok(kage_dir()?.join("ABOUTME.md"))
}

/// 配線台帳 `~/.kage/registry.json`（§8-5）。
pub fn registry_path() -> Result<PathBuf> {
    Ok(kage_dir()?.join("registry.json"))
}

/// 配線対象（OS ターゲット）。(a) は wsl、(b)(c) は windows に収束（ADR-0002 / ADR-0006）。
pub struct Target {
    pub id: &'static str,
    pub label: &'static str,
    /// 検出対象の `.claude/`（§9-1）。
    pub claude_dir: PathBuf,
    /// prompt 配線先 CLAUDE.md（§8-3）。
    pub claude_md: PathBuf,
    /// connection 配線先 `.claude.json`（§8-5 / ADR-0006）。
    pub claude_json: PathBuf,
}

/// (a) WSL 面。`$HOME` 直下に解決（§9-1）。
pub fn wsl_target() -> Result<Target> {
    let h = home()?;
    Ok(Target {
        id: "wsl",
        label: "WSL Claude Code",
        claude_dir: h.join(".claude"),
        claude_md: h.join(".claude").join("CLAUDE.md"),
        claude_json: h.join(".claude.json"),
    })
}

/// 配線対象の列挙（§9-1: 検出が列挙源、registry 非依存）。
/// Phase 3(a) は WSL のみ。windows（(b)(c) 収束）は次段で追加する。
pub fn targets() -> Result<Vec<Target>> {
    Ok(vec![wsl_target()?])
}
