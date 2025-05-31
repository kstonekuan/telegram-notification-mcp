use worker::*;
use futures::stream::{self, StreamExt};
use std::time::Duration;

mod telegram;
mod mcp;

use mcp::{handle_mcp_request, McpRequest};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    
    router
        .get_async("/mcp", |req, _ctx| async move {
            // SSE endpoint - establishes the connection
            let mut headers = Headers::new();
            headers.set("Content-Type", "text/event-stream")?;
            headers.set("Cache-Control", "no-cache")?;
            headers.set("Connection", "keep-alive")?;
            headers.set("Access-Control-Allow-Origin", "*")?;
            headers.set("X-Accel-Buffering", "no")?; // Disable Nginx buffering
            
            // Send the endpoint event immediately
            // The endpoint must be a full URL for the client to POST to
            let url = req.url()?;
            let host = url.host_str().unwrap_or("localhost");
            let port = url.port().map(|p| format!(":{}", p)).unwrap_or_default();
            let protocol = if host == "localhost" { "http" } else { "https" };
            let endpoint_url = format!("{}://{}{}/mcp", protocol, host, port);
            
            // Create a stream that sends the endpoint event and then keeps the connection alive
            let endpoint_event = format!("event: endpoint\ndata: {}\n\n", endpoint_url);
            
            // Create an infinite stream that yields the endpoint event immediately and then ping events
            let stream = stream::once(async move { 
                Ok::<_, Error>(endpoint_event.into_bytes()) 
            })
                .chain(
                    // Use unfold to create an infinite stream with delays
                    stream::unfold((), |_| async {
                        // Wait 30 seconds between pings
                        worker::Delay::from(Duration::from_secs(30)).await;
                        Some((Ok::<_, Error>(":ping\n\n".to_string().into_bytes()), ()))
                    })
                );
            
            Ok(Response::from_stream(stream)?.with_headers(headers))
        })
        .post_async("/mcp", |mut req, ctx| async move {
            // Message endpoint - handles MCP requests
            let bot_token = match ctx.env.secret("TELEGRAM_BOT_TOKEN") {
                Ok(token) => token.to_string(),
                Err(e) => return Err(e),
            };
            
            let chat_id = match ctx.env.secret("TELEGRAM_CHAT_ID") {
                Ok(id) => id.to_string(),
                Err(e) => return Err(e),
            };
            
            // Parse the request body
            let body = match req.text().await {
                Ok(b) => b,
                Err(e) => return Err(e),
            };
            
            let mcp_request: McpRequest = match serde_json::from_str(&body) {
                Ok(req) => req,
                Err(e) => return Err(Error::RustError(format!("Invalid JSON: {}", e))),
            };
            
            // Check if this is a notification (no id field)
            let is_notification = match &mcp_request {
                McpRequest::JsonRpc2_0 { id, method, params } => {
                    if id.is_none() {
                        // Handle notifications that we care about
                        match method.as_str() {
                            "initialized" => {
                                // Client has completed initialization
                                true
                            }
                            "$/userInputRequired" | "$/progress" | "$/error" | "$/taskComplete" => {
                                // Send telegram notification
                                if let Some(params) = params {
                                    let (icon, default_msg) = match method.as_str() {
                                        "$/userInputRequired" => ("⚠️", "User Input Required"),
                                        "$/progress" => ("⏳", "Task in Progress"),
                                        "$/error" => ("❌", "Error Occurred"),
                                        "$/taskComplete" => ("✅", "Task Completed"),
                                        _ => ("📢", "Notification"),
                                    };
                                    
                                    let message = params.get("message")
                                        .or_else(|| params.get("description"))
                                        .and_then(|m| m.as_str())
                                        .unwrap_or(default_msg);
                                    
                                    let _ = crate::telegram::send_telegram_notification(
                                        &bot_token,
                                        &chat_id,
                                        &format!("{} {}\n\n{}", icon, default_msg, message)
                                    ).await;
                                }
                                true
                            }
                            _ => true // Other notifications we don't handle
                        }
                    } else {
                        false
                    }
                }
            };
            
            // Notifications don't get responses
            if is_notification {
                return Response::empty();
            }
            
            // Handle the request and get response
            let response = match handle_mcp_request(mcp_request, &bot_token, &chat_id).await {
                Ok(resp) => resp,
                Err(e) => return Err(e),
            };
            
            // Return JSON response (not SSE for POST)
            let mut headers = Headers::new();
            headers.set("Content-Type", "application/json")?;
            headers.set("Access-Control-Allow-Origin", "*")?;
            
            Ok(Response::from_json(&response)?.with_headers(headers))
        })
        .options("/mcp", |_req, _ctx| {
            // Handle CORS preflight requests
            let mut headers = Headers::new();
            headers.set("Access-Control-Allow-Origin", "*")?;
            headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
            headers.set("Access-Control-Allow-Headers", "Content-Type, Accept")?;
            headers.set("Access-Control-Max-Age", "86400")?;
            
            Ok(Response::empty()?.with_headers(headers))
        })
        .get("/health", |_req, _ctx| {
            Response::ok("Telegram Notification MCP Server is running")
        })
        .run(req, env)
        .await
}