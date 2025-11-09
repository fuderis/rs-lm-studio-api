use crate::prelude::*;
use super::{ Message, Delta };

// Response choice
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Choice {
    pub index: usize,
    pub logprobs: Option<serde_json::Value>,
    pub finish_reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl Choice {
    /// Returns response string
    pub fn text(&self) -> Option<String> {
        Some(
            self.message
            .as_ref()
            .map_or_else(
                || self.text.clone().unwrap_or_default(),
                |msg| msg.content.clone(),
            )
        )
    }

    /// Returns mutable response string
    pub fn text_mut(&mut self) -> Option<&mut String> {
        self.message
            .as_mut()
            .map_or_else(
                || self.text.as_mut(),
                |msg| Some(&mut msg.content),
            )
    }
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
