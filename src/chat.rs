use std::{collections::HashMap, vec};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
pub struct Config {
    /// Set GPT "role": "system"
    pub role: Option<String>,

    /// Model specification. Default value if not specified.
    #[arg(short, long, default_value = "gpt-3.5-turbo")]
    pub model: String,

    /// Accept input until EOF comes in.
    /// To insert EOF, "Ctrl+z" for Windows, "Ctrl+d" for Unix systems
    #[arg(short, long)]
    pub lines: bool,

    /// Stream function off.
    /// When turned off, tokens used are displayed at the end of the session
    #[arg(short, long)]
    pub nostream: bool,
}

impl Config {
    pub fn check_model(&self) {
        let mut models = [
            "gpt-4",
            "gpt-4-0314",
            "gpt-4-32k",
            "gpt-4-32k-0314",
            "gpt-3.5-turbo",
            "gpt-3.5-turbo-0301",
        ]
        .into_iter();
        let find = models.find(|x| x == &self.model);
        if find.is_none() {
            panic!("Model name is incorrect");
        }
    }
}
#[derive(Debug, Serialize)]
pub struct MessageBody {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

impl Default for MessageBody {
    fn default() -> Self {
        Self {
            model: String::from("gpt-3.5-turbo"),
            messages: Vec::new(),
            stream: true,
        }
    }
}

impl MessageBody {
    pub fn new(model: String, role: Option<String>, stream: bool) -> Self {
        let mut body = Self {
            model,
            messages: Vec::new(),
            stream,
        };
        if let Some(content) = role {
            body.add_message(Role::System, content)
        }
        body
    }
}

impl MessageBody {
    pub fn add_message(&mut self, role: Role, content: String) {
        let message = Message { role, content };
        self.messages.push(message);
    }

    /// except "role": "sysytem"
    pub fn reset_messages(&mut self) {
        if self.messages.is_empty() {
            println!("--Messages is Empty--");
        } else if self.messages[0].role == Role::System {
            let front = self.messages.swap_remove(0);
            self.messages = vec![front];
            println!("--Messages reset--");
        } else {
            self.messages.clear();
            println!("--Messages reset--");
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

#[derive(Debug, Deserialize)]
pub struct Completion {
    #[serde(rename = "id")]
    _id: String,
    #[serde(rename = "object")]
    _object: String,
    #[serde(rename = "created")]
    _created: u32,
    pub choices: Vec<Choice>,
    pub usage: Tokens,
}

impl Completion {
    pub fn get_total_tokens(&self) -> u32 {
        self.usage.total
    }

    pub fn get_content(mut self) -> String {
        let choice = self.choices.pop().unwrap();
        choice.message.content
    }
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    #[serde(rename = "index")]
    _index: u32,
    pub message: Message,
    #[serde(rename = "finish_reason")]
    _finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct Tokens {
    #[serde(rename = "prompt_tokens")]
    _prompt: u32,
    #[serde(rename = "completion_tokens")]
    _completion: u32,
    #[serde(rename = "total_tokens")]
    pub total: u32,
}

#[derive(Debug, Deserialize)]
pub struct CompletionChunk {
    #[serde(rename = "id")]
    _id: String,
    #[serde(rename = "object")]
    _object: String,
    #[serde(rename = "created")]
    _created: u32,
    #[serde(rename = "model")]
    _model: String,
    pub choices: Vec<ChoiceChunk>,
}

#[derive(Debug, Deserialize)]
pub struct ChoiceChunk {
    pub delta: HashMap<String, String>,
    #[serde(rename = "index")]
    _index: u32,
    #[serde(rename = "finish_reason")]
    _finish_reason: Option<String>,
}

pub fn chunk_to_string(chunk: &[u8]) -> String {
    let chunks = chunk_to_vector(chunk);
    let mut message = String::new();
    for c in chunks {
        if &c == "[DONE]" {
            break;
        }
        let completion: CompletionChunk = serde_json::from_str(&c).unwrap();
        let choice = &completion.choices[0];
        if choice.delta.contains_key("content") {
            message.push_str(&choice.delta["content"]);
        }
    }
    message
}

fn chunk_to_vector(chunk: &[u8]) -> Vec<String> {
    let chunk = std::str::from_utf8(chunk).unwrap().trim_end();
    let mut chunks: Vec<String> = chunk.split("data: ").map(|x| x.to_string()).collect();
    chunks.remove(0);
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn check_model_test() {
        let config = Config {
            model: String::from("gpt-3-turbo"),
            role: None,
            lines: false,
            nostream: false,
        };
        config.check_model();
    }

    #[test]
    fn add_message_test() {
        let mut body = MessageBody::default();
        body.add_message(Role::System, "system".to_string());
        body.add_message(Role::User, "user".to_string());
        println!("{:?}", body);
        assert_eq!(Role::System, body.messages[0].role);
        assert_eq!("user".to_string(), body.messages[1].content);
    }

    #[test]
    fn reset_messages_test() {
        let mut body = MessageBody::default();
        body.add_message(Role::User, "user".to_string());
        body.reset_messages();
        println!("{:?}", body);
        assert!(body.messages.is_empty());

        body.add_message(Role::System, "system".to_string());
        body.add_message(Role::User, "user".to_string());
        body.add_message(Role::Assistant, "assistant".to_string());
        body.reset_messages();
        println!("{:?}", body);
        assert_eq!("system".to_string(), body.messages[0].content);
    }

    #[test]
    fn struct_to_json() {
        let mut body = MessageBody::default();
        body.add_message(Role::System, "system".to_string());
        body.add_message(Role::User, "user".to_string());
        let serialized = serde_json::to_string(&body).unwrap();
        println!("{}", serialized);
    }

    #[test]
    fn json_to_struct() {
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

        let res: Completion = serde_json::from_str(response).unwrap();
        println!("{:?}", res);
        assert_eq!(Role::Assistant, res.choices[0].message.role);
        assert_eq!(21, res.usage.total);
    }

    #[test]
    fn json_to_chunk_message() {
        let chunk =  b"data: {\"id\":\"chatcmpl-7AA\",\"object\":\"chat.completion.chunk\",\"created\":1682,\"model\":\"gpt-3.5-turbo-0301\",\"choices\":[{\"delta\":{\"role\":\"assistant\"},\"index\":0,\"finish_reason\":null}]}\n\ndata: {\"id\":\"chatcmpl-7AA\",\"object\":\"chat.completion.chunk\",\"created\":1682,\"model\":\"gpt-3.5-turbo-0301\",\"choices\":[{\"delta\":{\"content\":\"Hello\"},\"index\":0,\"finish_reason\":null}]}\n\n";
        let s = chunk_to_string(chunk);
        println!("{s}");
    }
}
