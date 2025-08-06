use crate::prelude::*;
use super::{ Message, Delta };

// Response choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub index: usize,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: String,
    pub message: Message,
}


// Response choice stream
#[derive(Debug, Clone, Deserialize)]
pub struct StreamChoice {
    pub delta: Delta,
}

impl StreamChoice {
    /// Returns response string
    pub fn text(&self) -> Option<&String> {
        self.delta.content.as_ref()
    }

    /// Returns mutable response string
    pub fn text_mut(&mut self) -> Option<&mut String> {
        self.delta.content.as_mut()
    }
}
