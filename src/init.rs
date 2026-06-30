//! `kage init`（§9）: 検出 → 配線（prompt + connection）→ registry 記録。
//!
//! 列挙源は §9-1 検出（registry 非依存・全面を訪れる）。2回目以降は「配線」ではなく
//! reconcile（§9-2: 意図 × 前状態 × 現実の突き合わせ → 現実へ収束）。冪等。
use crate::error::Result;
use crate::wiring::{connection, prompt};
use crate::{detection, paths, registry, scaffold};

pub fn run() -> Result<()> {
    // 源を用意（§9 init の1行目）。冪等な self-heal（§10-1）。
    if scaffold::ensure_aboutme()? {
        let p = paths::aboutme_path()?;
        println!(
            "✓ {} 雛形を作成（あなたのコンテキストに編集してください）",
            p.display()
        );
    }

    let mut reg = registry::Registry::new();

    for target in paths::targets()? {
        println!("● {} [{}]", target.label, target.id);

        // 検出 + 不在時の先回り作成（§9-1 / ADR-0005 self-heal）。
        if detection::ensure_claude_dir(&target.claude_dir)? {
            println!("  ・{} を先回り作成", target.claude_dir.display());
        }

        // prompt 配線（§7-3 pull・ポインタのみ／実体は複製しない 不変条件2）。
        let pr = prompt::upsert(&target.claude_md)?;
        println!(
            "  ・CLAUDE.md  {}  … {}",
            target.claude_md.display(),
            pr.action
        );
        reg.prompt.targets.insert(
            target.id.to_string(),
            registry::PromptTarget {
                path: target.claude_md.display().to_string(),
                is_created: pr.is_created,
            },
        );

        // connection 配線（§8-5 / ADR-0006・mcpServers.kage のみ 不変条件4）。
        let ca = connection::upsert(&target.claude_json)?;
        println!(
            "  ・mcpServers.kage  {}  … {}",
            target.claude_json.display(),
            ca
        );
        reg.connection.targets.insert(
            target.id.to_string(),
            registry::ConnectionTarget {
                path: target.claude_json.display().to_string(),
            },
        );
    }

    registry::write(&reg)?;
    println!("✓ registry.json に配線を記録（undo 情報）");
    println!();
    println!(
        "次: `kage serve` でサーバ起動 → `kage status` で検証 → 新しい Claude Code セッションで反映"
    );
    Ok(())
}
