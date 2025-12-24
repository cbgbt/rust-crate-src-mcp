use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{
        CallToolResult, Content, ErrorCode, ErrorData, Implementation, ServerCapabilities,
        ServerInfo,
    },
    tool, tool_handler, tool_router,
    transport::stdio,
};
use rust_crate_src_mcp::get_crate_source;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Clone)]
pub struct Server {
    tool_router: ToolRouter<Self>,
}

#[derive(Deserialize, JsonSchema)]
struct GetCrateSourceRequest {
    crate_name: String,
    version: Option<String>,
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl Server {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Download and extract Rust crate source code from crates.io. Use this to:
- Debug compilation errors by reading the source of dependencies
- Understand how a crate implements specific functionality
- Evaluate potential dependencies before adding them
- Find usage examples in a crate's internal code"
    )]
    async fn get_rust_crate_source(
        &self,
        params: Parameters<GetCrateSourceRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        let req = params.0;
        let result = get_crate_source(&req.crate_name, req.version.as_deref())
            .await
            .map_err(|e| ErrorData::new(ErrorCode(-1), e.to_string(), None))?;
        let json = serde_json::json!({
            "crate_name": result.crate_name,
            "version": result.version,
            "checkout_path": result.checkout_path.display().to_string(),
            "message": result.message,
        });
        Ok(CallToolResult::success(vec![Content::text(
            json.to_string(),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = Server::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
