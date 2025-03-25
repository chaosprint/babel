use serde::{Deserialize, Serialize};
use std::pin::Pin;
use futures::stream::Stream;
use reqwest::Client;
use tokio_stream::StreamExt;
use tracing::error;
use dotenv::dotenv;
use std::marker::PhantomData;
use async_stream::stream;

use super::base::Provider;
use super::base::Model;

// Chat message structure
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

// Chat request structure
#[derive(Serialize, Debug)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    temperature: f32,
    max_tokens: Option<u32>,
}

// Response structures
#[derive(Debug, Deserialize, Clone)]
pub struct Usage {
    pub prompt_tokens: Option<u32>,
    pub completion_tokens: Option<u32>,
    pub total_tokens: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Delta {
    // pub role: Option<String>,
    pub content: Option<String>,
}

impl Default for Delta {
    fn default() -> Self {
        Self {
            // role: None,
            content: None,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    // pub index: Option<u32>,
    #[serde(default)]
    delta: Delta,
    #[serde(default)]
    finish_reason: Option<String>,
    // pub native_finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct StreamResponse {
    // id: String,
    // provider: Option<String>,
    // model: String,
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

impl StreamResponse {

    pub fn get_content(&self) -> Option<String> {
        self.choices
            .first()
            .and_then(|choice| choice.delta.content.clone())
    }

    pub fn get_usage(&self) -> Option<Usage> {
        self.usage.clone()
    }

    pub fn is_finished(&self) -> bool {
        self.choices
            .first()
            .and_then(|choice| choice.finish_reason.as_ref())
            .map(|reason| reason == "stop")
            .unwrap_or(false)
    }
}

// LLMClient Builder
pub struct LLMBuilder<P: Provider> {
    model: Option<P::ModelType>,
    api_key: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
    system_prompt: Option<String>,
}

impl<P: Provider> LLMBuilder<P> {
    pub fn new() -> Self {
        Self {
            model: None,
            api_key: None,
            max_tokens: None,
            temperature: None,
            system_prompt: Some("You are a helpful AI assistant.".to_string()),
        }
    }
    
    pub fn model(mut self, model: P::ModelType) -> Self {
        self.model = Some(model);
        self
    }
    
    pub fn api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }
    
    pub fn max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }
    
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }
    
    pub fn system_prompt(mut self, prompt: String) -> Self {
        self.system_prompt = Some(prompt);
        self
    }
    
    pub fn build(self) -> Result<LLMClient<P>, String> {
        // Load environment variables
        dotenv().ok();
        
        let model = self.model.ok_or("Model is required".to_string())?;
        
        // Try to get API key from environment if not provided
        let api_key = match self.api_key {
            Some(key) => key,
            None => {
                let env_var = format!("{}_API_KEY", P::provider_name().to_uppercase());
                std::env::var(&env_var)
                    .map_err(|_| format!("{} not found in environment variables", env_var))?
            }
        };
        
        Ok(LLMClient {
            model,
            api_key,
            max_tokens: self.max_tokens.unwrap_or(1024),
            temperature: self.temperature.unwrap_or(0.7),
            system_prompt: self.system_prompt,
            client: Client::new(),
            _provider: PhantomData,
        })
    }
}

// LLMClient implementation
pub struct LLMClient<P: Provider> {
    model: P::ModelType,
    api_key: String,
    max_tokens: u32,
    temperature: f32,
    system_prompt: Option<String>,
    client: Client,
    _provider: PhantomData<P>,
}

impl<P: Provider> LLMClient<P> {
    pub fn get_model_id(&self) -> &'static str {
        self.model.model_id()
    }
    
    pub fn get_provider_name() -> &'static str {
        P::provider_name()
    }
    
    pub fn get_system_prompt(&self) -> Option<String> {
        self.system_prompt.clone()
    }
    
    // Stream chat implementation
    pub async fn stream_chat(
        &self,
        history: Vec<ChatMessage>,
    ) -> Pin<Box<dyn Stream<Item = Result<StreamResponse, String>> + Send>> {
        let mut messages = Vec::new();
        
        // Add system prompt if available
        if let Some(system_prompt) = &self.system_prompt {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: system_prompt.clone(),
            });
        }
        
        // Add chat history
        messages.extend(history);
        
        let client = self.client.clone();
        let model_id = self.model.model_id().to_string();
        let api_key = self.api_key.clone();
        let temperature = self.temperature;
        let max_tokens = Some(self.max_tokens);
        
        // Build base URL based on provider
        let base_url = match P::provider_name() {
            "groq" => "https://api.groq.com/openai/v1/chat/completions".to_string(),
            "openrouter" => "https://openrouter.ai/api/v1/chat/completions".to_string(),
            _ => return Box::pin(stream! {
                yield Err("Unsupported provider".to_string());
            }),
        };
        
        Box::pin(stream! {
            // Build request
            let request = ChatRequest {
                model: model_id,
                messages,
                stream: true,
                temperature,
                max_tokens,
            };
            
            // Send request
            let response = client
                .post(&base_url)
                .header("Authorization", format!("Bearer {}", api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await;
                
            match response {
                Ok(res) => {
                    let mut stream = res.bytes_stream();
                    let mut buffer = String::new();
                    
                    while let Some(item) = stream.next().await {
                        match item {
                            Ok(bytes) => {
                                let chunk = String::from_utf8_lossy(&bytes);
                                buffer.push_str(&chunk);
                                
                                // Process OpenAI-style SSE format
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
                            Err(e) => {
                                error!("Error reading stream: {}", e);
                                yield Err(format!("Error reading stream: {}", e));
                                break;
                            }
                        }
                    }
                    
                    // Process any remaining data in buffer
                    if !buffer.is_empty() && buffer.starts_with("data: ") && buffer != "data: [DONE]" {
                        let data = buffer.replacen("data: ", "", 1);
                        match serde_json::from_str::<StreamResponse>(&data) {
                            Ok(response) => yield Ok(response),
                            Err(e) => yield Err(format!("Failed to parse final response: {}", e)),
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
    
    // Non-streaming chat implementation
    pub async fn chat(&self, history: Vec<ChatMessage>) -> Result<String, String> {
        let mut response_text = String::new();
        let mut stream = self.stream_chat(history).await;
        
        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    if let Some(content) = response.get_content() {
                        response_text.push_str(&content);
                    }
                }
                Err(e) => return Err(e),
            }
        }
        
        Ok(response_text)
    }
}