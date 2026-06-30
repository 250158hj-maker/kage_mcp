//! 固定値の唯一の定義箇所（§8-4: 実行時に変わらない値はコードに焼く。二重管理＝SSoT 違反）。
use std::net::Ipv4Addr;

/// バインドアドレス。loopback 限定（§5-3。DNS rebinding 回避＝rmcp 既定の allowed_hosts と整合）。
pub const BIND_ADDR: Ipv4Addr = Ipv4Addr::LOCALHOST;

/// 固定ポート（§5-7 / ADR-0004）。衝突しても別ポートへ逃げない。番号は実装時確定 = 7373。
pub const KAGE_PORT: u16 = 7373;

/// HTTP 上の MCP マウントパス。
pub const MCP_PATH: &str = "/mcp";

/// MCP リソース URI（§8-1）と表示名。URI 決め打ち参照（§5-5: モデルの選択に委ねない）。
pub const RESOURCE_URI: &str = "kage://me";
pub const RESOURCE_NAME: &str = "me";

/// initialize の serverInfo に申告する自己情報。
pub const SERVER_NAME: &str = "kage";
pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

/// 各面の `.claude.json` に焼くエンドポイント文字列（§8-5 / ADR-0006）。
/// 固定ポートゆえ常に同一文字列（§5-7: クロスOS追従問題を構造的に消す）。
pub fn endpoint_url() -> String {
    format!("http://{BIND_ADDR}:{KAGE_PORT}{MCP_PATH}")
}
