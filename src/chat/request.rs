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
    pub model: Model,
    pub messages: Vec<Message>,
    #[serde(skip)] pub context: bool,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
    #[serde(skip)] pub skip_think: bool,
}

impl ::std::default::Default for Messages {
    fn default() -> Self {
        Self {
            model: Model::Custom(str!()),
            messages: vec![],
            context: true,
            temperature: 0.7,
            max_tokens: -1,
            stream: false,
            skip_think: false,
        }
    }
}


/// Chat prompt request
#[derive(Debug, Clone, Serialize)]
pub struct Prompt {
    pub model: Model,
    pub prompt: String,
    pub temperature: f32,
    pub max_tokens: i32,
    pub stream: bool,
    pub stop: String,
    #[serde(skip)] pub skip_think: bool,
}

impl ::std::default::Default for Prompt {
    fn default() -> Self {
        Self {
            model: Model::Custom(str!()),
            prompt: str!(),
            temperature: 0.7,
            max_tokens: -1,
            stream: false,
            stop: str!("\n"),
            skip_think: false,
        }
    }
}
