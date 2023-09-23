use crate::types::{ChatCompletionResponse,ChatCompletionRequest, StreamResponse};

use hyper::Body;
use hyper_tls::HttpsConnector;
use reqwest::Response;
use serde_json::from_str;
use std::error::Error;
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

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
        Self::parse_response(response).await
    }

    pub async fn chat_stream(&self, request: ChatCompletionRequest) -> Result<mpsc::Receiver<StreamResponse>, Box<dyn Error>> {
        let hyper_request = self.build_hyper_request(request)?;
        let res = self.send_hyper_request(hyper_request).await?;
        
        if !res.status().is_success() {
            return Err("Request failed".into());
        }

        let (tx, rx) = mpsc::channel::<StreamResponse>(1000);
        tokio::spawn(async move {
            let mut body = res.into_body();
            while let Some(chunk_result) = body.next().await {
                match chunk_result {
                    Ok(chunk) => match String::from_utf8(chunk.to_vec()) {
                        Ok(chunk_str) => Self::process_chunk(chunk_str, &tx).await,
                        Err(_) => eprintln!("Error converting chunk to string"),
                    },
                    Err(_) => eprintln!("Error receiving chunk"),
                }
            }
        });

        Ok(rx)
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

    fn build_hyper_request(&self, request: ChatCompletionRequest) -> Result<hyper::Request<Body>, Box<dyn Error>> {
        let body = Body::from(serde_json::to_string(&request)?);
        hyper::Request::builder()
            .method(hyper::Method::POST)
            .uri(API_URL)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .body(body)
            .map_err(Into::into)
    }

    async fn send_hyper_request(&self, request: hyper::Request<Body>) -> Result<hyper::Response<Body>, Box<dyn Error>> {
        let https = HttpsConnector::new();
        let hyper_client = hyper::Client::builder().build::<_, Body>(https);
        hyper_client.request(request).await.map_err(Into::into)
    }

    async fn parse_response(response: Response) -> Result<ChatCompletionResponse, Box<dyn Error>> {
        let text = response.error_for_status()?.text().await?;
        from_str(&text).map_err(Into::into)
    }

    async fn process_chunk(chunk_str: String, tx: &mpsc::Sender<StreamResponse>) {
        for line in chunk_str.split('\n').filter(|s| !s.is_empty() && s.contains('{')) {
            if let Some(start) = line.find('{') {
                let json_str = &line[start..];
                match serde_json::from_str::<StreamResponse>(json_str.trim()) {
                    Ok(parsed_obj) => {
                        if let Err(_) = tx.send(parsed_obj).await {
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