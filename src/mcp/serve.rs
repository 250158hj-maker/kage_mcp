//! `kage serve`: MCP サーバを foreground 起動（§5-6）。
//!
//! 固定ポート衝突時は別ポートへ逃げず stop（§5-7 / ADR-0004）。プロセスの生死がターミナルに
//! 可視で、§10-1「失敗を無言にしない」と整合する。
use crate::config;
use crate::error::{KageError, Result};
use crate::mcp::server::KageServer;
use crate::scaffold;
use rmcp::transport::StreamableHttpService;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use std::net::SocketAddr;

/// foreground でサーバを起動し、Ctrl+C まで配信を続ける。
pub async fn run() -> Result<()> {
    // serve は源を必要とする。不在なら self-heal で雛形を用意（§10-1: ①一意 ∧ ②非侵襲）。
    if scaffold::ensure_aboutme()? {
        let path = crate::paths::aboutme_path()?;
        println!(
            "kage: ABOUTME.md が無かったため雛形を作成しました: {}",
            path.display()
        );
        println!("      あなたのコンテキストに書き換えてください（これが SSoT）。");
    }

    // StreamableHttpService を axum router に載せる（Phase 0 で確定した公式構成）。
    let service = StreamableHttpService::new(
        || Ok(KageServer::new()),
        LocalSessionManager::default().into(),
        Default::default(),
    );
    let app = axum::Router::new().nest_service(config::MCP_PATH, service);

    let addr = SocketAddr::from((config::BIND_ADDR, config::KAGE_PORT));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(bind_error)?;

    println!(
        "kage serve: {} で配信中（foreground / Ctrl+C で停止）",
        config::endpoint_url()
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    println!("kage serve: 停止しました");
    Ok(())
}

/// bind 失敗を分類。ポート占有は stop（§5-7: 逃げず占有調査を案内）。
fn bind_error(e: std::io::Error) -> KageError {
    if e.kind() == std::io::ErrorKind::AddrInUse {
        KageError::stop(
            format!(
                "ポート {} は使用中で kage serve を起動できません",
                config::KAGE_PORT
            ),
            format!(
                "別ポートへは切り替えません（各面の設定が固定ポートを指すため到達不能になる）。\
                 占有プロセスを調査・停止してください: ss -ltnp 'sport = :{port}' \
                 ／既に別ターミナルで kage serve 中ならそれを利用してください。",
                port = config::KAGE_PORT
            ),
        )
    } else {
        KageError::from(e)
    }
}

/// Ctrl+C を待つ（foreground の素直な停止経路）。
async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
}
