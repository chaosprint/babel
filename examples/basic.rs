use babel::*;
fn main() {
    let model = OpenRouter::AnthropicClaude37SonnetThinking;
    println!("{}", model.to_string());

    let groq_model = Groq::Llama3_70b8192;
    println!("{}", groq_model.to_string());
}
