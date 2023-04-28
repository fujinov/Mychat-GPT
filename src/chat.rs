use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct Config {
    pub lines: bool,
    pub stream: bool,
}

#[derive(Debug, Serialize)]
pub struct MessageBody {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
}

impl MessageBody {
    pub fn add_message(&mut self, role: Role, content: String) {
        let message = Message { role, content };
        self.messages.push(message);
    }
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
    fn add_message_test() {
        let mut body = MessageBody::default();
        body.add_message(Role::System, "system".to_string());
        body.add_message(Role::User, "user".to_string());
        println!("{:?}", body);
        assert_eq!(Role::System, body.messages[0].role);
        assert_eq!("user".to_string(), body.messages[1].content);
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
