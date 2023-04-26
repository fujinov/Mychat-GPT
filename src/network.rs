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
    use std::io::{stdout, Write};

    let mut count: u8 = 0;
    let message = "回答を待っています";
    loop {
        let dot = match count % 3 {
            0 => ".  ",
            1 => ".. ",
            2 => "...",
            _ => "   ",
        };
        print!("\0\r");
        print!("{}{}", message, dot);
        stdout().flush().unwrap();
        sleep(Duration::from_secs(1)).await;
        count += 1;
        if count >= 30 {
            println!();
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::{MessageBody, Role};
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
        body.add_message(Role::User, "こんにちは！".to_string());
        let timeout = Duration::from_secs(20);
        let client = Client::builder().timeout(timeout).build().unwrap();
        let url = "https://api.openai.com/v1/chat/completions";
        let response = client
            .post(url)
            .bearer_auth(get_api_key())
            .json(&body)
            .send()
            .await;
        match response {
            Ok(res) => println!("{}", res.text().await.unwrap()),
            Err(e) => println!("error: {}", e),
        }
    }

    #[ignore = "take time"]
    #[tokio::test]
    async fn print_wait_message() {
        let handle1 = tokio::spawn(async {
            sleep(Duration::from_secs(5)).await;
        });
        let handle2 = tokio::spawn(waitting_message());

        let result;
        tokio::select! {
            _ = handle1 => {
                result = "OK";
            }
            _ = handle2 => {
                result = "NG"
            }
        }
        println!();
        assert_eq!("OK", result);
    }
}
