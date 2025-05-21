use serde_json::Value;
use std::env;

mod api;
use api::{ChatRequest, Message};

const MODEL_NAME: &str = "gpt-3.5-turbo";
const OPENAI_API_KEY: &str = "OPENAI_API_KEY";
const MAX_COMPLETION_TOKENS: u32 = 100;
const BASE_API_URL: &str = "https://api.openai.com/v1";

const PROMPT: &str = "You are a excited biologist specializing in the study of cows. 
Tell an an amsuing fact about cows in one sentence. 
The fact should be something that is not widely known and may include: 
 - popular culture (e.g. music, song lyrics, movies, books, etc.)
 - history (e.g. events, figures, notable cows etc.)
 - science (e.g. genetics, physiology, etc.)
 - nature (e.g. animals, plants, etc.)
 - technology (e.g. engineering, software, etc.)
 - art (e.g. literature, painting, etc.)
 - food (e.g. recipes, ingredients, etc.)
 - other (e.g. random facts, etc.)";

const MAX_LINE_LENGTH: usize = 80;
const OUTPUT_TEMPLATE: &str = r#"
{cow_fact}
    \   ^__^
     \  (oo)\_______
        (__)\       )\/\
            ||----w |
            ||     ||
"#;

fn wrap_text(text: &str, max_length: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.len() + word.len() + 1 <= max_length {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            lines.push(format!(" ( {:<width$} )", current_line, width = max_length));
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(format!(" ( {:<width$} )", current_line, width = max_length));
    }
    lines.join("\n")
}

fn main() {
    let api_key = match env::var(OPENAI_API_KEY) {
        Ok(key) => key,
        Err(e) => {
            println!("Error getting API key: {}", e);
            return;
        }
    };
    let url = format!("{}/chat/completions", BASE_API_URL);

    let client = reqwest::blocking::Client::new();
    let request = ChatRequest {
        model: MODEL_NAME.to_string(),
        temperature: 1.2,
        messages: vec![Message {
            role: "user".to_string(),
            content: PROMPT.to_string(),
        }],
        max_completion_tokens: Some(MAX_COMPLETION_TOKENS),
        user: Some("user".to_string()),
    };

    let response = match client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
    {
        Ok(res) => res,
        Err(e) => {
            println!("Error making request: {}", e);
            return;
        }
    };

    let json: Value = match response.json() {
        Ok(json) => json,
        Err(e) => {
            println!("Error parsing JSON: {}", e);
            return;
        }
    };

    if let Some(content) = json["choices"][0]["message"]["content"].as_str() {
        let wrapped_content = wrap_text(content, MAX_LINE_LENGTH);
        println!("{}", OUTPUT_TEMPLATE.replace("{cow_fact}", &wrapped_content));
    } else {
        println!("Could not find content in response");
    }
}
