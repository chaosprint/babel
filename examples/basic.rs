use babel::*;
fn main() {
    let model = OpenRouterModel::GPT4Turbo;
    println!("{}", model.model_id());

    let groq_model = GroqModel::Llama3_70b8192;
    println!("{}", groq_model.model_id());
}
