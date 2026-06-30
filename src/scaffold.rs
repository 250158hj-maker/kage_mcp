//! `~/.kage/` と ABOUTME.md 雛形の用意（§9: init の1行目）。
//!
//! 不在時の作成は self-heal（ADR-0001 の2軸を満たす）:
//!   ① 終状態が一意＝「ABOUTME.md が在る」一通り
//!   ② 他者非侵襲＝kage 自身の領域 `~/.kage/` に閉じる
//! 既存ファイルには触れない（冪等・SSoT 保全。不変条件1）。
use crate::error::Result;
use crate::paths;
use std::fs;

/// ABOUTME 雛形はバイナリに埋め込む（§8-4: 雛形は `~/.kage/` に置かず再現性を保つ）。
const ABOUTME_TEMPLATE: &str = include_str!("../templates/ABOUTME.md");

/// `~/.kage/ABOUTME.md` を用意する。既存なら一切触らない（SSoT を上書きしない）。
///
/// 戻り値: 新規作成したら `true`、既存ならば `false`。
pub fn ensure_aboutme() -> Result<bool> {
    let dir = paths::kage_dir()?;
    fs::create_dir_all(&dir)?;

    let path = paths::aboutme_path()?;
    if path.exists() {
        return Ok(false);
    }
    fs::write(&path, ABOUTME_TEMPLATE)?;
    Ok(true)
}
