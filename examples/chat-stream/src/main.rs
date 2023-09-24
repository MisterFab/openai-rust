use openai_rust::{
    types::{Role, ChatCompletionRequestBuilder, MessageRequestBuilder},
    OpenAIClient
};
use std::error::Error;
use std::env;
use std::io::{stdout, Write};

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
            .content("How is that possible? Give me a detailed answer.")
            .build()?,
    ];

    let request = ChatCompletionRequestBuilder::default()
        .messages(messages)
        .model("gpt-4")
        .build()?;

    let mut rx = client.chat_stream(request).await?;
    
    let mut lock = stdout().lock();
    while let Some(response) = rx.recv().await {
        for choice in response.choices {
            if let Some(delta) = &choice.delta {
                if let Some(content) = &delta.content {
                    write!(lock, "{}", content).unwrap();
                }
            }
        }
        stdout().flush()?;
    }
    
    Ok(())
}