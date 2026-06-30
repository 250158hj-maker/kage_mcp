//! `kage uninstall`（§9-3）: init の逆。配線の痕跡のみを撤去する。
//!
//! - prompt: `isCreated` 分岐（true→ファイル削除 / false→マーカー範囲削除しユーザー記述温存）。
//! - connection: `mcpServers.kage` キーのみ削除（他サーバ温存・空 mcpServers 残骸許容）。
//! - `.claude/` ディレクトリと SSoT `~/.kage/ABOUTME.md` は削除しない（§9-3）。
use crate::error::Result;
use crate::wiring::{connection, prompt};
use crate::{paths, registry};
use std::path::Path;

pub fn run() -> Result<()> {
    match registry::read()? {
        Some(reg) => undo_from_registry(&reg)?,
        None => undo_from_reality()?,
    }
    // undo を消費したので台帳を削除（§9-2: registry 削除は冪等性にとって非イベント）。
    // SSoT（ABOUTME.md）と各面の `.claude/` は残す（§9-3）。
    registry::delete()?;
    println!();
    println!("✓ kage の配線を撤去しました（~/.kage/ABOUTME.md と各面の .claude/ は残しています）");
    Ok(())
}

/// 通常経路: 台帳に焼き付けた実パス + isCreated で undo（§8-5 の同一性保証）。
fn undo_from_registry(reg: &registry::Registry) -> Result<()> {
    for (id, t) in &reg.prompt.targets {
        let result = prompt::remove(Path::new(&t.path), t.is_created)?;
        println!("● [{id}] CLAUDE.md  {}  … {}", t.path, result);
    }
    for (id, t) in &reg.connection.targets {
        let result = connection::remove(Path::new(&t.path))?;
        println!("● [{id}] mcpServers.kage  {}  … {}", t.path, result);
    }
    Ok(())
}

/// フォールバック: 台帳が無い → 現実の面から best-effort（§9-2: reality is truth）。
/// isCreated 不明ゆえファイル削除はせず、マーカー範囲削除のみに留める（安全側）。
fn undo_from_reality() -> Result<()> {
    println!(
        "registry.json が無いため、検出した面から痕跡のみ撤去します（ファイル削除はしません）。"
    );
    for target in paths::targets()? {
        let result = prompt::remove(&target.claude_md, false)?;
        println!(
            "● [{}] CLAUDE.md  {}  … {}",
            target.id,
            target.claude_md.display(),
            result
        );
        let result = connection::remove(&target.claude_json)?;
        println!(
            "● [{}] mcpServers.kage  {}  … {}",
            target.id,
            target.claude_json.display(),
            result
        );
    }
    Ok(())
}
