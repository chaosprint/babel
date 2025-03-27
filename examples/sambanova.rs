use babel::{ChatMessage, SambaNova, SambaNovaModel, LLMBuilder};
use tokio;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Create a Groq LLM instance
    let llm = LLMBuilder::<SambaNova>::new()
        .model(SambaNovaModel::DeepSeekV3_0324)
        .temperature(0.7)
        .max_tokens(2048)
        .system_prompt("Be as concise as possible.".to_string())
        .build()?;
    
    println!("Using SambaNova model: {}", llm.get_model_id());
    
    // Create chat message
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "Give me a html snake game with js and css.".to_string(),
        }
    ];
    
    // Get complete response
    println!("LLM response:");
    let response = llm.chat(messages).await?;
    println!("{}\n", response);
    
    Ok(())
}