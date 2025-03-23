pub enum Groq {
    DistilWhisperLargeV3En,
    Gemma2_9bIt,
    Llama33_70bVersatile,
    Llama31_8bInstant,
    LlamaGuard3_8b,
    Llama3_70b8192,
    Llama3_8b8192,
    Mixtral8x7b32768,
    WhisperLargeV3,
    WhisperLargeV3Turbo,
    // Preview models
    QwenQwq32bPreview,
    MistralSaba24bPreview,
    Qwen25Coder32bPreview,
    Qwen2532bPreview,
    DeepseekR1DistillQwen32bPreview,
    DeepseekR1DistillLlama70bSpecdecPreview,
    DeepseekR1DistillLlama70bPreview,
    Llama3370bSpecdecPreview,
    Llama321bPreview,
    Llama323bPreview,
    Llama3211bVisionPreview,
    Llama3290bVisionPreview,
}


impl std::fmt::Display for Groq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::DistilWhisperLargeV3En => "distil-whisper-large-v3-en",
            Self::Gemma2_9bIt => "gemma2-9b-it",
            Self::Llama33_70bVersatile => "llama-3.3-70b-versatile",
            Self::Llama31_8bInstant => "llama-3.1-8b-instant",
            Self::LlamaGuard3_8b => "llama-guard-3-8b",
            Self::Llama3_70b8192 => "llama3-70b-8192",
            Self::Llama3_8b8192 => "llama3-8b-8192",
            Self::Mixtral8x7b32768 => "mixtral-8x7b-32768",
            Self::WhisperLargeV3 => "whisper-large-v3",
            Self::WhisperLargeV3Turbo => "whisper-large-v3-turbo",
            // Preview models
            Self::QwenQwq32bPreview => "qwen-qwq-32b",
            Self::MistralSaba24bPreview => "mistral-saba-24b",
            Self::Qwen25Coder32bPreview => "qwen-2.5-coder-32b",
            Self::Qwen2532bPreview => "qwen-2.5-32b",
            Self::DeepseekR1DistillQwen32bPreview => "deepseek-r1-distill-qwen-32b",
            Self::DeepseekR1DistillLlama70bSpecdecPreview => {
                "deepseek-r1-distill-llama-70b-specdec"
            }
            Self::DeepseekR1DistillLlama70bPreview => "deepseek-r1-distill-llama-70b",
            Self::Llama3370bSpecdecPreview => "llama-3.3-70b-specdec",
            Self::Llama321bPreview => "llama-3.2-1b-preview",
            Self::Llama323bPreview => "llama-3.2-3b-preview",
            Self::Llama3211bVisionPreview => "llama-3.2-11b-vision-preview",
            Self::Llama3290bVisionPreview => "llama-3.2-90b-vision-preview",
        };
        write!(f, "{}", s)
    }
}
