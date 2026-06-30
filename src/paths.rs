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
