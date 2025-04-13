use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
pub struct Body {
    pub messages: Vec<Message>,
    pub max_tokens: i32,
    pub model: String,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub async fn send(body: &Body) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let api_key =
        env::var("OPENROUTER_API_KEY").map_err(|e| format!("OPENROUTER_API_KEY not set: {}", e))?;

    let response = client
        .post("https://openrouter.ai/api/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(body)
        .send()
        .await?;

    println!("OpenRouter Response Status: {}", response.status());

    if response.status().is_success() {
        let resp_text = response.text().await?;
        println!("OpenRouter Response Body: {}", resp_text);
        Ok(resp_text) // Return the response body string
    } else {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Could not read error body".to_string());
        println!("OpenRouter Error Response Body: {}", error_text);
        Err(format!(
            "OpenRouter request failed with status {}: {}",
            status, error_text
        )
        .into())
    }
}
