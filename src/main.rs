use clap::Parser;
use std::io::{self, Write};

mod chat;
mod cli;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    if cli.interactive {
        interactive_loop().await?;
    } else {
        // 提前提取所有需要的字段
        let format = cli.format.clone();
        let interactive = cli.interactive;
        let no_pager = cli.no_pager;
        let message = cli.message;
        let command = cli.command;

        match message {
            Some(msg) => {
                let response = chat::send_chat_request(&msg).await?;
                print_response(
                    &response,
                    &cli::Cli {
                        message: Some(msg),
                        interactive,
                        no_pager,
                        format,
                        command: None,
                    },
                )?;
            }
            None => match command {
                Some(cmd) => handle_command(cmd)?,
                None => println!("没有提供任何命令或消息"),
            },
        }
    }

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

