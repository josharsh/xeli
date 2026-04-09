use crate::ai::config::AiConfig;
use anyhow::{bail, Result};
use serde_json::json;

pub async fn query_ai(config: &AiConfig, prompt: &str) -> Result<String> {
    match config.provider.as_str() {
        "openai" => query_openai(config, prompt).await,
        "anthropic" => query_anthropic(config, prompt).await,
        _ => bail!("Unknown AI provider: {}", config.provider),
    }
}

async fn query_openai(config: &AiConfig, prompt: &str) -> Result<String> {
    let api_key = config
        .openai_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No OpenAI API key. Set OPENAI_API_KEY or run: xeli config set-key openai <key>"))?;

    let model = config
        .model
        .as_deref()
        .unwrap_or("gpt-4o-mini");

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a SQL query generator. Output ONLY valid DuckDB SQL. No markdown, no explanation."
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ],
            "temperature": 0.0,
            "max_tokens": 500
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        bail!("OpenAI API error {}: {}", status, body);
    }

    let body: serde_json::Value = response.json().await?;
    let sql = body["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(sql)
}

async fn query_anthropic(config: &AiConfig, prompt: &str) -> Result<String> {
    let api_key = config
        .anthropic_api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("No Anthropic API key. Set ANTHROPIC_API_KEY or run: xeli config set-key anthropic <key>"))?;

    let model = config
        .model
        .as_deref()
        .unwrap_or("claude-sonnet-4-5-20250929");

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&json!({
            "model": model,
            "max_tokens": 500,
            "system": "You are a SQL query generator. Output ONLY valid DuckDB SQL. No markdown, no explanation.",
            "messages": [
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        bail!("Anthropic API error {}: {}", status, body);
    }

    let body: serde_json::Value = response.json().await?;
    let sql = body["content"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();

    Ok(sql)
}
