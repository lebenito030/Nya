use crate::config::{Config, get_config_path};
use anyhow::Result;
use reqwest;
use serde_json::json;
use std::fs;
use std::io::Write;
use std::time::Duration;
use tokio::time;

pub async fn send_chat_request(message: &str) -> Result<()> {
    let config_path = get_config_path();
    let config_content = fs::read_to_string(config_path)?;
    let config: Config = serde_json::from_str(&config_content)?;

    let api_url = config
        .api_url
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("API URL未配置"))?;
    let api_key = config
        .api_key
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("API密钥未配置"))?;

    let client = reqwest::Client::new();
    let mut response = client
        .post(api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": config.model,
            "messages": [{"role": "user", "content": message}],
            "stream": true
        }))
        .send()
        .await?;

    // 使用 text_stream 替代 bytes_stream
    while let Some(chunk) = response.chunk().await? {
        let chunk_str = String::from_utf8_lossy(&chunk);

        // 解析 SSE 格式的数据
        for line in chunk_str.lines() {
            if line.starts_with("data: ") {
                let json_str = &line[6..];
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(json_str) {
                    if let Some(content) = json_value["choices"][0]["delta"]["content"].as_str() {
                        // 逐字输出，增加一个小延迟增强打字效果
                        for char in content.chars() {
                            print!("{}", char);
                            std::io::stdout().flush().unwrap();
                            time::sleep(Duration::from_millis(20)).await;
                        }
                    }
                }
            }

            // 检查流是否结束
            if line.contains("[DONE]") {
                break;
            }
        }
    }

    println!(); // 最后换行
    Ok(())
}
