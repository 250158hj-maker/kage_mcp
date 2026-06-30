//! パス解決（§8-4: `[username]` は実行時に `$HOME` から導出し、保存しない）。
//!
//! Phase 1 は SSoT（`~/.kage/`）のみ。各面（WSL / Windows / Desktop）の実パス解決は
//! Phase 3（init 配線）で追加する。
use crate::error::{KageError, Result};
use std::path::{Path, PathBuf};

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

/// WSL から Windows 側ホームへ到達する base（§7-2: `/mnt/c` 経由）。
const WIN_USERS_DIR: &str = "/mnt/c/Users";

/// (b)(c) Windows 面（Win Terminal と Desktop【Code】タブ Local が同一設定に収束。ADR-0002 / ADR-0006）。
///
/// `/mnt/c` が無い、または `.claude/` を持つ Windows 実ユーザーがいなければ `None`（面なし）。
/// 複数いて一意に決められない場合は stop（§5-4 / §10-1 ①一意性が崩れる）。
pub fn windows_target() -> Result<Option<Target>> {
    let users = Path::new(WIN_USERS_DIR);
    if !users.is_dir() {
        return Ok(None); // WSL2 で Windows に到達できない環境。
    }
    let Some(user) = resolve_windows_user(users)? else {
        return Ok(None); // `.claude/` を持つ Windows ユーザーが不在。
    };
    let base = users.join(&user);
    Ok(Some(Target {
        id: "windows",
        label: "Windows Claude Code (Terminal / Desktop Code Local)",
        claude_dir: base.join(".claude"),
        claude_md: base.join(".claude").join("CLAUDE.md"),
        claude_json: base.join(".claude.json"),
    }))
}

/// `.claude/` を持つ Windows 実ユーザーを解決する（§5-4）。
///
/// WSL ユーザー名とは無関係ゆえ `$HOME` から導けない。検出目的は「書き込み先の有無」（§9-1）と
/// 同型なので、コマンド有無ではなく `.claude/` の存在で判定する。
fn resolve_windows_user(users: &Path) -> Result<Option<String>> {
    // システム / 標準プロファイルは除外（`.claude/` を持たない想定だが二重に弾く）。
    const SYSTEM: &[&str] = &[
        "Default",
        "Default User",
        "Public",
        "All Users",
        "defaultuser0",
    ];
    let mut found: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(users)? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if SYSTEM.contains(&name.as_str()) {
            continue;
        }
        if users.join(&name).join(".claude").is_dir() {
            found.push(name);
        }
    }
    found.sort();
    match found.len() {
        0 => Ok(None),
        1 => Ok(found.into_iter().next()),
        _ => Err(KageError::stop(
            format!(".claude/ を持つ Windows ユーザーが複数あり一意に決められません: {found:?}"),
            "配線先ユーザーを確定できないため停止します。意図したユーザーに絞ってください",
        )),
    }
}

/// 配線対象の列挙（§9-1: 検出が列挙源、registry 非依存）。
/// (a) WSL は常に対象。(b)(c) は Windows 面（解決できれば）に収束（ADR-0002 / ADR-0006）。
pub fn targets() -> Result<Vec<Target>> {
    let mut targets = vec![wsl_target()?];
    if let Some(win) = windows_target()? {
        targets.push(win);
    }
    Ok(targets)
}
