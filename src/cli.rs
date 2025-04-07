use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nya")]
#[command(about = "AI聊天命令行工具")]
pub struct Cli {
    #[arg()]
    pub message: Option<String>,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 配置API设置
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// 显示当前配置
    Show,

    /// 设置API密钥、API地址和模型名称
    Set {
        /// API密钥
        #[arg(long, short = 'k')]
        api_key: Option<String>,

        /// API地址
        #[arg(long, short = 'u')]
        api_url: Option<String>,

        /// 模型名称
        #[arg(long, short = 'm')]
        model: Option<String>,
    },
}
