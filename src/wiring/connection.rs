//! MCP 設定 JSON への接続配線（§8-5 / ADR-0006）。
//!
//! 配線先は各面の `.claude.json` のトップレベル `mcpServers`。`kage` キーのみを操作し、
//! 他キー・他 MCP サーバには触れない（不変条件4）。整形は実機の 2-space pretty に合わせ diff を最小化する。
use crate::config;
use crate::error::{KageError, Result};
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

/// `.claude.json` のトップレベル `mcpServers.kage` を upsert する。戻り値: 人向けの操作種別。
pub fn upsert(path: &Path) -> Result<&'static str> {
    let existed = path.exists();
    let mut root: Value = if existed {
        let raw = fs::read_to_string(path)?;
        serde_json::from_str(&raw).map_err(|e| {
            KageError::stop(
                format!("{} を JSON として解析できません: {e}", path.display()),
                "ファイルが壊れていないか確認してください（kage は未変更です）",
            )
        })?
    } else {
        json!({})
    };

    let obj = root.as_object_mut().ok_or_else(|| {
        KageError::stop(
            format!(
                "{} のトップレベルが JSON オブジェクトではありません",
                path.display()
            ),
            "想定外の形式です。手動で確認してください",
        )
    })?;

    let desired = json!({ "type": "http", "url": config::endpoint_url() });

    let servers = obj
        .entry("mcpServers")
        .or_insert_with(|| json!({}))
        .as_object_mut()
        .ok_or_else(|| {
            KageError::stop(
                "mcpServers が JSON オブジェクトではありません".to_string(),
                "想定外の形式です。手動で確認してください",
            )
        })?;

    let action = match servers.get("kage") {
        Some(current) if *current == desired => return Ok("変更なし"),
        Some(_) => "更新",
        None if existed => "追加",
        None => "新規ファイル作成",
    };
    servers.insert("kage".to_string(), desired);

    // 実機 .claude.json と同じ 2-space pretty・末尾改行なし（footprint 最小・不変条件4 の精神）。
    fs::write(path, serde_json::to_string_pretty(&root)?)?;
    Ok(action)
}

/// `.claude.json` から `mcpServers.kage` のみを撤去する（§9-3）。
///
/// 削除前に `mcpServers` の存在を検証する（§8-5）。他キー・他 MCP サーバには触れない。
/// 撤去後に空の `mcpServers` が残っても意図的に許容する（§8-5）。
pub fn remove(path: &Path) -> Result<&'static str> {
    if !path.exists() {
        return Ok("ファイル無し（no-op）");
    }
    let raw = fs::read_to_string(path)?;
    let mut root: Value = serde_json::from_str(&raw).map_err(|e| {
        KageError::stop(
            format!("{} を JSON として解析できません: {e}", path.display()),
            "ファイルが壊れていないか確認してください（kage は未変更です）",
        )
    })?;
    let Some(obj) = root.as_object_mut() else {
        return Ok("非オブジェクト（no-op）");
    };
    // §8-5: kage キー削除前に mcpServers の存在を検証する。
    let Some(servers) = obj.get_mut("mcpServers").and_then(|v| v.as_object_mut()) else {
        return Ok("mcpServers 無し（no-op）");
    };
    if servers.remove("kage").is_none() {
        return Ok("kage キー無し（no-op）");
    }
    // 空 mcpServers 残骸は許容（§8-5）。他サーバ設定には触れない。
    fs::write(path, serde_json::to_string_pretty(&root)?)?;
    Ok("mcpServers.kage を削除")
}
