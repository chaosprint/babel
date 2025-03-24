use super::base::{Model, Provider, define_provider_models};

pub struct OpenRouter;

impl Provider for OpenRouter {
    type ModelType = OpenRouterModel;
    
    fn provider_name() -> &'static str {
        "openrouter"
    }
}

define_provider_models!(OpenRouter, OpenRouterModel, {
    (Claude35Sonnet, "anthropic/claude-3-5-sonnet"),
    (GPT4Turbo, "openai/gpt-4-turbo"),
    // Additional OpenRouter models
    (GoogleGemini20Flash, "google/gemini-2.0-flash-001"),
    (DeepSeekR1Free, "deepseek/deepseek-r1:free"),
    (MetaLlama3370BInstruct, "meta-llama/llama-3.3-70b-instruct"),
    (OpenAIGPT4oMini, "openai/gpt-4o-mini"),
    (GoogleGeminiFlash158B, "google/gemini-flash-1.5-8b"),
    (GoogleGeminiFlash15, "google/gemini-flash-1.5"),
    (GoogleGeminiPro20ExpFree, "google/gemini-2.0-pro-exp-02-05:free"),
    (GoogleGemini20FlashLite, "google/gemini-2.0-flash-lite-001"),
    (DeepSeekR1, "deepseek/deepseek-r1"),
    (MistralNemo, "mistralai/mistral-nemo"),
    (AnthropicClaude35SonnetBeta, "anthropic/claude-3.5-sonnet:beta"),
    (GoogleGemini20FlashExpFree, "google/gemini-2.0-flash-exp:free"),
    (DeepSeekR1DistillLlama70B, "deepseek/deepseek-r1-distill-llama-70b"),
    (MetaLlama3170BInstruct, "meta-llama/llama-3.1-70b-instruct"),
    (Qwen25Coder32BInstruct, "qwen/qwen-2.5-coder-32b-instruct"),
    (MetaLlama31405BInstruct, "meta-llama/llama-3.1-405b-instruct"),
    (MistralMistral7BInstruct, "mistralai/mistral-7b-instruct"),
    (WizardLM28x22B, "microsoft/wizardlm-2-8x22b"),
    (MistralSmall3, "mistralai/mistral-small-24b-instruct-2501"),
    (MetaLlama318BInstruct, "meta-llama/llama-3.1-8b-instruct"),
    (MythoMax13B, "gryphe/mythomax-l2-13b"),
    (Qwen257BInstruct, "qwen/qwen-2.5-7b-instruct"),
    (NousHermes3405BInstruct, "nousresearch/hermes-3-llama-3.1-405b"),
    (OpenAIGPT4oMini20240718, "openai/gpt-4o-mini-2024-07-18"),
    (OpenAIGPT4o, "openai/gpt-4o"),
    (GoogleGeminiFlashLite20PreviewFree, "google/gemini-2.0-flash-lite-preview-02-05:free"),
    (GoogleGemma327B, "google/gemma-3-27b-it"),
    (CohereCommandR082024, "cohere/command-r-08-2024"),
    (MetaLlama321BInstruct, "meta-llama/llama-3.2-1b-instruct"),
    (QwenQwen25VL72BInstruct, "qwen/qwen2.5-vl-72b-instruct"),
    (QwenQwQ32BFree, "qwen/qwq-32b:free"),
    (MiniMaxMiniMax01, "minimax/minimax-01"),
    (Qwen2572BInstruct, "qwen/qwen-2.5-72b-instruct"),
    (GoogleGeminiPro15, "google/gemini-pro-1.5"),
    (MistralTiny, "mistralai/mistral-tiny"),
    (GoogleGemma327BFree, "google/gemma-3-27b-it:free"),
    (QwenQwQ32B, "qwen/qwq-32b"),
    (OpenAIGPT4o20241120, "openai/gpt-4o-2024-11-20"),
    (MetaLlama323BInstruct, "meta-llama/llama-3.2-3b-instruct"),
    (NeverSleepLlama3Lumimaid8BExtended, "neversleep/llama-3-lumimaid-8b:extended"),
    (LiquidLFM7B, "liquid/lfm-7b"),
    (AnthropicClaude3Haiku, "anthropic/claude-3-haiku"),
    (AnthropicClaude37Sonnet, "anthropic/claude-3.7-sonnet"),
    (AnthropicClaude37SonnetThinking, "anthropic/claude-3.7-sonnet:thinking"),
    (AnthropicClaude37SonnetBeta, "anthropic/claude-3.7-sonnet:beta"),
    (DeepSeekChat, "deepseek/deepseek-chat"),
    (DeepSeekChatFree, "deepseek/deepseek-chat:free")
});