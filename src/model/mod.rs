mod base;
mod groq;
mod openrouter;
mod chat;
mod sambanova;

// Re-export the main components
pub use base::{Model, Provider};
pub use groq::{Groq, GroqModel};
pub use sambanova::{SambaNova, SambaNovaModel};
pub use openrouter::{OpenRouter, OpenRouterModel};
pub use chat::{ChatMessage, LLMClient, LLMBuilder, StreamResponse, Usage};

// Example usage:
/*
use crate::model::{Groq, GroqModel, LLMBuilder, ChatMessage};

async fn example() -> Result<(), String> {
    // Create a Groq LLM
    let llm = LLMBuilder::<Groq>::new()
        .model(GroqModel::Llama3_70b8192)
        .temperature(0.7)
        .max_tokens(2048)
        .build()?;
    
    // Create chat messages
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Hello, how are you?".to_string(),
        }
    ];
    
    // Get streaming response
    let mut stream = llm.stream_chat(messages.clone()).await;
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                if let Some(content) = response.get_content() {
                    print!("{}", content);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    // Or get complete response
    let response = llm.chat(messages).await?;
    println!("Response: {}", response);
    
    Ok(())
}
*/