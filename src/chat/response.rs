use crate::prelude::*;
use super::{ Choice, StreamChoice, Usage, Role, Message };

/// Chat response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    #[serde(default)]
    pub stats: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_fingerprint: Option<String>,
}

impl Response {
    /// Returns response text
    pub fn text(&self) -> String {
        self.choices[0].text().clone().unwrap()
    }
}


use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;

/// Chat stream reader
#[derive(Debug)]
pub struct ResponseReader {
    pub receiver: UnboundedReceiverStream<Result<StreamChoice>>,
    pub message: Message,
    pub is_ready: bool,
    pub context: bool
}

impl ResponseReader {
    /// Creates a new stream reader
    pub fn new(receiver: UnboundedReceiverStream<Result<StreamChoice>>, context: bool) -> Self {
        Self {
            receiver,
            message: Message { role: Role::Assistant, content: str!("") },
            is_ready: false,
            context
        }
    }

    /// Iters next stream choice
    pub async fn next(&mut self) -> Option<Result<StreamChoice>> {
        let result = self.receiver.next().await;

        match result {
            Some(result) => {
                match result {
                    Ok(choice) => {
                        if let Some(text) = choice.text() {
                            self.message.content.push_str(&text);
                        }

                        Some(Ok(choice))
                    },

                    Err(e) => Some(Err(e))
                }
            },

            _ => {
                self.is_ready = true;
                None
            }
        }
    }
}
