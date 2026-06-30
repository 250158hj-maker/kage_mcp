//! 各面への配線（§7-2）。prompt（CLAUDE.md ポインタ）と connection（MCP 設定 JSON）。
//!
//! 非対称: prompt は自分の痕跡（マーカー範囲）・connection は他者共有（キーのみ）。
//! これは ADR-0001 ②非侵襲の適用（§8-5）。
pub mod connection;
pub mod prompt;
