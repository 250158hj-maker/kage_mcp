//! `kage status`: 三状態の機械検証（§7-3 / §9）。silent failure 検出器（§10-1）。
//!
//! 「現在の検証」という単一責任に閉じる（§8-4: 履歴は持たない）。
//! Phase 2 は WSL（ローカル面）の (1) サーバ到達 (2) 源 ABOUTME の取得可否 を見る。
//! CLAUDE.md ポインタ配線の検証・各面（Windows）到達は Phase 3（配線実装）で追加する。
//!
//! 依存を増やさず std だけで検証する（§10 可搬性 / 計画: rmcp client は重いので使わない）。
//! 最頻の無言失敗＝serve 起動忘れ（§5-6 foreground の構造的弱点）を確実に捕捉する。
use crate::config;
use crate::error::Result;
use crate::paths;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(2);

/// 三状態を検証して結果を表示する。戻り値: 取得成功なら `true`、不健全（接続失敗 / 取得失敗）なら `false`。
pub fn run() -> Result<bool> {
    let addr = SocketAddr::from((config::BIND_ADDR, config::KAGE_PORT));

    // (1) サーバ到達。HTTP 層が応答するかまで見る（ソケットが開くだけの誤検出を避ける）。
    if !http_alive(&addr) {
        // 接続失敗（§7-3）。
        println!(
            "✗ 接続失敗: kage サーバ（{}）に到達できません",
            config::endpoint_url()
        );
        println!("  → 別ターミナルで `kage serve` を起動してください（foreground・§5-6）。");
        return Ok(false);
    }

    // (2) 源 ABOUTME の取得可否。サーバはこの実体を無加工配信する（§8-1）ため、
    //     同一ファイルの存在・非空を確認すればローカル面の取得可否と一致する。
    let path = paths::aboutme_path()?;
    let retrieved = match std::fs::read_to_string(&path) {
        Ok(body) if !body.trim().is_empty() => {
            println!("✓ 取得成功: サーバ到達 OK / ABOUTME.md 取得 OK");
            println!("  endpoint: {}", config::endpoint_url());
            println!("  source  : {} ({} bytes)", path.display(), body.len());
            true
        }
        Ok(_) => {
            // 取得失敗（空）。§7-3。
            println!("✗ 取得失敗: サーバ到達 OK / ABOUTME.md が空です");
            println!(
                "  → {} にコンテキストを記述してください（これが SSoT）。",
                path.display()
            );
            false
        }
        Err(e) => {
            // 取得失敗（不在・読めない）。§7-3。
            println!("✗ 取得失敗: サーバ到達 OK / ABOUTME.md を読めません: {e}");
            println!("  → {} を確認してください。", path.display());
            false
        }
    };
    Ok(retrieved)
}

/// kage エンドポイントに最小 HTTP GET を投げ、HTTP 応答が返るかを見る。
/// MCP プロトコルは喋らない（§5-2: 自作しない）。HTTP 層の生存確認のみ。
fn http_alive(addr: &SocketAddr) -> bool {
    let probe = || -> std::io::Result<bool> {
        let mut stream = TcpStream::connect_timeout(addr, TIMEOUT)?;
        stream.set_read_timeout(Some(TIMEOUT))?;
        stream.set_write_timeout(Some(TIMEOUT))?;
        write!(
            stream,
            "GET {} HTTP/1.1\r\nHost: {}:{}\r\nConnection: close\r\n\r\n",
            config::MCP_PATH,
            config::BIND_ADDR,
            config::KAGE_PORT
        )?;
        let mut buf = [0u8; 5];
        stream.read_exact(&mut buf)?;
        Ok(&buf == b"HTTP/")
    };
    probe().unwrap_or(false)
}
