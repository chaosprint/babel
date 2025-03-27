# Babel

Babel is a Rust library designed to simplify interactions with various LLM (Large Language Model) providers. It provides a unified interface for making API calls to different LLM services while handling the variations in model naming conventions across providers.

## Features

- **Provider-agnostic API**: Interact with different LLM providers using a consistent interface
- **Enum-based model selection**: Get IDE autocompletion for available models
- **Streaming support**: Process responses as they arrive for real-time applications
- **Regular request support**: Get complete responses in a single call
- **Type-safe model mapping**: Map Rust enums to provider-specific model strings

## Installation

Add Babel to your `Cargo.toml`:

```toml
[dependencies]
babel = "0.0.3"
```

## Supported Providers

Currently, Babel supports the following providers:
- Groq
- OpenRouter (API compatible)
- SambaNova

## Quick Start

Here's a simple example of using Babel with Groq:

```rust
use babel::model::{ChatMessage, Groq, GroqModel, LLMBuilder};
use tokio;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Create a Groq LLM instance
    let groq_llm = LLMBuilder::<Groq>::new()
        .model(GroqModel::QwenQwq32bPreview)
        .temperature(0.7)
        .max_tokens(2048)
        .system_prompt("You are a helpful assistant.".to_string())
        .build()?;
    
    println!("Using Groq model: {}", groq_llm.get_model_id());
    
    // Create chat message
    let messages = vec![
        ChatMessage {
            role: "user".to_string(),
            content: "What is machine learning?".to_string(),
        }
    ];
    
    // Get complete response
    let response = groq_llm.chat(messages).await?;
    println!("{}", response);
    
    Ok(())
}
```

## API Authentication

Babel supports two methods for API authentication:

1. **Direct API key**: Pass your API key directly to the builder
2. **Environment variables**: Store your API keys in environment variables following the pattern `{PROVIDER_NAME}_API_KEY` (e.g., `GROQ_API_KEY`)

## Multi-turn Conversations

Babel makes it easy to maintain conversation history:

```rust
// Start a conversation
let mut conversation = vec![
    ChatMessage {
        role: "user".to_string(),
        content: "What are the key features of Rust?".to_string(),
    }
];

// Get first response
let response = llm.chat(conversation.clone()).await?;

// Add response to conversation history
conversation.push(ChatMessage {
    role: "assistant".to_string(),
    content: response,
});

// Add next user message
conversation.push(ChatMessage {
    role: "user".to_string(),
    content: "What advantages does Rust have over C++?".to_string(),
});

// Continue the conversation
let next_response = llm.chat(conversation).await?;
```

## Streaming Responses

For applications that need to process responses as they arrive:

```rust
let mut stream = llm.stream_chat(messages).await;

while let Some(result) = stream.next().await {
    match result {
        Ok(response) => {
            if let Some(content) = response.get_content() {
                // Process each chunk of the response
                print!("{}", content);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

## Adding New Providers

Babel is designed to be extensible. To add a new provider:

1. Create a new module for your provider
2. Define a provider struct and implement the `Provider` trait
3. Define a model enum and use the `define_provider_models!` macro to implement the `Model` trait

Example:

```rust
use super::base::{Model, Provider, define_provider_models};

// Define your provider
pub struct MyProvider;

impl Provider for MyProvider {
    type ModelType = MyProviderModel;
    
    fn provider_name() -> &'static str {
        "myprovider"
    }
}

// Define models for your provider
define_provider_models!(MyProvider, MyProviderModel, {
    (ModelA, "model-a-identifier"),
    (ModelB, "model-b-identifier"),
    (ModelC, "model-c-identifier")
});
```

## License

> MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.