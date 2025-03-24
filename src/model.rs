// Define a generic Model trait
pub trait Model {
    fn model_id(&self) -> &'static str;
}

// Define a Provider trait
pub trait Provider {
    type ModelType: Model;
    fn provider_name() -> &'static str;
}

// Define Groq provider
pub struct Groq;

impl Provider for Groq {
    type ModelType = GroqModel;
    
    fn provider_name() -> &'static str {
        "groq"
    }
}

// Use macro to define model enums
macro_rules! define_provider_models {
    ($provider:ident, $enum_name:ident, {
        $(($variant:ident, $value:expr)),*
    }) => {
        // Define model enum
        pub enum $enum_name {
            $($variant),*
        }
        
        // Implement Model trait for enum
        impl Model for $enum_name {
            fn model_id(&self) -> &'static str {
                match self {
                    $(Self::$variant => $value),*
                }
            }
        }
        
        // Remove associated constant definitions
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


pub struct LLMBuilder<P: Provider> {
    model: Option<P::ModelType>,
    api_key: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

impl<P: Provider> LLMBuilder<P> {
    pub fn new() -> Self {
        Self {
            model: None,
            api_key: None,
            max_tokens: None,
            temperature: None,
        }
    }
    
    pub fn model(mut self, model: P::ModelType) -> Self {
        self.model = Some(model);
        self
    }
    
    pub fn api_key(mut self, key: String) -> Self {
        self.api_key = Some(key);
        self
    }
    
    pub fn max_tokens(mut self, tokens: u32) -> Self {
        self.max_tokens = Some(tokens);
        self
    }
    
    pub fn temperature(mut self, temp: f32) -> Self {
        self.temperature = Some(temp);
        self
    }
    
    pub fn build(self) -> Result<LLM<P>, String> {
        let model = self.model.ok_or("Model is required".to_string())?;
        let api_key = self.api_key.ok_or("API key is required".to_string())?;
        
        Ok(LLM {
            model,
            api_key,
            max_tokens: self.max_tokens.unwrap_or(1024),
            temperature: self.temperature.unwrap_or(0.7),
            _provider: std::marker::PhantomData,
        })
    }
}

pub struct LLM<P: Provider> {
    model: P::ModelType,
    api_key: String,
    max_tokens: u32,
    temperature: f32,
    _provider: std::marker::PhantomData<P>,
}

impl<P: Provider> LLM<P> {
    pub fn get_model_id(&self) -> &'static str {
        self.model.model_id()
    }
    
    pub fn get_provider_name() -> &'static str {
        P::provider_name()
    }
}