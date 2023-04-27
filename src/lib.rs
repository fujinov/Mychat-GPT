use std::io;

pub mod chat;
pub mod file;
pub mod network;

pub fn input_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

pub fn input_lines() -> String {
    let mut lines = String::new();

    loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        if s == "\n" {
            break;
        } else {
            lines.push_str(&s);
        }
    }
    lines.trim_end().to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_test() {
        let s = input_lines();
        if s == "q" || s == "quit" {
            println!("end");
        }
    }
}
