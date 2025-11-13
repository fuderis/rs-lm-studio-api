use crate::prelude::*;
use super::Role;

// A message
#[derive(Debug, Clone, From, Serialize, Deserialize, Eq, PartialEq)]
#[from(String, "Self { role: Role::User, content: vec![value.into()] }")]
#[from(&str, "Self { role: Role::User, content: vec![value.into()] }")]
pub struct Message {
    pub role: Role,
    pub content: Vec<Content>,
}

impl Message {
    // Creates a new message from text
    pub fn new<S: Into<String>>(role: Role, text: S) -> Self {
        Self {
            role,
            content: vec![text.into().into()],
        }
    }

    /// Returns content chars count
    pub fn len(&self) -> usize {
        let mut count = 0;

        for content in &self.content {
            match &content {
                Content::Text { text } => count += text.chars().count(),
                _ => {}
            }
        }

        count
    }

    /// Returns message texts
    pub fn texts(&self) -> Vec<&String> {
        let mut texts = vec![];

        for content in &self.content {
            match &content {
                Content::Text { text } => texts.push(text),
                _ => {}
            }
        }
        
        texts
    }

    /// Returns message full text
    pub fn text(&self) -> String {
        self.texts()
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}
