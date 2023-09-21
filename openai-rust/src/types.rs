use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    System,
    Assistant,
    Function,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionRequest {
    pub messages: Vec<MessageRequest>,
    pub model: Option<String>,
    pub max_tokens: Option<i32>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Function>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<ChoiceWrapper>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChoiceWrapper {
    pub index: i32,
    pub message: MessageResponse,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageRequest {
    pub role: Role,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<FunctionCall>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageResponse {
    pub role: Role,
    pub content: Option<String>,
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: Parameters,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Property {
    #[serde(rename = "type")]
    pub param_type: String,
    pub description: String,
}

impl Default for ChatCompletionRequest {
    fn default() -> Self {
        Self {
            messages: vec![MessageRequest::new()],
            model: Some("gpt-3.5-turbo".to_string()),
            max_tokens: None,
            temperature: None,
            top_p: None,
            n: None,
            functions: None,
            function_call: None,
        }
    }
}

impl ChatCompletionRequest {
    pub fn new(messages: Vec<MessageRequest>) -> Self {
        Self {
            messages,
            ..Default::default()
        }
    }

    pub fn model<S: Into<String>>(mut self, model: S) -> Self {
        self.model = Some(model.into());
        self
    }

    pub fn messages(mut self, messages: Vec<MessageRequest>) -> Self {
        self.messages = messages;
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

    pub fn functions(mut self, functions: Vec<Function>) -> Self {
        self.functions = Some(functions);
        self
    }

    pub fn function_call(mut self, function_call: String) -> Self {
        self.function_call = Some(function_call);
        self
    }
}

impl Default for MessageRequest {
    fn default() -> Self {
        Self {
            role: Role::User,
            content: Some("No content".to_string()),
            name: None,
            function_call: None,
        }
    }
}

impl MessageRequest {
    pub fn new() -> Self {
        MessageRequest {
            ..Default::default()
        }
    }

    pub fn role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    pub fn content<S: Into<String>>(mut self, content: S) -> Self {
        self.content = Some(content.into());
        self
    }

    pub fn name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn function_call(mut self, function_call: Option<FunctionCall>) -> Self {
        self.function_call = function_call;
        self
    }
}

impl Function {
    pub fn new<S: Into<String>>(name: S, description: S, parameters: Parameters) -> Self {
        Function {
            name: name.into(),
            description: description.into(),
            parameters,
        }
    }
}

impl Parameters {
    pub fn new<S: Into<String>>(param_type: S, properties: HashMap<String, Property>, required: Vec<String>) -> Self {
        Parameters {
            param_type: param_type.into(),
            properties,
            required,
        }
    }
}

impl Property {
    pub fn new<S: Into<String>>(param_type: S, description: S) -> Self {
        Property {
            param_type: param_type.into(),
            description: description.into(),
        }
    }
}