use reqwest::{header, Client};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct OpenAiResponse {
    choices: Vec<OpenAiMessage>,
    created: i64,
    id: String,
    model: String,
    object: String,
    system_fingerprint: Option<String>,
    usage: UsageToken,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct OpenAiMessage {
    finish_reason: String,
    index: i8,
    logprobs: Option<String>,
    message: Message,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Message {
    content: String,
    role: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct UsageToken {
    completion_tokens: i32,
    prompt_tokens: i32,
    total_tokens: i32,
}

pub async fn generate_answer(
    content: String,
) -> Result<String, handle_errors::Error> {
    let open_ai_key =
        env::var("OPEN_AI_KEY").expect("Do not have OPEN_AI_KEY in env");
    let client = Client::new();
    let json = json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {"role": "system", "content": content}
          ]
    });

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header(header::AUTHORIZATION, format!("Bearer {}", open_ai_key))
        .header(header::CONTENT_TYPE, "application/json")
        .json(&json)
        .send()
        .await
        .map_err(|e| handle_errors::Error::ReqwestAPIError(e))?;

    match res.json::<OpenAiResponse>().await {
        Ok(res) => Ok(res.choices.into_iter().nth(0).unwrap().message.content),
        Err(e) => Err(handle_errors::Error::ReqwestAPIError(e)),
    }
}
