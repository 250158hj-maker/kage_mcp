//! CLAUDE.md へのポインタ配線（§7-3 pull / §12）。
//!
//! 置くのは「`kage://me` を取りに行け」というポインタのみ。コンテキスト実体は複製しない（不変条件2）。
//! 共有ファイルゆえマーカー範囲だけを upsert する（§9-2: 無→構築 / 現行版→no-op / 旧版→範囲置換）。
use crate::error::Result;
use std::fs;
use std::path::Path;

/// 追記範囲を囲むマーカー（§8-5）。開始・終了で同一文字列を用い、その間を1ブロックとする。
pub const MARKER: &str = "<!-- kage init で追加されました -->";

/// 配線するトリガーブロック（マーカー込み）。雛形はバイナリ埋め込み（§8-4）。
const BLOCK: &str = include_str!("../../templates/CLAUDE_trigger.md");

/// upsert の結果（人向け表示用）。
pub struct PromptResult {
    pub action: &'static str,
    /// kage がファイルを新規作成したか（registry の isCreated。undo 分岐に使う §9-3）。
    pub is_created: bool,
}

/// CLAUDE.md にトリガーブロックを upsert する。
///
/// symlink の場合は透過的に実体へ書き込む（実体を follow）。既存ユーザー記述は温存する。
pub fn upsert(path: &Path) -> Result<PromptResult> {
    let existed = path.exists();
    let current = if existed {
        fs::read_to_string(path)?
    } else {
        String::new()
    };
    let block = BLOCK.trim_end();

    if let Some((start, end)) = find_block(&current) {
        if current[start..end].trim_end() == block {
            // 現行版が既にある → no-op（冪等）。
            return Ok(PromptResult {
                action: "変更なし",
                is_created: false,
            });
        }
        // 旧版 → マーカー範囲のみ置換（前後のユーザー記述は不変）。
        let replaced = format!("{}{}{}", &current[..start], block, &current[end..]);
        fs::write(path, replaced)?;
        return Ok(PromptResult {
            action: "更新（旧版を置換）",
            is_created: false,
        });
    }

    // マーカー無 → 構築 / 追記。
    let next = if current.trim().is_empty() {
        format!("{block}\n")
    } else {
        format!("{}\n\n{}\n", current.trim_end(), block)
    };
    fs::write(path, next)?;
    Ok(PromptResult {
        action: if existed { "追記" } else { "新規作成" },
        is_created: !existed,
    })
}

/// 最初のマーカー開始位置から、次のマーカー終了位置までを1ブロックとして返す。
fn find_block(text: &str) -> Option<(usize, usize)> {
    let first = text.find(MARKER)?;
    let after = first + MARKER.len();
    let second_end = text[after..].find(MARKER)? + after + MARKER.len();
    Some((first, second_end))
}
