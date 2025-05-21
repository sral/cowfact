use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub temperature: f32,
    pub messages: Vec<Message>,
    pub max_completion_tokens: Option<u32>,
    pub user: Option<String>,
}