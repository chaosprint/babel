use babel::*;
fn main() {
let groq_llm = LLMBuilder::<Groq>::new()
    .model(GroqModel::Gemma2_9bIt)
    .api_key("your-api-key".to_string())
    .max_tokens(512)
    .temperature(0.7)
    .build()
    .expect("Failed to build Groq LLM");

let openrouter_llm = LLMBuilder::<OpenRouter>::new()
    .model(OpenRouterModel::GPT4Turbo)
    .api_key("your-api-key".to_string())
    .max_tokens(1024)
    .temperature(0.8)
    .build()
    .expect("Failed to build OpenRouter LLM");
}