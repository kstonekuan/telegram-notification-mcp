use worker::*;

mod telegram;
mod mcp;

use mcp::{handle_mcp_request, McpRequest};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    
    router
        .post_async("/mcp", |mut req, ctx| async move {
            let body = req.text().await?;
            let mcp_request: McpRequest = serde_json::from_str(&body)
                .map_err(|e| Error::RustError(format!("Invalid MCP request: {}", e)))?;
            
            let bot_token = ctx.env.secret("TELEGRAM_BOT_TOKEN")?.to_string();
            let chat_id = ctx.env.secret("TELEGRAM_CHAT_ID")?.to_string();
            
            let response = handle_mcp_request(mcp_request, &bot_token, &chat_id).await?;
            
            Response::from_json(&response)
        })
        .get("/health", |_req, _ctx| {
            Response::ok("Telegram Notification MCP Server is running")
        })
        .run(req, env)
        .await
}