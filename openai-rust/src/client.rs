use std::error::Error;
use reqwest::{Client, Response};
use crate::types::{ChatCompletionResponse,ChatCompletionRequest};

pub struct OpenAIClient {
    client: Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new<S: Into<String>>(api_key: S) -> OpenAIClient {
        OpenAIClient {
            client: Client::new(),
            api_key: api_key.into(),
        }
    }

    pub async fn chat(&self, request: ChatCompletionRequest) -> Result<ChatCompletionResponse, Box<dyn Error>> {
        //println!("Sending request: {:?}", request);
        let response = self.send_request(&request).await?;
        let parsed_response = Self::parse_response(response).await?;
        Ok(parsed_response)
    }
    
    async fn send_request(&self, request: &ChatCompletionRequest) -> Result<Response, Box<dyn Error>> {
        let api_url = "https://api.openai.com/v1/chat/completions";
        let response = self.client
            .post(api_url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        Ok(response)
    }

    async fn parse_response(response: Response) -> Result<ChatCompletionResponse, Box<dyn Error>> {
        let text = response.error_for_status()?.text().await?;
        //println!("Response: {}", text);
        let parsed_response: ChatCompletionResponse = serde_json::from_str(&text)?;
        Ok(parsed_response)
    }
}