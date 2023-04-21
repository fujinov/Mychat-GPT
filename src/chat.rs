use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Completion {
    _id: String,
    _object: String,
    _created: usize,
    pub choices: Vec<Choice>,
    pub usage: HashMap<String, usize>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    _index: usize,
    pub message: HashMap<String, String>,
    _finish_reason: String,
}