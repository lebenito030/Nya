use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "nya")]
#[command(about = "AI聊天命令行工具")]
pub struct Cli {
    #[arg()]
    pub message: Option<String>,
    
    /// 启用交互式对话模式
    #[arg(long, short = 'i')]
    pub interactive: bool,
    
    /// 禁用分页显示(默认启用)
    #[arg(long, short = 'P')]
    pub no_pager: bool,
    
    /// 输出格式(plain/markdown)
    #[arg(long, short = 'f', default_value = "plain")]
    pub format: String,
    
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
