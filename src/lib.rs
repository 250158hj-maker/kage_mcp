//! kage: Windows + WSL2 で `~/.kage/ABOUTME.md`（SSoT）を複数 Claude Code に
//! 横断配信するクロスOS MCP サーバ + 配線 CLI。
//!
//! 設計の最終権威は `docs/基本設計書.md`。本コードのコメントは「なぜ」を §N / ADR で指す。
pub mod cli;
pub mod config;
pub mod error;
pub mod mcp;
pub mod paths;
pub mod scaffold;
