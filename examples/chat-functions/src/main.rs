use openai_rust::{
    types::{ChatCompletionRequest, Function, MessageRequest, Parameters, Property, Role},
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
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::new(api_key);

    let messages = vec![MessageRequest::new().role(Role::User).content("What is the weather like in Boston?")];
    
    let mut properties = HashMap::new();
    properties.insert("location".to_string(), Property::new("string", "The city and state, e.g. San Francisco, CA"));
    let parameters = Parameters::new("object", properties, vec!["location".to_string()]);
    let functions = vec![Function::new("get_current_weather", "Get the current weather in a given location", parameters)];

    let request = ChatCompletionRequest::new(messages.clone()).model("gpt-4").functions(functions);
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
                new_messages.push(MessageRequest::new().role(Role::Assistant).function_call(Some(function_call.clone())));
                new_messages.push(MessageRequest::new().role(Role::Function).name(&function_call.name).content(function_response));

                let second_request = ChatCompletionRequest::new(new_messages).model("gpt-4");
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
