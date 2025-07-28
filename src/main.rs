use clap::Parser;
use std::io::{self, Read, Write};
use atty::Stream;
use minus::{Pager, LineNumbers};

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

    let messages = vec![chat::Message {
        role: "user".to_string(),
        content: message.clone(),
    }];
    let response = chat::send_chat_request(messages.as_slice()).await?;
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
    let mut history: Vec<chat::Message> = Vec::new();
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        history.push(chat::Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        let response = chat::send_chat_request(history.as_slice()).await?;
        history.push(chat::Message {
            role: "assistant".to_string(),
            content: response,
        });
    }
    Ok(())
}

fn print_response(response: &str, cli: &cli::Cli) -> anyhow::Result<()> {
    if !cli.no_pager && atty::is(Stream::Stdout) && response.lines().count() > 10 {
        let pager = Pager::new();
        pager.set_line_numbers(LineNumbers::Enabled)?;
        pager.push_str(response)?;
        minus::dynamic_paging(pager)?;
    } else {
        println!("{}", response);
    }
    Ok(())
}

fn handle_command(command: cli::Commands) -> anyhow::Result<()> {
    match command {
        cli::Commands::Config { command } => match command {
            cli::ConfigCommands::Show => config::show_config()?,
            cli::ConfigCommands::Set {
                api_key,
                api_url,
                model,
            } => {
                let config = config::Config {
                    api_key,
                    api_url,
                    model,
                };
                config::update_config(config)?;
                println!("配置更新成功");
            }
        },
    }
    Ok(())
}
