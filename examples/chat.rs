use babel::model::{ChatMessage, Groq, GroqModel, LLMBuilder};
use tokio;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Create a Groq LLM instance
    let groq_llm = LLMBuilder::<Groq>::new()
        .model(GroqModel::QwenQwq32bPreview)
        .temperature(0.7)
        .max_tokens(2048)
        .system_prompt("You are a helpful assistant.".to_string())
        .build()?;
    
    println!("Using Groq model: {}", groq_llm.get_model_id());
    
    // Create chat message
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "What is machine learning?".to_string(),
        }
    ];
    
    // Get complete response
    println!("Groq response:");
    let response = groq_llm.chat(messages).await?;
    println!("{}\n", response);
    
    
    // Multi-turn conversation example
    println!("\nMulti-turn conversation example:");
    let mut conversation = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "What are the key features of Rust?".to_string(),
        }
    ];
    
    // First turn
    let response = groq_llm.chat(conversation.clone()).await?;
    println!("AI: {}\n", response);
    
    // Add AI's reply to conversation history
    conversation.push(ChatMessage {
        role: "assistant".to_string(),
        content: response,
    });
    
    // Add user's next question
    conversation.push(ChatMessage {
        role: "user".to_string(),
        content: "What advantages does Rust have over C++?".to_string(),
    });
    
    // Second turn
    let response = groq_llm.chat(conversation.clone()).await?;
    println!("AI: {}\n", response);
    
    // Add AI's second reply to conversation history
    conversation.push(ChatMessage {
        role: "assistant".to_string(),
        content: response,
    });
    
    // Add user's third question
    conversation.push(ChatMessage {
        role: "user".to_string(),
        content: "Give me a simple example of Rust's ownership system.".to_string(),
    });
    
    // Third turn
    let response = groq_llm.chat(conversation).await?;
    println!("AI: {}", response);
    
    Ok(())
}