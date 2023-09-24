use crate::types::{ChatCompletionResponse,ChatCompletionRequest, StreamResponse};

use reqwest::Response;
use serde_json::from_str;
use std::error::Error;
use tokio::sync::mpsc;
use futures_util::StreamExt;
use bytes::Bytes;

const API_URL: &str = "https://api.openai.com/v1/chat/completions";

pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    pub async fn chat(&self, request: ChatCompletionRequest) -> Result<ChatCompletionResponse, Box<dyn Error>> {
        let response = self.send_request(&request).await?;
        self.parse_response(response).await
    }

    async fn send_request(&self, request: &ChatCompletionRequest) -> Result<Response, Box<dyn Error>> {
        self.client
            .post(API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request)
            .send()
            .await
            .map_err(Into::into)
    }

    async fn parse_response(&self, response: Response) -> Result<ChatCompletionResponse, Box<dyn Error>> {
        let text = response.error_for_status()?.text().await?;
        from_str(&text).map_err(Into::into)
    }

    pub async fn chat_stream(&self, mut request: ChatCompletionRequest) -> Result<mpsc::UnboundedReceiver<StreamResponse>, Box<dyn Error>>{
        request.stream = Some(true);
        let request = self.build_stream(request).await;
        let response = request.send().await?;
    
        let mut stream = response.bytes_stream();
        
        let (tx, rx) = mpsc::unbounded_channel();
        
        tokio::spawn(async move {
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        Self::process_chunk(bytes, &tx).await;
                    },
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }
        });
    
        Ok(rx)
    }

    async fn build_stream(&self, request: ChatCompletionRequest) -> reqwest::RequestBuilder {
        self.client
            .post(API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
    }

    async fn process_chunk(chunk: Bytes, tx: &mpsc::UnboundedSender<StreamResponse>) {
        for line in String::from_utf8_lossy(&chunk).split('\n').filter(|s| !s.is_empty() && s.contains('{')) {
            if let Some(start) = line.find('{') {
                let json_str = &line[start..];
                match serde_json::from_str::<StreamResponse>(json_str.trim()) {
                    Ok(parsed_obj) => {
                        if let Err(_) = tx.send(parsed_obj) {
                            eprintln!("Error sending parsed object through channel");
                        }
                    }
                    Err(e) => {
                        eprintln!("JSON deserialize error: {:?}", e);
                    }
                }
            } else {
                eprintln!("No valid JSON found in chunk");
            }
        }
    }
}
