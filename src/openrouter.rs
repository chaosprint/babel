pub enum OpenRouter {
    GoogleGemini20Flash,
    DeepSeekR1Free,
    MetaLlama3370BInstruct,
    AnthropicClaude35Sonnet,
    OpenAIGPT4oMini,
    GoogleGeminiFlash158B,
    GoogleGeminiFlash15,
    GoogleGeminiPro20ExpFree,
    GoogleGemini20FlashLite,
    DeepSeekR1,
    MistralNemo,
    AnthropicClaude35SonnetBeta,
    GoogleGemini20FlashExpFree,
    DeepSeekR1DistillLlama70B,
    MetaLlama3170BInstruct,
    Qwen25Coder32BInstruct,
    MetaLlama31405BInstruct,
    MistralMistral7BInstruct,
    WizardLM28x22B,
    MistralSmall3,
    MetaLlama318BInstruct,
    MythoMax13B,
    Qwen257BInstruct,
    NousHermes3405BInstruct,
    OpenAIGPT4oMini20240718,
    OpenAIGPT4o,
    GoogleGeminiFlashLite20PreviewFree,
    GoogleGemma327B,
    CohereCommandR082024,
    MetaLlama321BInstruct,
    QwenQwen25VL72BInstruct,
    QwenQwQ32BFree,
    MiniMaxMiniMax01,
    Qwen2572BInstruct,
    GoogleGeminiPro15,
    MistralTiny,
    GoogleGemma327BFree,
    QwenQwQ32B,
    OpenAIGPT4o20241120,
    MetaLlama323BInstruct,
    NeverSleepLlama3Lumimaid8BExtended,
    LiquidLFM7B,
    AnthropicClaude3Haiku,
    AnthropicClaude37Sonnet,
    AnthropicClaude37SonnetThinking,
    AnthropicClaude37SonnetBeta,
    DeepSeekChat,
    DeepSeekChatFree,
}

// Implement Display trait for OpenRouter enum
// This allows converting OpenRouter variants to their string representation
// using standard Rust formatting mechanisms
impl std::fmt::Display for OpenRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::GoogleGemini20Flash => "google/gemini-2.0-flash-001",
            Self::DeepSeekR1Free => "deepseek/deepseek-r1:free",
            Self::MetaLlama3370BInstruct => "meta-llama/llama-3.3-70b-instruct",
            Self::AnthropicClaude35Sonnet => "anthropic/claude-3.5-sonnet",
            Self::OpenAIGPT4oMini => "openai/gpt-4o-mini",
            Self::GoogleGeminiFlash158B => "google/gemini-flash-1.5-8b",
            Self::GoogleGeminiFlash15 => "google/gemini-flash-1.5",
            Self::GoogleGeminiPro20ExpFree => "google/gemini-2.0-pro-exp-02-05:free",
            Self::GoogleGemini20FlashLite => "google/gemini-2.0-flash-lite-001",
            Self::DeepSeekR1 => "deepseek/deepseek-r1",
            Self::MistralNemo => "mistralai/mistral-nemo",
            Self::AnthropicClaude35SonnetBeta => "anthropic/claude-3.5-sonnet:beta",
            Self::GoogleGemini20FlashExpFree => "google/gemini-2.0-flash-exp:free",
            Self::DeepSeekR1DistillLlama70B => "deepseek/deepseek-r1-distill-llama-70b",
            Self::MetaLlama3170BInstruct => "meta-llama/llama-3.1-70b-instruct",
            Self::Qwen25Coder32BInstruct => "qwen/qwen-2.5-coder-32b-instruct",
            Self::MetaLlama31405BInstruct => "meta-llama/llama-3.1-405b-instruct",
            Self::MistralMistral7BInstruct => "mistralai/mistral-7b-instruct",
            Self::WizardLM28x22B => "microsoft/wizardlm-2-8x22b",
            Self::MistralSmall3 => "mistralai/mistral-small-24b-instruct-2501",
            Self::MetaLlama318BInstruct => "meta-llama/llama-3.1-8b-instruct",
            Self::MythoMax13B => "gryphe/mythomax-l2-13b",
            Self::Qwen257BInstruct => "qwen/qwen-2.5-7b-instruct",
            Self::NousHermes3405BInstruct => "nousresearch/hermes-3-llama-3.1-405b",
            Self::OpenAIGPT4oMini20240718 => "openai/gpt-4o-mini-2024-07-18",
            Self::OpenAIGPT4o => "openai/gpt-4o",
            Self::GoogleGeminiFlashLite20PreviewFree => {
                "google/gemini-2.0-flash-lite-preview-02-05:free"
            }
            Self::GoogleGemma327B => "google/gemma-3-27b-it",
            Self::CohereCommandR082024 => "cohere/command-r-08-2024",
            Self::MetaLlama321BInstruct => "meta-llama/llama-3.2-1b-instruct",
            Self::QwenQwen25VL72BInstruct => "qwen/qwen2.5-vl-72b-instruct",
            Self::QwenQwQ32BFree => "qwen/qwq-32b:free",
            Self::MiniMaxMiniMax01 => "minimax/minimax-01",
            Self::Qwen2572BInstruct => "qwen/qwen-2.5-72b-instruct",
            Self::GoogleGeminiPro15 => "google/gemini-pro-1.5",
            Self::MistralTiny => "mistralai/mistral-tiny",
            Self::GoogleGemma327BFree => "google/gemma-3-27b-it:free",
            Self::QwenQwQ32B => "qwen/qwq-32b",
            Self::OpenAIGPT4o20241120 => "openai/gpt-4o-2024-11-20",
            Self::MetaLlama323BInstruct => "meta-llama/llama-3.2-3b-instruct",
            Self::NeverSleepLlama3Lumimaid8BExtended => {
                "neversleep/llama-3-lumimaid-8b:extended"
            }
            Self::LiquidLFM7B => "liquid/lfm-7b",
            Self::AnthropicClaude3Haiku => "anthropic/claude-3-haiku",
            Self::AnthropicClaude37Sonnet => "anthropic/claude-3.7-sonnet",
            Self::AnthropicClaude37SonnetThinking => {
                "anthropic/claude-3.7-sonnet:thinking"
            }
            Self::AnthropicClaude37SonnetBeta => "anthropic/claude-3.7-sonnet:beta",
            Self::DeepSeekChat => "deepseek/deepseek-chat",
            Self::DeepSeekChatFree => "deepseek/deepseek-chat:free",
        };
        write!(f, "{}", s)
    }
}