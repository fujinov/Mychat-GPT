use reqwest::ClientBuilder;
use std::{env, time::Duration};
use tokio::time::sleep;

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

    #[test]
    fn get_token() {
        let token = env::var("OPENAI_API_KEY");
        println!("////{}////", token.unwrap());
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
