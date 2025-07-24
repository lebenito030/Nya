use clap::Parser;
use std::io::{self, Read, Write};
use atty::Stream;

mod chat;
mod cli;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    if cli.interactive {
        interactive_loop().await?;
        return Ok(());
    }

    let mut pipe_content = String::new();
    if !atty::is(Stream::Stdin) {
        io::stdin().read_to_string(&mut pipe_content)?;
    }

    let message = match cli.message {
        Some(msg) => {
            if !pipe_content.is_empty() {
                format!("{}\n\n{}", pipe_content, msg)
            } else {
                msg
            }
        }
        None => {
            if !pipe_content.is_empty() {
                pipe_content
            } else {
                // 如果没有管道内容也没有消息，则检查是否有其他命令
                if let Some(cmd) = cli.command {
                    handle_command(cmd)?;
                } else {
                    println!("没有提供任何命令或消息");
                }
                return Ok(());
            }
        }
    };

    let response = chat::send_chat_request(&message).await?;
    print_response(
        &response,
        &cli::Cli {
            message: Some(message),
            interactive: cli.interactive,
            no_pager: cli.no_pager,
            format: cli.format,
            command: None,
        },
    )?;

    Ok(())
}

async fn interactive_loop() -> anyhow::Result<()> {
    println!("进入交互模式(输入'exit'退出)");
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        chat::send_chat_request(input).await?;
    }
    Ok(())
}

fn print_response(response: &str, cli: &cli::Cli) -> anyhow::Result<()> {
    if !cli.no_pager && response.lines().count() > 10 {
        // 使用pager显示长内容
        let mut pager = std::process::Command::new("less")
            .arg("-RF")
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        {
            let mut stdin = pager.stdin.take().unwrap(); // 获取子进程的标准输入
            write!(stdin, "{}", response)?; // 写入响应内容
            // 子进程的输入流会在作用域结束时自动关闭
        }

        pager.wait()?; // 等待子进程完成
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
