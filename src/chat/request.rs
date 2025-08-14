use crate::prelude::*;
use super::{ Model, Message, format::* };

/// Request
#[derive(Debug, Clone, Serialize, Deserialize, From)]
#[serde(untagged)]
pub enum Request {
    #[from] Messages(Messages),
    #[from] Prompt(Prompt),
}


/// Chat messages request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Messages {
    pub model: Model,
    pub messages: Vec<Message>,
    #[serde(skip)]
    pub context: bool,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "response_format")]
    pub format: Option<Format>,
    #[serde(skip)]
    pub skip_think: bool,
}

impl ::std::default::Default for Messages {
    fn default() -> Self {
        Self {
            model: Model::Other(str!()),
            messages: vec![],
            context: true,
            temperature: 0.7,
            max_tokens: -1,
            stream: false,
            format: None,
            skip_think: true,
        }
    }
}


/// Chat prompt request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub model: Model,
    pub prompt: String,
    #[serde(skip)]
    pub context: bool,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
    pub stop: String,
    #[serde(skip)]
    pub skip_think: bool,
}

impl ::std::default::Default for Prompt {
    fn default() -> Self {
        Self {
            model: Model::Other(str!()),
            prompt: str!(),
            context: true,
            temperature: 0.7,
            max_tokens: -1,
            stream: false,
            stop: str!("\n"),
            skip_think: true,
        }
    }
}
