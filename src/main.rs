use clap::Parser;

mod chat;
mod cli;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    match cli.message {
        Some(message) => {
            if let Err(e) = chat::send_chat_request(&message).await {
                println!("发送聊天请求失败: {}", e);
                return Err(e);
            }
        },
        None => match cli.command {
            Some(command) => handle_command(command)?,
            None => println!("没有提供任何命令或消息"),
        }
    }

    Ok(())
}

fn handle_command(command: cli::Commands) -> anyhow::Result<()> {
    match command {
        cli::Commands::Config { command } => match command {
            cli::ConfigCommands::Show => config::show_config()?,
            cli::ConfigCommands::Set { api_key, api_url, model } => {
                let config = config::Config { 
                    api_key, 
                    api_url, 
                    model 
                };
                config::update_config(config)?;
                println!("配置更新成功");
            }
        }
    }
    Ok(())
}

