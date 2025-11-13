use crate::prelude::*;
use super::{ Choice, StreamChoice, Usage, Role, Message, Embedding };

/// Chat response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<u64>,
    pub model: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<Choice>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data: Vec<Embedding>,
    pub usage: Usage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<HashMap<String, Value>>,
    #[serde(rename = "system_fingerprint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
}

impl Response {
    /// Returns response text
    pub fn text(&self) -> Option<String> {
        if !self.choices.is_empty() {
            let text = self.choices[0].text();
            
            Some(text)
        } else {
            None
        }
    }

    /// Returns embedding data
    pub fn data(&self) -> Option<Vec<Vec<f32>>> {
        if !self.data.is_empty() {
            let data = self.data.clone()
                .into_iter()
                .map(|embed| embed.embedding)
                .collect::<Vec<_>>();

            Some(data)
        } else {
            None
        }
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
            message: Message { role: Role::Assistant, content: vec!["".into()] },
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
                        if let Some(chunk) = choice.text() {
                            self.message.content[0].add_chunk(&chunk);
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
