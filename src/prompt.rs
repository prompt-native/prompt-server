use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    #[serde(rename = "type")]
    pub function_type: String,
    pub required: Option<bool>,
    pub description: Option<String>,
    pub enums: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Function {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Vec<FunctionParameter>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub role: String,
    pub name: Option<String>,
    pub content: Option<String>,
    pub function_call: Option<FunctionCall>,
}

#[derive(Debug, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub version: String,
    pub engine: String,
    pub context: Option<String>,
    pub parameters: Option<Vec<Parameter>>,
    pub examples: Option<Vec<Message>>,
    pub messages: Vec<Message>,
    pub functions: Option<Vec<Function>>,
}

impl Chat {
    pub fn from(json: &str) -> Chat {
        let prompt: Chat = serde_json::from_str(&json).unwrap();
        prompt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_basic_chat() {
        let json = r#"  
        {
            "$schema": "../schema/chat-schema.json",
            "version": "0.2",
            "engine": "chat-bison",
            "messages": [
              {
                "role": "user",
                "content": "Write a hello world in js"
              }
            ],
            "parameters": [
              {
                "name": "temperature",
                "value": 0.1
              }
            ]
          }
        "#;

        let prompt = Chat::from(&json);
        assert_eq!(prompt.version, "0.2");
        assert_eq!(prompt.engine, "chat-bison");
    }
}