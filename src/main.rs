use std::time::Duration;

use mychat_gpt::chat::*;
use mychat_gpt::network::{get_api_key, waitting_message};
use mychat_gpt::{input_line, input_lines};

use reqwest::Client;

#[tokio::main]
async fn main() {
    let config = Config { lines: false };
    let mut tokens: u32 = 0;
    let mut body = MessageBody::default();

    let timeout = Duration::from_secs(45);
    let url = "https://api.openai.com/v1/chat/completions";
    let api_key = get_api_key();
    let client = Client::builder().timeout(timeout).build().unwrap();

    println!("*** チャットをはじめます ***");
    println!("** q もしくは quit で終了 **");
    // println!("** s もしくは save で保存して終了 **");
    loop {
        println!("<あなた>");
        let user = match config.lines {
            true => input_lines(),
            false => input_line(),
        };
        if user == "q" || user == "quit" {
            println!("{tokens}");
            break;
        } //else if end == "s" || end == "save" {
          //     save_file()
          // }
        body.add_message(Role::User, user);

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
                println!("レスポンス取得エラー");
                println!("もう一度内容を入力してください。");
                body.messages.pop();
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
            Err(e) => {
                println!("エラー: {e}");
                println!("もう一度内容を入力してください。");
                body.messages.pop();
                continue;
            }
        };

        let chat_completion: Completion = serde_json::from_str(&res).unwrap();
        tokens += chat_completion.get_total_tokens();
        let gpt = chat_completion.get_content();
        println!("<ChatGPT>");
        println!("{gpt}");
        body.add_message(Role::Assistant, gpt);
    }
}
