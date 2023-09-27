use openai_rust::{
    types::{TranslationRequestBuilder},
    OpenAIClient
};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::new(api_key);

    let request = TranslationRequestBuilder::default()
        .file("./Die Theorie der Unordnung.mp3")
        .model("whisper-1")
        .build()?;

    let response = client.translation(request).await?;
    
    println!("{}", response.text);

    Ok(())
}