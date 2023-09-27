# OpenAI Rust Client

This Rust library provides a simple and easy-to-use client for interacting with OpenAI's GPT Chat API.

## Features

- Asynchronous API support with streaming capabilities
- Strongly-typed request and response objects
- Easy-to-use builder methods for request customization
- Support for chat completions, audio transcriptions/tranlations, and image creation
- Fully compatible with GPT-3.5-turbo, GPT-4 models, and other API endpoints
- Handling multipart/form-data for file handling in transcription and translation

## Installation

Add the following line to your `Cargo.toml`:

```toml
[dependencies]
openai-rust = "0.1.0"
```

Run `cargo build` to install the dependency.

## Environment Setup

Before running any code, make sure to set the OpenAI API key in your environment:

```bash
export OPENAI_API_KEY=your-openai-api-key
```

## Quick Start

Here is a simple example that demonstrates how to use the library:

```rust
use openai_rust::{
    types::{Role, ChatCompletionRequestBuilder, MessageRequestBuilder},
    OpenAIClient
};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::new(api_key);

    let messages = vec![
        MessageRequestBuilder::default()
            .role(Role::User)
            .content("What's the meaning of life?")
            .build()?,
        MessageRequestBuilder::default()
            .role(Role::Assistant)
            .content("The meaning of life is to serve the greater good.")
            .build()?,
        MessageRequestBuilder::default()
            .role(Role::User)
            .content("What is the greatest good?")
            .build()?,
        MessageRequestBuilder::default()
            .role(Role::Assistant)
            .content("The greatest good is to live in a society that values liberty and justice for all.")
            .build()?,
        MessageRequestBuilder::default()
            .role(Role::User)
            .content("How is that possible?")
            .build()?,
    ];

    let request = ChatCompletionRequestBuilder::default()
        .messages(messages)
        .model("gpt-4")    
        .max_tokens(200)
        .build()?;

    let response = client.chat(request).await?;
    
    for choice in response.choices {
        println!("Response: {}", choice.message.content.ok_or("No content!")?);
    }

    Ok(())
}
```

Output:

```
> Response: Creating such a society is possible through cooperation, collective effort, working towards equality, and promoting universal values such as peace, respect, and understanding. Education and legislation play crucial roles, as well as each individual's actions and attitudes toward others. The aim is to create a community where every person feels valued and free to express themselves without fear of judgement or harm. This involves a continuing process of dialogue, growth, and social progress.
```

## Contributing

If you'd like to contribute to the project, feel free to open an issue or submit a pull request.