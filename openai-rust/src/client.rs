use std::error::Error;
use reqwest::{Client, Response};
use crate::types::{Role, ChatCompletionResponse,ChatCompletionRequest,Message};

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
        let parsed_response: ChatCompletionResponse = serde_json::from_str(&text)?;
        Ok(parsed_response)
    }
}

impl Default for ChatCompletionRequest {
    fn default() -> Self {
        Self {
            messages: vec![],
            model: Some("gpt-3.5-turbo".to_string()),
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
        }
    }
}


impl ChatCompletionRequest {
    pub fn new(messages: Vec<Message>) -> Self {
        Self {
            messages,
            ..Default::default()
        }
    }

    pub fn model<S: Into<String>>(mut self, model: S) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn max_tokens(mut self, max_tokens: i32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    pub fn temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f32) -> Self {
        self.top_p = Some(top_p);
        self
    }

    pub fn n(mut self, n: i32) -> Self {
        self.n = Some(n);
        self
    }
}

impl Message {
    pub fn new<S: Into<String>>(role: Role, content: S) -> Self {
        Message {
            role,
            content: content.into(),
        }
    }
}