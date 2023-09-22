use serde::{Deserialize, Serialize};
use derive_builder::Builder;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    System,
    Assistant,
    Function,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ChatCompletionRequest {
    #[builder(default = "String::from(\"gpt-3.5-turbo\")")]
    pub model: String,
    #[builder(default = "vec![MessageRequest::default()]")]
    pub messages: Vec<MessageRequest>,    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<i32>,
    pub stream: Option<bool>,
    pub stop: Option<Vec<String>>,
    pub max_tokens: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChoiceWrapper>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct ChoiceWrapper {
    pub index: i32,
    pub message: MessageResponse,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageRequest {
    pub role: Role,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct MessageResponse {
    pub role: Role,
    pub content: Option<String>,
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Parameters,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Parameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
#[builder(setter(into))]
pub struct Property {
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}