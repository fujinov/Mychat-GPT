struct Chat {
    title: String,
    role: Option<String>,
    user_messages: Vec<String>,
    gpt_messages: Vec<String>,
    tokens: u32,
}

impl Chat {
    fn new() -> Chat {
        Chat {
            title: String::new(),
            role: None,
            user_messages: Vec::new(),
            gpt_messages: Vec::new(),
            tokens: 0,
        }
    }
}

#[tokio::main]
async fn main() {}
