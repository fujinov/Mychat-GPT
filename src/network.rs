use reqwest::ClientBuilder;
use serde_json::json;
use std::time::Duration;

use tokio::time::sleep;

// pub async fn request(url: &str) {
//     let timeout = Duration::new(30, 0);
//     let client = ClientBuilder::new().timeout(timeout).build()?;
//     let res = client.post(url).bearer_auth("d").json(json)
// }

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
        if count >= 20 {
            println!();
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::network::*;
    use crate::chat::*;

    #[test]
    fn json_de_test() {
        let response = r#"
        {
            "id": "chatcmpl-123",
            "object": "chat.completion",
            "created": 1677652288,
            "choices": [{
              "index": 0,
              "message": {
                "role": "assistant",
                "content": "\n\nHello there, how may I assist you today?"
              },
              "finish_reason": "stop"
            }],
            "usage": {
              "prompt_tokens": 9,
              "completion_tokens": 12,
              "total_tokens": 21
            }
          }
          "#;
          
        let res:Completion = serde_json::from_str(response).unwrap();
        assert_eq!("assistant", res.choices[0].message["role"]);
        assert_eq!(21, res.usage["total_tokens"]);
    }

    // #[tokio::test]
    // async fn request_test() {
    //     let res = request("3").await;
    //     match res {
    //         Ok(s) => println!("{}", s),
    //         Err(e) => println!("There is an error.\n{}", e),
    //     }
    // }

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
