use std::io::{stdout, Write};
use std::time::Duration;

use mychat_gpt::chat::*;
use mychat_gpt::file::{get_api_key, save_file};
use mychat_gpt::network::waitting_message;
use mychat_gpt::{input_line, input_lines, response_error};

use clap::Parser;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let config = Config::parse();
    config.check_model();
    let mut body = MessageBody::new(config.model, config.role, !config.nostream);
    let mut tokens: u32 = 0;

    let timeout = Duration::from_secs(45);
    let url = "https://api.openai.com/v1/chat/completions";
    let api_key = get_api_key();
    let client = Client::builder().timeout(timeout).build().unwrap();

    println!("***********************************");
    println!("*      チャットをはじめます       *");
    println!("*      q または quit で終了       *");
    println!("*   s または save で保存して終了  *");
    println!("***********************************");
    loop {
        println!("<あなた>");
        let user = match config.lines {
            true => input_lines(),
            false => input_line(),
        };
        if user == "q" || user == "quit" {
            if config.nostream {
                println!("Total: {tokens}tokens");
            }
            break;
        } else if user == "s" || user == "save" {
            let state = save_file(&body);
            match state {
                Ok(m) | Err(m) => println!("--{m}--"),
            }
            if config.nostream {
                println!("Total {tokens}tokens");
            }
            break;
        }
        body.add_message(Role::User, user);

        let mut gpt = String::new();
        if body.stream {
            let reqest = client
                .post(url)
                .bearer_auth(&api_key)
                .json(&body)
                .send()
                .await;
            let mut response = match reqest {
                Ok(res) => res,
                Err(_) => {
                    response_error(&mut body);
                    continue;
                }
            };
            println!("<ChatGPT>");
            while let Some(chunk) = response.chunk().await.unwrap() {
                let s = chunk_to_string(&chunk);
                print!("{s}");
                stdout().flush().unwrap();
                gpt.push_str(&s);
            }
            println!();
        } else {
            let print_message = waitting_message();
            let reqest = async {
                client
                    .post(url)
                    .bearer_auth(&api_key)
                    .json(&body)
                    .send()
                    .await
            };
            let response;
            tokio::select! {
                _ = print_message => {
                    response_error(&mut body);
                    continue;
                }
                res = reqest => {
                    print!("\r");
                    print!("{}", " ".repeat(30));
                    print!("\r");
                    response = res;
                }
            }
            let res = match response {
                Ok(res) => res.text().await.unwrap(),
                Err(_) => {
                    response_error(&mut body);
                    continue;
                }
            };

            let chat_completion: Completion = serde_json::from_str(&res).unwrap();
            tokens += chat_completion.get_total_tokens();
            gpt.push_str(&chat_completion.get_content());
            println!("<ChatGPT>");
            println!("{gpt}");
        }
        body.add_message(Role::Assistant, gpt);
    }
}
