use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct MessageBody {
    pub model: String,
    pub messages: Vec<Message>,
}

impl MessageBody {
    pub fn new(model: Option<&str>) -> Self {
        Self {
            model: String::from(model.unwrap_or("gpt-3.5-turbo")),
            messages: Vec::new(),
        }
    }

    pub fn add_message(&mut self, role: Role, content: String) {
        let message = Message { role, content };
        self.messages.push(message);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_message_test() {
        let mut body = MessageBody::new(None);
        body.add_message(Role::System, "system".to_string());
        body.add_message(Role::User, "user".to_string());
        println!("{:?}", body);
        assert_eq!(Role::System, body.messages[0].role);
        assert_eq!("user".to_string(), body.messages[1].content);
    }

    #[test]
    fn struct_to_json() {
        let mut body = MessageBody::new(None);
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
}
