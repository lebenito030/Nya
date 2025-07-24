use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub api_key: Option<String>,
    pub api_url: Option<String>,
    pub model: Option<String>,
}

pub fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("无法找到用户目录")
        .join(".config/nya/.nya_config.json")
}

pub fn update_config(
    Config { api_key, api_url, model }: Config
) -> Result<()> {
    let config_path = get_config_path();
    let mut config: Config = if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        serde_json::from_str(&content)?
    } else {
        Config::default()
    };

    config.api_key = api_key.or(config.api_key);
    config.api_url = api_url.or(config.api_url);
    config.model = model.or(config.model);

    let json = serde_json::to_string_pretty(&config)?;
    
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    fs::write(config_path, json)?;

    let _ = show_config();
    
    Ok(())
}

pub fn show_config() -> anyhow::Result<()> {
    let config_path = get_config_path();
    println!("配置文件路径: {}", config_path.display());

    if config_path.exists() {
        let content = fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&content)?;

        println!("当前配置:");
        println!("  API Key: {}", config.api_key.as_deref().unwrap_or("未设置"));
        println!("  API URL: {}", config.api_url.as_deref().unwrap_or("未设置"));
        println!("  Model: {}", config.model.as_deref().unwrap_or("未设置"));
    } else {
        println!("配置文件不存在!");
    }
    Ok(())
}
