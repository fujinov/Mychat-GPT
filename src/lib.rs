pub mod chat;
pub mod file;
pub mod network;

use std::io;

use self::chat::MessageBody;

pub fn input_line() -> std::io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim_end().to_string())
}

/// 空行（\n）がくるまで入力を受け付け
pub fn input_lines() -> std::io::Result<String> {
    let mut lines = String::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s)?;
        if s == "\n" {
            break;
        } else {
            lines.push_str(&s);
        }
    }
    Ok(lines.trim_end().to_string())
}

pub fn response_error(body: &mut MessageBody) {
    println!("レスポンス取得エラー");
    println!("もう一度内容を入力してください。");
    body.messages.pop();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "requires input"]
    #[test]
    fn read_test() {
        let s = input_lines();
        match s {
            Ok(s) => println!("Ok: {s}"),
            Err(e) => println!("Err: {e}"),
        }
    }
}
