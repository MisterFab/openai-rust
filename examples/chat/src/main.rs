use openai_rust::{types::{Role, ChatCompletionRequest, Message}, OpenAIClient};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::new(api_key);

    let messages = vec![
        Message::new(Role::User, "What's the meaning of life?"),
        Message::new(Role::Assistant, "The meaning of life is to serve the greater good."),
        Message::new(Role::User, "What is the greatest good?"),
        Message::new(Role::Assistant, "The greatest good is to live in a society that values liberty and justice for all."),
        Message::new(Role::User, "How is that possible?"),
    ];

    let request = ChatCompletionRequest::new(messages)
        .model("gpt-4")    
        .max_tokens(200);

    let response = client.chat(request).await?;
    
    for choice in response.choices {
        println!("Response: {}", choice.message.content);
    }

    Ok(())
}