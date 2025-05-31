use serde::{Deserialize, Serialize};
use worker::*;
use crate::telegram::send_telegram_notification;

#[derive(Debug, Deserialize)]
#[serde(tag = "jsonrpc")]
pub enum McpRequest {
    #[serde(rename = "2.0")]
    JsonRpc2_0 {
        method: String,
        params: Option<serde_json::Value>,
        id: Option<serde_json::Value>,
    },
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum McpResponse {
    Result {
        jsonrpc: String,
        result: serde_json::Value,
        id: Option<serde_json::Value>,
    },
    Error {
        jsonrpc: String,
        error: McpError,
        id: Option<serde_json::Value>,
    },
}

#[derive(Debug, Serialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct ServerInfo {
    name: String,
    version: String,
}

#[derive(Debug, Serialize)]
struct InitializeResult {
    #[serde(rename = "protocolVersion")]
    protocol_version: String,
    #[serde(rename = "serverInfo")]
    server_info: ServerInfo,
    capabilities: ServerCapabilities,
}

#[derive(Debug, Serialize)]
struct ServerCapabilities {
    tools: ToolsCapability,
}

#[derive(Debug, Serialize)]
struct ToolsCapability {}

#[derive(Debug, Serialize)]
struct ListToolsResult {
    tools: Vec<Tool>,
}

#[derive(Debug, Serialize)]
struct Tool {
    name: String,
    description: String,
    #[serde(rename = "inputSchema")]
    input_schema: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ToolCallParams {
    name: String,
    arguments: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct NotifyParams {
    message: String,
}

pub async fn handle_mcp_request(
    request: McpRequest,
    bot_token: &str,
    chat_id: &str,
) -> Result<McpResponse> {
    match request {
        McpRequest::JsonRpc2_0 { method, params, id } => {
            match method.as_str() {
                "initialize" => {
                    // Initialize requires an ID
                    if id.is_none() {
                        return Ok(McpResponse::Error {
                            jsonrpc: "2.0".to_string(),
                            error: McpError {
                                code: -32600,
                                message: "Invalid Request: initialize requires an id".to_string(),
                                data: None,
                            },
                            id: None,
                        });
                    }
                    
                    let result = InitializeResult {
                        protocol_version: "2024-11-05".to_string(),
                        server_info: ServerInfo {
                            name: "telegram-notification-mcp".to_string(),
                            version: "0.1.0".to_string(),
                        },
                        capabilities: ServerCapabilities {
                            tools: ToolsCapability {},
                        },
                    };
                    
                    Ok(McpResponse::Result {
                        jsonrpc: "2.0".to_string(),
                        result: serde_json::to_value(result).unwrap(),
                        id,
                    })
                }
                "tools/list" => {
                    let tools = vec![
                        Tool {
                            name: "notify_telegram".to_string(),
                            description: "Send a notification to Telegram. Use this when: task completes, error occurs, user input needed, or any important status update.".to_string(),
                            input_schema: serde_json::json!({
                                "type": "object",
                                "properties": {
                                    "message": {
                                        "type": "string",
                                        "description": "The notification message to send. Include relevant context like: what task completed, what error occurred, what input is needed, etc."
                                    }
                                },
                                "required": ["message"],
                                "additionalProperties": false
                            }),
                        },
                    ];
                    
                    let result = ListToolsResult { tools };
                    
                    Ok(McpResponse::Result {
                        jsonrpc: "2.0".to_string(),
                        result: serde_json::to_value(result).unwrap(),
                        id,
                    })
                }
                "tools/call" => {
                    if let Some(params) = params {
                        if let Ok(tool_call) = serde_json::from_value::<ToolCallParams>(params) {
                            match tool_call.name.as_str() {
                                "notify_telegram" => {
                                    if let Ok(notify_params) = serde_json::from_value::<NotifyParams>(tool_call.arguments) {
                                        send_telegram_notification(bot_token, chat_id, &notify_params.message).await?;
                                        
                                        return Ok(McpResponse::Result {
                                            jsonrpc: "2.0".to_string(),
                                            result: serde_json::json!({
                                                "content": [{
                                                    "type": "text",
                                                    "text": "Notification sent successfully"
                                                }]
                                            }),
                                            id,
                                        });
                                    } else {
                                        return Ok(McpResponse::Error {
                                            jsonrpc: "2.0".to_string(),
                                            error: McpError {
                                                code: -32602,
                                                message: "Invalid parameters for notify_telegram".to_string(),
                                                data: None,
                                            },
                                            id,
                                        });
                                    }
                                }
                                _ => {
                                    return Ok(McpResponse::Error {
                                        jsonrpc: "2.0".to_string(),
                                        error: McpError {
                                            code: -32602,
                                            message: format!("Unknown tool: {}", tool_call.name),
                                            data: None,
                                        },
                                        id,
                                    });
                                }
                            }
                        }
                    }
                    
                    Ok(McpResponse::Error {
                        jsonrpc: "2.0".to_string(),
                        error: McpError {
                            code: -32602,
                            message: "Invalid params: expected {name: string, arguments: object}".to_string(),
                            data: None,
                        },
                        id,
                    })
                }
                _ => Ok(McpResponse::Error {
                    jsonrpc: "2.0".to_string(),
                    error: McpError {
                        code: -32601,
                        message: format!("Method not found: {}", method),
                        data: None,
                    },
                    id,
                }),
            }
        }
    }
}