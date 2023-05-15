pub mod chat;
pub mod file;
pub mod network;

use std::io::{self, Read};

use self::chat::MessageBody;

pub const DEFALT_NAME: &str = "You";

pub fn print_manual() {
    println!("{}", "*".repeat(40));
    println!("*{0: ^38}*", "Start chatting");
    println!("*{0: ^38}*", "Quit with a blank line or \":q\"");
    println!("*{0: ^38}*", "Reset messages with \":r\"");
    println!("*{0: ^38}*", "Save and Quit with \":sq\"");
    println!("*{0: ^38}*", "Save and Reset with \":sr\"");
    println!("{}", "*".repeat(40));
}

pub fn input_line() -> std::io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim_end().to_string())
}

/// Accept input until EOF comes in.
/// To insert EOF, "Ctrl+z" for Windows, "Ctrl+d" for Unix systems
pub fn input_lines() -> std::io::Result<String> {
    let mut lines = String::new();
    io::stdin().read_to_string(&mut lines)?;
    Ok(lines.trim_end().to_string())
}

pub fn response_error(body: &mut MessageBody) {
    println!("Response acquisition error");
    println!("Please enter the content again");
    body.messages.pop();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_manual_test() {
        print_manual();
    }

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
