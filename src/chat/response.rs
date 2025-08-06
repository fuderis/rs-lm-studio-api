use crate::prelude::*;
use super::{ Choice, Usage, Role, Message };

// Chat response
#[derive(Debug, Clone, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    #[serde(default)]
    pub stats: HashMap<String, serde_json::Value>,
    pub system_fingerprint: String,
}

impl Response {
    // Get reponse text
    pub fn text(&self) -> &str {
        &self.choices[0].message.content
    }
}


use futures::StreamExt;
use tokio_stream::wrappers::UnboundedReceiverStream;

// Chat response stream
#[derive(Debug)]
pub struct ResponseReader {
    pub receiver: UnboundedReceiverStream<Result<String>>,
    pub message: Message,
    pub is_ready: bool,
    pub context: bool
}

impl ResponseReader {
    pub fn new(receiver: UnboundedReceiverStream<Result<String>>, context: bool) -> Self {
        Self {
            receiver,
            message: Message { role: Role::Assistant, content: str!("") },
            is_ready: false,
            context
        }
    }

    pub async fn next(&mut self) -> Option<Result<String>> {
        let result = self.receiver.next().await;

        match result {
            Some(result) => {
                match result {
                    Ok(part) => {
                        self.message.content.push_str(&part);
                        Some(Ok(part))
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
