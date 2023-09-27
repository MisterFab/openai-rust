use openai_rust::{
    types::{ChatCompletionRequestBuilder, FunctionBuilder, MessageRequestBuilder, ParametersBuilder, PropertyBuilder, Role},
    OpenAIClient,
};
use serde_json::{self, Value};
use std::collections::HashMap;
use std::error::Error;
use std::env;

fn get_current_weather(location: String) -> String {
    format!("The weather in {} is 72 degrees and sunny.", location)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::new(api_key);

    let messages = vec![
        MessageRequestBuilder::default()
            .role(Role::User)
            .content("What is the weather like in Boston?")
            .build()?
    ];
    
    let mut properties = HashMap::default();
    properties.insert(
        "location".to_string(),
        PropertyBuilder::default()
            .param_type("string")
            .description("The city and state, e.g. San Francisco, CA")
            .build()?
    );

    let parameters = ParametersBuilder::default()
        .param_type("object")
        .properties(properties)
        .required(vec!["location".to_string()])
        .build()?;

    let functions = vec![
        FunctionBuilder::default()
            .name("get_current_weather")
            .description("Get the current weather in a given location")
            .parameters(parameters)
            .build()?];

    let request = ChatCompletionRequestBuilder::default()
                    .model("gpt-4")
                    .messages(messages.clone())
                    .functions(functions)
                    .build()?;

    let response = client.chat(request).await?;

    let mut available_functions: HashMap<String, fn(String) -> String> = HashMap::new();
    available_functions.insert("get_current_weather".to_string(), get_current_weather);

    for choice in &response.choices {
        if let Some(function_call) = &choice.message.function_call {
            if let Some(function_to_call) = available_functions.get(&function_call.name) {
                let arguments: Value = serde_json::from_str(&function_call.arguments)?;

                let location = arguments["location"].as_str().ok_or("location not found")?.to_string();
                let function_response = function_to_call(location);

                let mut new_messages = messages.clone();
                new_messages.push(MessageRequestBuilder::default()
                                    .role(Role::Assistant)
                                    .function_call(function_call.clone())
                                    .build()?
                );
                new_messages.push(MessageRequestBuilder::default()
                                    .role(Role::Function)
                                    .name(&function_call.name)
                                    .content(function_response)
                                    .build()?
                );

                let second_request = ChatCompletionRequestBuilder::default()
                                        .model("gpt-4")
                                        .messages(new_messages)
                                        .build()?;
                                
                let second_response = client.chat(second_request).await?;

                for second_choice in &second_response.choices {
                    if let Some(content) = &second_choice.message.content {
                        println!("Response: {}", content);
                    }
                }
            }
        } else {
            println!("No function call found");
        }
    }

    Ok(())
}