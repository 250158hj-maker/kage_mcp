//! エントリ（§9）。parse → dispatch のみ。失敗は exit code + stderr へ（§10-1: 無言にしない）。
use clap::Parser;
use kage_mcp::cli::{Cli, Command};
use kage_mcp::error::{KageError, Result};

fn main() {
    if let Err(e) = run(Cli::parse()) {
        // §10-1: 原因 + 次アクション(hint) を stderr へ。
        eprintln!("kage: エラー: {e}");
        if let Some(hint) = e.hint() {
            eprintln!("  → {hint}");
        }
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        // init は同期（検出・ファイル/JSON 配線のみ）。
        Command::Init => kage_mcp::init::run(),
        Command::Serve => {
            // serve のみ async。サブコマンド単位でランタイムを起こす（sync コマンドに async を強いない）。
            let runtime = tokio::runtime::Runtime::new()
                .map_err(|e| KageError::new(format!("tokio ランタイム生成に失敗: {e}")))?;
            runtime.block_on(kage_mcp::mcp::serve::run())
        }
        // status は同期（std の TCP/ファイルのみ）。ランタイム不要。
        // ヘルスチェックとして不健全（接続失敗 / 取得失敗）は非ゼロ終了でスクリプトから検知可能に。
        Command::Status => {
            if !kage_mcp::status::run()? {
                std::process::exit(1);
            }
            Ok(())
        }
    }
}
