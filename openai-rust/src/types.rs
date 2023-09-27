use serde::{Deserialize, Serialize};
use derive_builder::Builder;
use std::collections::HashMap;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    #[default]
    User,
    System,
    Assistant,
    Function,
}

#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(setter(into, strip_option), default, build_fn(validate = "Self::validate"))]
pub struct ChatCompletionRequest {
    #[builder(default = "String::from(\"gpt-3.5-turbo\")")]
    pub model: String,
    pub messages: Vec<MessageRequest>,    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<String>,
    pub temperature: Option<f64>,
    pub top_p: Option<f64>,
    pub n: Option<i32>,
    pub stream: Option<bool>,
    pub stop: Option<Vec<String>>,
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(setter(into))]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Parameters,
}

#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct MessageRequest {
    pub role: Role,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, Builder)]
#[builder(setter(into))]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<ChoiceWrapper>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ChoiceWrapper {
    pub index: i32,
    pub message: MessageResponse,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct MessageResponse {
    pub role: Role,
    pub content: Option<String>,
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(setter(into))]
pub struct Parameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(setter(into))]
pub struct Property {
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct StreamResponse {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub delta: Option<Delta>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
}
#[derive(Debug, Clone, Default, Builder)]
#[builder(setter(into, strip_option), default, build_fn(validate = "Self::validate"))]
pub struct TranscriptionRequest {
    pub file: String,
    #[builder(default = "String::from(\"whisper-1\")")]
    pub model: String,
    pub prompt: Option<String>,
    pub response_format: Option<String>,
    pub temperature: Option<f64>,
    pub language: Option<String>
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TranscriptionResponse {
    pub text: String,
}

#[derive(Debug, Clone, Default, Builder)]
#[builder(setter(into, strip_option), default, build_fn(validate = "Self::validate"))]
pub struct TranslationRequest {
    pub file: String,
    #[builder(default = "String::from(\"whisper-1\")")]
    pub model: String,
    pub prompt: Option<String>,
    pub response_format: Option<String>,
    pub temperature: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct TranslationResponse {
    pub text: String,
}

#[derive(Debug, Clone, Default, Serialize, Builder)]
#[builder(setter(into, strip_option), default, build_fn(validate = "Self::validate"))]
pub struct ImageRequest {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct ImageResponse {
    pub created: u32,
    pub data: Vec<Image>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Image {
    pub url: String,
}

impl ChatCompletionRequestBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(temp) = self.temperature {
            let temperature_value = temp.unwrap();
            if temperature_value < 0.0 || temperature_value > 2.0 {
                return Err(format!("Invalid temperature: {}. It should be between 0.0 and 2.0.", temperature_value));
            }
        }

        if let Some(presence_penalty) = self.presence_penalty {
            let presence_penalty_value = presence_penalty.unwrap();
            if presence_penalty_value < -2.0 || presence_penalty_value > 2.0 {
                return Err(format!("Invalid presence_penalty: {}. It should be between -2.0 and 2.0.", presence_penalty_value));
            }
        }

        if let Some(frequency_penalty) = self.frequency_penalty {
            let frequency_penalty_value = frequency_penalty.unwrap();
            if frequency_penalty_value < 0.0 || frequency_penalty_value > 2.0 {
                return Err(format!("Invalid frequency_penalty: {}. It should be between -2.0 and 2.0.", frequency_penalty_value));
            }
        }
            
        Ok(())
    }
}

impl TranscriptionRequestBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(temp) = self.temperature {
            let temperature_value = temp.unwrap();
            if temperature_value < 0.0 || temperature_value > 1.0 {
                return Err(format!("Invalid temperature: {}. It should be between 0.0 and 1.0.", temperature_value));
            }
        }
            
        Ok(())
    }
}

impl TranslationRequestBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(temp) = self.temperature {
            let temperature_value = temp.unwrap();
            if temperature_value < 0.0 || temperature_value > 1.0 {
                return Err(format!("Invalid temperature: {}. It should be between 0.0 and 1.0.", temperature_value));
            }
        }
            
        Ok(())
    }
}

impl ImageRequestBuilder {
    fn validate(&self) -> Result<(), String> {
        if let Some(prompt) = &self.prompt {
            if prompt.len() > 1000 {
                return Err(format!("Invalid prompt: {}. It should be less than 1000 characters.", prompt.len()));
            }
        }
            
        Ok(())
    }
}