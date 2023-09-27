use openai_rust::{
    types::ImageRequestBuilder,
    OpenAIClient
};
use std::error::Error;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::new(api_key);

    let request = ImageRequestBuilder::default()
        .prompt("A futuristic cyberpunk cityscape at night with towering neon-lit skyscrapers, flying cars, and a diverse crowd of humans and androids, in a highly detailed digital painting reminiscent of Blade Runner.")
        .build()?;

    let response = client.image(request).await?;
    
    for image in response.data {
        println!("Response: {}", image.url);
    }

    Ok(())
}