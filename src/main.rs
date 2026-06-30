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
        Command::Serve => {
            // serve のみ async。サブコマンド単位でランタイムを起こす（sync コマンドに async を強いない）。
            let runtime = tokio::runtime::Runtime::new()
                .map_err(|e| KageError::new(format!("tokio ランタイム生成に失敗: {e}")))?;
            runtime.block_on(kage_mcp::mcp::serve::run())
        }
    }
}
