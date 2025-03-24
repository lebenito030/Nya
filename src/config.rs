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

    config.api_key = api_key;
    config.api_url = api_url;
    config.model = model;

    let json = serde_json::to_string_pretty(&config)?;
    fs::write(config_path, json)?;
    
    println!("配置更新成功!");
    Ok(())
}
