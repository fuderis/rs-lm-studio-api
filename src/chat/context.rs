// use crate::prelude::*;
use super::{ Role, Message };

// Chat context
#[derive(Debug, Clone)]
pub struct Context {
    messages: Vec<Message>,
    context_tokens: usize,
    context_limit: usize
}

impl Context {
    // Creates a new chat context
    pub fn new<S>(context: S, context_limit: usize) -> Self
    where
        S: Into<String>
    {
        Self {
            messages: vec![Message::new(Role::System, context.into())],
            context_tokens: 0,
            context_limit
        }
    }

    // Add a message to context
    pub fn add<M>(&mut self, message: M)
    where
        M: Into<Message>
    {
        let message = message.into();

        // add message to context:
        self.context_tokens += message.content.chars().count();
        self.messages.push(message);

        // WARN: This is a temporary solution!
        // TODO: Write function for dividing text into tokens..
        let tokens_limit = self.context_limit * 4;
        
        // remove old extra messages:
        while self.messages.len() > 2 && self.context_tokens > tokens_limit {
            self.context_tokens -= self.messages[1].content.chars().count();
            self.messages.remove(1);
        }
    }

    // Get all context messages
    pub fn get(&self) -> Vec<Message> {
        self.messages.clone()
    }

    // Clear context messages
    pub fn clear(&mut self) {
        self.messages.truncate(1);
    }
}
