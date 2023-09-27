use crate::types::{ChatCompletionRequest, ChatCompletionResponse, StreamResponse, TranscriptionRequest, TranscriptionResponse, TranslationRequest, TranslationResponse};
use reqwest::{Client, RequestBuilder, Body, multipart::{Form, Part}};
use std::error::Error;
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};
use tokio_util::codec::{BytesCodec, FramedRead};
use futures::stream::StreamExt;
use bytes::Bytes;
use std::path::Path;
use serde::{Serialize, Deserialize};


const CHAT_API_URL: &str = "https://api.openai.com/v1/chat/completions";
const TRANSCRIPTIONS_API_URL: &str = "https://api.openai.com/v1/audio/transcriptions";
const TRANSLATIONS_API_URL: &str = "https://api.openai.com/v1/audio/translations";

pub struct OpenAIClient {
    client: reqwest::Client,
    api_key: String,
}

impl OpenAIClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
        }
    }

    pub async fn chat(&self, request: ChatCompletionRequest) -> Result<ChatCompletionResponse, Box<dyn Error + Send + Sync>> {
        let response = self.build_request(CHAT_API_URL, &request)?.send().await?;
        let text = response.text().await?;
        Ok(serde_json::from_str(&text)?)
    }

    pub async fn chat_stream(&self, mut request: ChatCompletionRequest) -> Result<UnboundedReceiver<StreamResponse>, Box<dyn Error + Send + Sync>> {
        request.stream = Some(true);
        let request = self.build_request(CHAT_API_URL, &request)?;
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

    pub async fn transcription(&self, request: TranscriptionRequest) -> Result<TranscriptionResponse, Box<dyn Error + Send + Sync>> {
        let file_part = self.create_file_part(&request.file).await?;
        let mut form = Form::new().part("file", file_part).text("model", request.model);
        
        if let Some(prompt) = request.prompt { form = form.text("prompt", prompt); }
        if let Some(response_format) = request.response_format { form = form.text("response_format", response_format); }
        if let Some(temperature) = request.temperature { form = form.text("temperature", temperature.to_string()); }
        if let Some(language) = request.language { form = form.text("language", language); }

        self.send_multipart_request(TRANSCRIPTIONS_API_URL, form).await
    }

    pub async fn translation(&self, request: TranslationRequest) -> Result<TranslationResponse, Box<dyn Error + Send + Sync>> {
        let file_part = self.create_file_part(&request.file).await?;
        let mut form = Form::new().part("file", file_part).text("model", request.model);
        
        if let Some(prompt) = request.prompt { form = form.text("prompt", prompt); }
        if let Some(response_format) = request.response_format { form = form.text("response_format", response_format); }
        if let Some(temperature) = request.temperature { form = form.text("temperature", temperature.to_string()); }

        self.send_multipart_request(TRANSLATIONS_API_URL, form).await
    }

    fn build_request<T: Serialize>(&self, url: &str, request: &T) -> Result<RequestBuilder, Box<dyn Error + Send + Sync>> {
        Ok(self.client.post(url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(request))
    }

    async fn send_multipart_request<R: for<'de> Deserialize<'de>>(&self, url: &str, form: Form) -> Result<R, Box<dyn Error + Send + Sync>> {
        let response = self.client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;
        let text = response.text().await?;
        Ok(serde_json::from_str(&text)?)
    }

    async fn process_chunk(chunk: Bytes, tx: &UnboundedSender<StreamResponse>) {
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

    async fn create_file_part<P: AsRef<Path>>(&self, path: P) -> Result<Part, Box<dyn Error + Send + Sync>> {
        let file_name = path.as_ref()
            .file_name()
            .ok_or("Invalid file name")?
            .to_str()
            .ok_or("Non UTF-8 file name")?
            .to_string();

        Ok(Part::stream(self.file_stream_body(path).await?)
            .file_name(file_name)
            .mime_str("application/octet-stream")?)
    }

    async fn file_stream_body<P: AsRef<Path>>(&self, path: P) -> Result<Body, Box<dyn Error + Send + Sync>> {
        let file = tokio::fs::File::open(path).await?;
        let stream = FramedRead::new(file, BytesCodec::new());
        Ok(Body::wrap_stream(stream))
    }
}