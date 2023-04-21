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
            sleep(tokio::time::Duration::from_secs(5)).await;
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
