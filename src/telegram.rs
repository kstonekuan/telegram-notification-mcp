use worker::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct TelegramMessage {
    chat_id: String,
    text: String,
    parse_mode: String,
}

#[derive(Debug, Deserialize)]
struct TelegramResponse {
    ok: bool,
    description: Option<String>,
}

pub async fn send_telegram_notification(
    bot_token: &str,
    chat_id: &str,
    message: &str,
) -> Result<()> {
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    
    let telegram_message = TelegramMessage {
        chat_id: chat_id.to_string(),
        text: format!("🤖 Claude Code Notification\n\n{}", message),
        parse_mode: "HTML".to_string(),
    };
    
    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    
    let mut headers = Headers::new();
    headers.set("Content-Type", "application/json")?;
    init.with_headers(headers);
    
    let body = serde_json::to_string(&telegram_message).unwrap();
    init.with_body(Some(body.into()));
    
    let request = Request::new_with_init(&url, &init)?;
    
    let mut response = Fetch::Request(request).send().await?;
    let response_text = response.text().await?;
    
    let telegram_response: TelegramResponse = serde_json::from_str(&response_text)
        .map_err(|e| Error::RustError(format!("Failed to parse Telegram response: {}", e)))?;
    
    if !telegram_response.ok {
        return Err(Error::RustError(format!(
            "Telegram API error: {}",
            telegram_response.description.unwrap_or_else(|| "Unknown error".to_string())
        )));
    }
    
    Ok(())
}