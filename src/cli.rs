//! CLI 定義（§9）。clap は引数定義のみで判断（self-heal / stop 等）を持たない。
//!
//! Phase 1 は `serve` のみ。`status`（Phase 2）・`init`（Phase 3）・`uninstall`（Phase 4）を順次追加。
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kage",
    version,
    about = "クロスOS で自分の AI コンテキストを複数 Claude Code に配信する MCP サーバ + 配線 CLI"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// 各面を検出し、CLAUDE.md トリガーと MCP 設定を配線する（冪等・§9）。
    Init,
    /// MCP サーバを foreground 起動する（localhost 固定ポート・§5-6 / §5-7）。
    Serve,
    /// kage サーバ到達とコンテキスト取得を検証する（三状態・§7-3 / §9）。
    Status,
}
