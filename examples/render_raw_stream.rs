use babel::{ChatMessage, LLMBuilder, OpenRouter, OpenRouterModel};
use futures::StreamExt;
use tokio;

#[tokio::main]
async fn main() -> Result<(), String> {
    
    // Create an OpenRouter LLM instance
    let openrouter_llm = LLMBuilder::<OpenRouter>::new()
        .model(OpenRouterModel::GoogleGemini20ProExpFree)
        .temperature(1.0)
        .max_tokens(512)
        .system_prompt("reply in json format and put all markdown response in 'response' key".into())
        .build()?;
    
    println!("Using OpenRouter model: {}", openrouter_llm.get_model_id());
    
    // Create chat message
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "How to visit Oslo?".to_string(),
        }
    ];
    
    // Get streaming response
    println!("OpenRouter response:");
    let mut stream = openrouter_llm.stream_chat(messages).await;
    let mut renderer = babel::utils::MarkdownStreamRenderer::new();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                if let Some(content) = response.get_content() {
                    renderer.process_chunk(&content);
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    
    Ok(())
}