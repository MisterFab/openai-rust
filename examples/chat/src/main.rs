use openai_rust::{
    types::{Role, ChatCompletionRequestBuilder, MessageRequestBuilder},
    OpenAIClient
};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
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