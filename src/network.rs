use std::io::{stdout, Write};
use std::{env, time::Duration};
use tokio::time::sleep;

pub fn get_api_key() -> String {
    let api_key = env::var("OPENAI_API_KEY");
    match api_key {
        Ok(key) => key,
        Err(_) => panic!("環境変数「OPENAI_API_KEY」にトークンを設定してください"),
    }
}

pub async fn waitting_message() {
    let mut count: u8 = 0;
    let message = "回答を待っています";
    loop {
        let dot = match count % 3 {
            0 => ".  ",
            1 => ".. ",
            2 => "...",
            _ => "   ",
        };
        print!("\r");
        print!("{}{}", message, dot);
        stdout().flush().unwrap();
        sleep(Duration::from_secs(1)).await;
        count += 1;
        if count >= 60 {
            println!();
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::{chunk_to_string, MessageBody, Role};
    use reqwest::Client;

    #[test]
    fn check_token() {
        let token = get_api_key();
        println!("////{}////", token);
    }

    #[ignore = "consume tokens"]
    #[tokio::test]
    async fn reqest_test() {
        let mut body = MessageBody::default();
        body.add_message(Role::User, "Hello!".to_string());
        let timeout = Duration::from_secs(60);
        let client = Client::builder().timeout(timeout).build().unwrap();
        let url = "https://api.openai.com/v1/chat/completions";
        let mut response = client
            .post(url)
            .bearer_auth(get_api_key())
            .json(&body)
            .send()
            .await
            .unwrap();

        let mut message = String::new();
        while let Some(chunk) = response.chunk().await.unwrap() {
            let s = chunk_to_string(&chunk);
            print!("{s}");
            stdout().flush().unwrap();
            message.push_str(&s);
        }
        println!();
        println!("Finish: {message}");
    }

    #[ignore = "take time"]
    #[tokio::test]
    async fn print_wait_message() {
        let handle1 = sleep(Duration::from_secs(5));
        let handle2 = waitting_message();

        let result;
        tokio::select! {
            _ = handle1 => {
                result = "OK";
            }
            _ = handle2 => {
                result = "NG"
            }
        }
        print!("\r");
        print!("{}", " ".repeat(30));
        print!("\r");
        assert_eq!("OK", result);
    }
}
