//! A minimal MCP server that talks over stdio, which is how a client such as
//! Claude Desktop runs a bundled binary.

use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{Implementation, ServerCapabilities, ServerConfig};
use rmcp::{ServerHandler, ServiceExt, schemars, tool, tool_handler, tool_router};
use serde::Deserialize;

#[derive(Debug, Clone, Default)]
struct Server;

#[derive(Deserialize, schemars::JsonSchema)]
struct GreetParams {
    /// Who to greet.
    name: String,
}

#[tool_router]
impl Server {
    #[tool(description = "Greet someone by name.")]
    fn greet(&self, Parameters(GreetParams { name }): Parameters<GreetParams>) -> String {
        format!("Hello, {name}!")
    }
}

#[tool_handler]
impl ServerHandler for Server {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build()).with_server_info(
            Implementation::new(env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
        )
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // stdout carries the protocol, so anything else goes to stderr.
    let service = Server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}
