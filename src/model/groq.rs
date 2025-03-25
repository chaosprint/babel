use super::base::{define_provider_models, Model, Provider};

#[derive(Debug)]
pub struct Groq;

impl Provider for Groq {
    type ModelType = GroqModel;

    fn provider_name() -> &'static str {
        "groq"
    }
}

define_provider_models!(Groq, GroqModel, {
    (DistilWhisperLargeV3En, "distil-whisper-large-v3-en"),
    (Gemma2_9bIt, "gemma2-9b-it"),
    (Llama33_70bVersatile, "llama-3.3-70b-versatile"),
    (Llama31_8bInstant, "llama-3.1-8b-instant"),
    (LlamaGuard3_8b, "llama-guard-3-8b"),
    (Llama3_70b8192, "llama3-70b-8192"),
    (Llama3_8b8192, "llama3-8b-8192"),
    (Mixtral8x7b32768, "mixtral-8x7b-32768"),
    (WhisperLargeV3, "whisper-large-v3"),
    (WhisperLargeV3Turbo, "whisper-large-v3-turbo"),
    // Preview models
    (QwenQwq32bPreview, "qwen-qwq-32b"),
    (MistralSaba24bPreview, "mistral-saba-24b"),
    (Qwen25Coder32bPreview, "qwen-2.5-coder-32b"),
    (Qwen2532bPreview, "qwen-2.5-32b"),
    (DeepseekR1DistillQwen32bPreview, "deepseek-r1-distill-qwen-32b"),
    (DeepseekR1DistillLlama70bSpecdecPreview, "deepseek-r1-distill-llama-70b-specdec"),
    (DeepseekR1DistillLlama70bPreview, "deepseek-r1-distill-llama-70b"),
    (Llama3370bSpecdecPreview, "llama-3.3-70b-specdec"),
    (Llama321bPreview, "llama-3.2-1b-preview"),
    (Llama323bPreview, "llama-3.2-3b-preview"),
    (Llama3211bVisionPreview, "llama-3.2-11b-vision-preview"),
    (Llama3290bVisionPreview, "llama-3.2-90b-vision-preview")
});
