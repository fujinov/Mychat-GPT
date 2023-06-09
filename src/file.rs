use std::env;
use std::fs::{DirBuilder, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::chat::{MessageBody, Role};
use crate::DEFALT_NAME;

use chrono::Local;

/// Get the API key from "./.apikey" or the environment variable "OPENAI_API_KEY"
pub fn get_api_key() -> String {
    let api_key = get_api_from_file();
    if let Some(key) = api_key {
        return key;
    }

    let api_key = env::var("OPENAI_API_KEY");
    match api_key {
        Ok(key) => key,
        Err(_) => panic!(
            "Save the API key in the file \"./.apikey\" or set the API key in the environment variable \"OPENAI_API_KEY\""
        ),
    }
}

fn get_api_from_file() -> Option<String> {
    let path = Path::new("./.apikey");
    if path.is_file() {
        let mut api_key = String::new();
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut api_key).unwrap();
        Some(api_key.trim_end().to_string())
    } else {
        None
    }
}

pub fn save_file(body: &MessageBody) -> Result<&str, &str> {
    let length = body.messages.len();
    if length < 2 {
        return Err("There is no log to save");
    }

    let mut file = access_file();
    let mut contents = String::new();

    for message in &body.messages[0..2] {
        if message.role == Role::User {
            let title = format!("# {}\n", message.content);
            contents.push_str(&title);
        }
    }

    for message in &body.messages {
        let role = match message.role {
            Role::System => "GPT's Role",
            Role::Assistant => "ChatGPT",
            Role::User => DEFALT_NAME,
        };
        let text = format!("## {role}\n");
        contents.push_str(&text);

        contents.push_str(&message.content);
        contents.push('\n');
    }
    contents.push('\n');
    file.write_all(contents.as_bytes())
        .expect("Failed to save log");
    Ok("Log saved")
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
    fn get_api_from_file_test() {
        let api = get_api_from_file();
        match api {
            Some(api) => println!("apikey: {api}"),
            None => println!("None"),
        }
    }

    #[test]
    fn get_api_key_test() {
        let api = get_api_key();
        println!("apikey: {api}");
    }

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
