use crate::prelude::*;
use super::{ Role, Message, SystemInfo, };
use tiktoken_rs::{ CoreBPE, cl100k_base };

/// Chat context
#[derive(Clone)]
pub struct Context {
    tokenizer: CoreBPE,
    pub messages: Vec<Message>,
    pub system_prompt: Arc<Mutex<Box<dyn SystemInfo + Send + Sync>>>,
    pub tokens_limit: u32,
    pub total_tokens: u32,
}

impl Context {
    /// Creates a new chat context
    pub fn new(system_prompt: Box<dyn SystemInfo + Send + Sync>, tokens_limit: u32) -> Self {
        // init tokeninzer:
        let tokenizer = cl100k_base().expect("Failed to create tokenizer");
        
        // creating context:
        let mut context = Self {
            tokenizer,
            messages: vec![
                Message::new(Role::System, str!()),
            ],
            system_prompt: Arc::new(Mutex::new(system_prompt)),
            tokens_limit,
            total_tokens: 0,
        };

        // plus tokens count:
        context.total_tokens = context.count_tokens(&context.messages[0].text());

        context
    }

    /// Add a message to context
    pub fn add<M: Into<Message>>(&mut self, message: M) {
        let message = message.into();
        let tokens_count = self.count_tokens(&message.text());

        // add message to context:
        self.messages.push(message);
        self.total_tokens += tokens_count;

        // remove old messages:
        while self.messages.len() > 3 && self.total_tokens > self.tokens_limit {
            let removed_count = self.count_tokens(&self.messages[1].text()) as u32;

            self.messages.remove(1);
            self.total_tokens -= removed_count;
        }
    }

    /// Returns all context messages
    pub fn get(&self) -> Vec<Message> {
        self.messages.clone()
    }

    /// Returns all context messages as single string
    pub fn get_as_string(&self) -> String {
        self.messages.clone()
            .into_iter()
            .map(|msg| msg.text())
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Clears context messages
    pub fn clear(&mut self) {
        self.messages.truncate(1);
    }

    /// Tokenizes text
    pub fn tokenize(&self, text: &str) -> Vec<u32> {
        self.tokenizer.encode_with_special_tokens(text)
    }
    
    /// Calculates tokens count
    pub fn count_tokens(&self, text: &str) -> u32 {
        self.tokenize(text).len() as u32
    }

    /// Updates the actual system info
    pub async fn update_system_info(&mut self) {
        let text = self.system_prompt.lock().await.update();
        self.messages[0].content = vec![Into::<Content>::into(text)];
    }
}
