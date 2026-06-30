//! 面の検出（§9-1 / ADR-0005）。
//!
//! 検出対象は `.claude/` ディレクトリの存在（書き込み先そのもの）。`claude` コマンドの有無では
//! 判定しない（false negative を招く。ADR-0005）。不在なら先回り作成する self-heal:
//!   ① 終状態が一意＝「`.claude/` が在る」一通り
//!   ② 他者非侵襲＝Claude Code 標準の無害なディレクトリ
//! 両軸成立ゆえ self-heal 可（ADR-0001）。`.claude/` は削除しない（§9-3）ため undo 追跡はしない。
use crate::error::Result;
use std::fs;
use std::path::Path;

/// `.claude/` を検証し、無ければ作成する。戻り値: 新規作成したら `true`。
pub fn ensure_claude_dir(dir: &Path) -> Result<bool> {
    if dir.is_dir() {
        return Ok(false);
    }
    fs::create_dir_all(dir)?;
    Ok(true)
}
