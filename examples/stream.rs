use babel::model::{ChatMessage, LLMBuilder, OpenRouter, OpenRouterModel};
use futures::StreamExt;
use tokio;

#[tokio::main]
async fn main() -> Result<(), String> {
    
    // Create an OpenRouter LLM instance
    let openrouter_llm = LLMBuilder::<OpenRouter>::new()
        .model(OpenRouterModel::GoogleGeminiPro20ExpFree)
        .temperature(0.5)
        .max_tokens(512)
        .build()?;
    
    println!("Using OpenRouter model: {}", openrouter_llm.get_model_id());
    
    // Create chat message
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Briefly explain AI history.".to_string(),
        }
    ];
    
    // Get streaming response
    println!("OpenRouter response:");
    let mut stream = openrouter_llm.stream_chat(messages).await;
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                if let Some(content) = response.get_content() {
                    print!("{}", content);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}