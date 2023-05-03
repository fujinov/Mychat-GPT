use std::fs::{DirBuilder, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::chat::{MessageBody, Role};

use chrono::Local;

pub fn save_file(body: &MessageBody) -> Result<&str, &str> {
    let length = body.messages.len();
    if length < 2 {
        return Err("保存するログがありません");
    }
    if length == 2 && body.messages[0].role == Role::System {
        return Err("保存するログがありません");
    }

    let mut file = access_file();
    let mut contents = String::new();

    for message in &body.messages[0..2] {
        if message.role != Role::User {
            continue;
        }
        let count = message.content.chars().count();
        let title = if count >= 25 {
            let (i, _) = message.content.char_indices().nth(25).unwrap();
            &message.content[..i]
        } else {
            &message.content
        };
        let text = format!("# {title}\n");
        contents.push_str(&text);
    }

    for message in &body.messages {
        let role = match message.role {
            Role::System => "システム設定",
            Role::Assistant => "ChatGPT",
            Role::User => "あなた",
        };
        let text = format!("## {role}\n");
        contents.push_str(&text);

        contents.push_str(&message.content);
        contents.push('\n');
    }
    contents.push('\n');
    file.write_all(contents.as_bytes())
        .expect("ログの保存に失敗しました");
    Ok("ログを保存しました")
}

fn access_file() -> std::fs::File {
    let mut dir_path = access_dir();
    let mut date = get_date();
    date.push_str(".md");
    dir_path.push(date);
    let file_path = dir_path.as_path();

    if file_path.is_file() {
        OpenOptions::new().append(true).open(file_path).unwrap()
    } else {
        OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap()
    }
}

fn access_dir() -> PathBuf {
    let path = Path::new("./chat/");
    if !path.is_dir() {
        DirBuilder::new().create(path).unwrap();
    }
    path.to_owned()
}

fn get_date() -> String {
    let now = Local::now().date_naive();
    now.format("%Y%m%d").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat::*;

    #[test]
    fn exist_file() {
        let file = access_file();
        let file_type = file.metadata().unwrap().file_type();
        assert_eq!(file_type.is_file(), true);
    }

    #[test]
    fn save_test() {
        let mut body = MessageBody::default();
        body.add_message(Role::System, "system".to_string());
        body.add_message(Role::User, "user".to_string());
        _ = save_file(&body);
    }
}
