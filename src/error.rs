//! 横断エラー型（§10-1: silent failure 禁止）。
//!
//! stop（ADR-0001: 取るべき手が一意でない/他者領域に触れる）を「メッセージ＋次アクション(hint)」
//! で表現し、無言の失敗を構造的に防ぐ。self-heal 可否（①一意 ∧ ②非侵襲）の判断は呼び出し側が持ち、
//! ここは「止める」を運ぶ器に徹する。
use std::fmt;

pub type Result<T> = std::result::Result<T, KageError>;

#[derive(Debug)]
pub struct KageError {
    message: String,
    /// stop 時に観測者（ユーザー / AI エージェント）へ示す次アクション。
    hint: Option<String>,
}

impl KageError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            hint: None,
        }
    }

    /// stop。失敗を無言にせず、次に取るべき手(hint)を必ず添える（§10-1）。
    pub fn stop(message: impl Into<String>, hint: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            hint: Some(hint.into()),
        }
    }

    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }
}

impl fmt::Display for KageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for KageError {}

impl From<std::io::Error> for KageError {
    fn from(e: std::io::Error) -> Self {
        KageError::new(format!("I/O エラー: {e}"))
    }
}

impl From<serde_json::Error> for KageError {
    fn from(e: serde_json::Error) -> Self {
        KageError::new(format!("JSON エラー: {e}"))
    }
}
