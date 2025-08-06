use crate::prelude::*;
use super::{ Model, Message };

/// Request
#[derive(Debug, Clone, Serialize, From)]
#[serde(untagged)]
pub enum Request {
    #[from] Messages(Messages),
    #[from] Prompt(Prompt),
}


/// Chat messages request
#[derive(Debug, Clone, Serialize)]
pub struct Messages {
    #[serde(skip)] pub context: bool,
    #[serde(skip)] pub think: bool,
    pub model: Model,
    pub messages: Vec<Message>,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
}

impl ::std::default::Default for Messages {
    fn default() -> Self {
        Self {
            context: true,
            think: false,
            model: Model::Custom(str!()),
            messages: vec![],
            temperature: 0.7,
            max_tokens: -1,
            stream: false,
        }
    }
}


/// Chat prompt request
#[derive(Debug, Clone, Serialize)]
pub struct Prompt {
    #[serde(skip)] pub context: bool,
    #[serde(skip)] pub think: bool,
    pub model: Model,
    pub prompt: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
    pub stop: String,
}

impl ::std::default::Default for Prompt {
    fn default() -> Self {
        Self {
            context: true,
            think: false,
            model: Model::Custom(str!()),
            prompt: str!(),
            temperature: 0.7,
            max_tokens: -1,
            stream: false,
            stop: str!("\n"),
        }
    }
}
