use std::io;

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

#[derive(Debug, Deserialize)]
pub struct Completion {
    pub version: String,
    pub engine: String,
    pub prompt: String,
    pub parameters: Option<Vec<Parameter>>,
}

impl Chat {
    pub fn from(json: &str) -> Result<Chat, io::Error> {
        let prompt: Chat = serde_json::from_str(&json)?;
        Ok(prompt)
    }
}

impl Completion {
    pub fn from(json: &str) -> Result<Completion, io::Error> {
        let prompt: Completion = serde_json::from_str(&json)?;
        Ok(prompt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_return_result_when_parse_baisc_chat() {
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

        let prompt = Chat::from(json).unwrap();
        assert_eq!(prompt.version, "0.2");
        assert_eq!(prompt.engine, "chat-bison");
    }

    #[test]
    fn test_should_return_result_when_parse_chat_if_contain_context() {
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

        let prompt = Chat::from(json).unwrap();
        assert_eq!(prompt.version, "0.2");
        assert_eq!(prompt.engine, "chat-bison");

        let messages = prompt.messages;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].role, "user");
        assert_eq!(messages[0].name, None);
        assert!(messages[0].function_call.is_none());
        assert_eq!(
            messages[0].content,
            Some("Write a hello world in js".to_string())
        );

        let parameters = prompt.parameters.unwrap();
        assert_eq!(parameters.len(), 1);
        assert_eq!(parameters[0].name, "temperature");
        assert_eq!(parameters[0].value, 0.1);
    }

    #[test]
    fn test_should_return_result_when_parse_baisc_completion() {
        let json = r#"  
        {
            "$schema": "../schema/completion-schema-0.3.json",
            "version": "0.2",
            "engine": "text-bison",
            "prompt": "I'm hungry and I want to",
            "parameters": [
                {
                "name": "temperature",
                "value": 0.1
                }
            ]
        }
        "#;

        let prompt = Completion::from(json).unwrap();
        assert_eq!(prompt.version, "0.2");
        assert_eq!(prompt.engine, "text-bison");
        assert_eq!(prompt.prompt, "I'm hungry and I want to");
        let parameters = prompt.parameters.unwrap();
        assert_eq!(parameters.len(), 1);
        assert_eq!(parameters[0].name, "temperature");
        assert_eq!(parameters[0].value, 0.1);
    }

    #[test]
    fn test_should_return_error_when_parse_chat_if_is_not_json() {
        let json = "!@$!$#";

        let prompt = Chat::from(json);
        assert!(prompt.is_err());
    }

    #[test]
    fn test_should_return_error_when_parse_completion_if_is_not_json() {
        let json = "!@$!$#";

        let prompt = Completion::from(json);
        assert!(prompt.is_err());
    }

    #[test]
    fn test_should_return_error_when_parse_chat_if_required_filed_is_missing() {
        let json = "{}";

        let prompt = Chat::from(json);
        assert!(prompt.is_err());
    }

    #[test]
    fn test_should_return_error_when_parse_completion_if_required_filed_is_missing() {
        let json = "{}";

        let prompt = Completion::from(json);
        assert!(prompt.is_err());
    }
}
