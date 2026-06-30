//! `kage://me` を配信する MCP サーバ実体（§5-5 Resource / §8-1 無加工配信）。
//!
//! Tool / Prompt は公開しない（§5-5: URI 決め打ち参照、モデルのツール選択に委ねない）。
//! read は ABOUTME.md を読むだけ＝SSoT を一切変えない（不変条件1）。
use crate::{config, paths};
use rmcp::{ErrorData as McpError, RoleServer, ServerHandler, model::*, service::RequestContext};

#[derive(Clone)]
pub struct KageServer;

impl KageServer {
    pub fn new() -> Self {
        Self
    }
}

impl Default for KageServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ServerHandler for KageServer {
    fn get_info(&self) -> ServerInfo {
        // ServerInfo(=InitializeResult) は #[non_exhaustive]。struct literal 不可ゆえ
        // default() + フィールド代入で構築する（Phase 0 スパイクで確定した罠）。
        let mut info = ServerInfo::default();
        info.capabilities = ServerCapabilities::builder().enable_resources().build();
        info.server_info = Implementation::new(config::SERVER_NAME, config::SERVER_VERSION);
        info.instructions = Some(
            "kage://me を読むと、ユーザーのグローバル AI コンテキスト（ABOUTME.md）を取得できます。"
                .into(),
        );
        info
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, McpError> {
        Ok(ListResourcesResult {
            resources: vec![
                Resource::new(config::RESOURCE_URI, config::RESOURCE_NAME)
                    .with_mime_type("text/markdown"),
            ],
            ..Default::default()
        })
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, McpError> {
        if request.uri != config::RESOURCE_URI {
            return Err(McpError::resource_not_found(
                format!("未知のリソース: {}", request.uri),
                None,
            ));
        }

        let path =
            paths::aboutme_path().map_err(|e| McpError::internal_error(e.to_string(), None))?;

        // §8-1: 加工・変換せず Markdown をそのまま返す。
        // 取得失敗（不在・読めない）は無言にせず resource_not_found で明示（§7-3 / §10-1）。
        let markdown = std::fs::read_to_string(&path).map_err(|e| {
            McpError::resource_not_found(
                format!("ABOUTME.md を取得できません（{}）: {e}", path.display()),
                None,
            )
        })?;

        Ok(ReadResourceResult::new(vec![ResourceContents::text(
            markdown,
            request.uri,
        )]))
    }
}
