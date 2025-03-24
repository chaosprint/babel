use dotenv::dotenv;
use futures::stream::Stream;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::StreamExt;
use tracing::{error, warn};

// use crate::strip_markdown_code_blocks;

/// LLM Provider enum
#[derive(Debug, Clone, PartialEq)]
pub enum Provider {
    OpenRouter,
    SambaNova,
    Groq,
    Ollama,
}

impl Provider {
    fn base_url(&self) -> String {
        match self {
            Provider::OpenRouter => "https://openrouter.ai/api/v1/chat/completions".to_string(),
            Provider::SambaNova => "https://api.sambanova.ai/v1/chat/completions".to_string(),
            Provider::Groq => "https://api.groq.com/openai/v1/chat/completions".to_string(),
            Provider::Ollama => "http://localhost:11434/api/chat".to_string(),
        }
    }

    fn default_model(&self) -> String {
        match self {
            Provider::OpenRouter => "google/gemini-2.0-flash-lite-001".to_string(),
            Provider::SambaNova => "QwQ-32B".to_string(),
            Provider::Groq => "qwen-qwq-32b".to_string(),
            Provider::Ollama => "llama3".to_string(),
        }
    }

    fn default_api_key(&self) -> String {
        // Load .env file if it exists
        dotenv().ok();

        match self {
            Provider::OpenRouter => std::env::var("OPENROUTER_API_KEY").unwrap_or_default(),
            Provider::SambaNova => std::env::var("SAMBANOVA_API_KEY").unwrap_or_default(),
            Provider::Groq => std::env::var("GROQ_API_KEY").unwrap_or_default(),
            Provider::Ollama => String::new(), // Ollama typically doesn't require an API key
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    content: Option<String>,
}

impl Default for Delta {
    fn default() -> Self {
        Self {
            role: None,
            content: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: Option<u32>,
    #[serde(default)]
    delta: Delta,
    #[serde(default)]
    finish_reason: Option<String>,
    pub native_finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StreamResponse {
    OpenAIStyle {
        id: String,
        provider: Option<String>,
        model: String,
        choices: Vec<Choice>,
        usage: Option<Usage>,
    },
    OllamaStyle {
        model: String,
        created_at: String,
        message: Message,
        #[serde(default)]
        done: bool,
        done_reason: Option<String>,
        total_duration: Option<u64>,
        load_duration: Option<u64>,
        prompt_eval_count: Option<u64>,
        prompt_eval_duration: Option<u64>,
        eval_count: Option<u64>,
        eval_duration: Option<u64>,
    },
}

impl StreamResponse {
    pub fn get_content(&self) -> Option<String> {
        match self {
            StreamResponse::OpenAIStyle { choices, .. } => choices
                .first()
                .and_then(|choice| choice.delta.content.clone()),

            StreamResponse::OllamaStyle { message, .. } => {
                let content_str = message.content.clone();
                if let Some(parsed_json) = serde_json::Value::String(content_str.clone()).as_str() {
                    // panic!("{}", parsed_json);
                    Some(parsed_json.to_string())
                } else {
                    warn!("Failed to parse JSON string: {}", content_str);
                    Some(content_str)
                }
            } // StreamResponse::OllamaStyle { message, .. } => {
              //     Some(message.content.clone())
              // }
        }
    }

    pub fn get_usage(&self) -> Option<Usage> {
        match self {
            StreamResponse::OpenAIStyle { usage, .. } => usage.clone(),
            StreamResponse::OllamaStyle {
                prompt_eval_count,
                eval_count,
                total_duration,
                ..
            } => {
                // Convert Ollama stats to approximate Usage
                if prompt_eval_count.is_some() || eval_count.is_some() {
                    Some(Usage {
                        prompt_tokens: prompt_eval_count.map(|v| v as u32),
                        completion_tokens: eval_count.map(|v| v as u32),
                        total_tokens: total_duration.map(|_| {
                            (prompt_eval_count.unwrap_or(0) + eval_count.unwrap_or(0)) as u32
                        }),
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            StreamResponse::OpenAIStyle { choices, .. } => choices
                .first()
                .and_then(|choice| choice.finish_reason.as_ref())
                .map(|reason| reason == "stop")
                .unwrap_or(false),
            StreamResponse::OllamaStyle { done, .. } => *done,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LLMClient {
    provider: Provider,
    api_key: String,
    model: String,
    system_prompt: Option<String>,
    client: Client,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
struct OpenAIChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    temperature: f32,
}

#[derive(Serialize, Debug)]
struct OllamaChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
}

impl LLMClient {
    pub fn new(provider: Provider) -> Self {
        dotenv::dotenv().ok();

        Self {
            api_key: provider.default_api_key(),
            model: provider.default_model(),
            provider,
            system_prompt: Some("You are a helpful AI assistant.".to_string()),
            client: Client::new(),
        }
    }

    /// Get the system prompt if configured
    pub fn get_system_prompt(&self) -> Option<String> {
        self.system_prompt.clone()
    }

    pub fn with_model(mut self, model: &str) -> Self {
        self.model = model.to_string();
        self
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = api_key.to_string();
        self
    }

    pub fn with_system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = Some(prompt.to_string());
        self
    }

    pub async fn stream_chat(
        &self,
        history: Vec<ChatMessage>,
    ) -> Pin<Box<dyn Stream<Item = Result<StreamResponse, String>> + Send>> {
        let messages = history;
        let client = self.client.clone();
        let provider = self.provider.clone();
        let model = self.model.clone();
        let api_key = self.api_key.clone();
        let base_url = provider.base_url();

        Box::pin(async_stream::stream! {
            let response = match provider {
                Provider::Ollama => {
                    // Ollama request format
                    let request = OllamaChatRequest {
                        model,
                        messages,
                        stream: true,
                    };

                    client
                        .post(&base_url)
                        .json(&request)
                        .send()
                        .await
                },
                _ => {
                    // OpenAI-compatible providers (OpenRouter, SambaNova, Groq)
                    let request = OpenAIChatRequest {
                        model,
                        messages,
                        stream: true,
                        temperature: 0.0,
                    };

                    client
                        .post(&base_url)
                        .header("Authorization", format!("Bearer {}", api_key))
                        .header("Content-Type", "application/json")
                        .json(&request)
                        .send()
                        .await
                }
            };

            match response {
                Ok(res) => {
                    let mut stream = res.bytes_stream();
                    let mut buffer = String::new();

                    while let Some(item) = stream.next().await {
                        match item {
                            Ok(bytes) => {
                                let chunk = String::from_utf8_lossy(&bytes);
                                buffer.push_str(&chunk);

                                if provider == Provider::Ollama {
                                    // Ollama sends complete JSON objects
                                    if !buffer.is_empty() {
                                        // Strip markdown code blocks if present
                                        // let clean_json = strip_markdown_code_blocks(&buffer);

                                        match serde_json::from_str::<StreamResponse>(&buffer) {
                                            Ok(response) => {
                                                buffer.clear();
                                                yield Ok(response);
                                            },
                                            Err(e) => {
                                                if buffer.ends_with("}\n") || buffer.ends_with("}") {
                                                    // It should be a complete JSON, so report the error
                                                    yield Err(format!("Failed to parse Ollama response: {}", e));
                                                    buffer.clear();
                                                }
                                                // Otherwise, wait for more data
                                            }
                                        }
                                    }
                                } else {
                                    // OpenAI-style chunked SSE format
                                    while let Some(pos) = buffer.find('\n') {
                                        let line = buffer[..pos].trim().to_string();
                                        buffer = buffer[pos + 1..].to_string();

                                        if line.starts_with("data: ") && line != "data: [DONE]" {
                                            let data = line.replacen("data: ", "", 1);
                                            match serde_json::from_str::<StreamResponse>(&data) {
                                                Ok(response) => yield Ok(response),
                                                Err(e) => yield Err(format!("Failed to parse response: {}", e)),
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Error reading stream: {}", e);
                                yield Err(format!("Error reading stream: {}", e));
                                break;
                            }
                        }
                    }

                    // Process any remaining data in buffer
                    if !buffer.is_empty() {
                        if provider == Provider::Ollama {
                            // let clean_json = strip_markdown_code_blocks(&buffer);
                            match serde_json::from_str::<StreamResponse>(&buffer) {
                                Ok(response) => yield Ok(response),
                                Err(e) => yield Err(format!("Failed to parse final Ollama response: {}", e)),
                            }
                        } else if buffer.starts_with("data: ") && buffer != "data: [DONE]" {
                            let data = buffer.replacen("data: ", "", 1);
                            match serde_json::from_str::<StreamResponse>(&data) {
                                Ok(response) => yield Ok(response),
                                Err(e) => yield Err(format!("Failed to parse final response: {}", e)),
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Request error: {}", e);
                    yield Err(format!("Request error: {}", e));
                }
            }
        })
    }
}
