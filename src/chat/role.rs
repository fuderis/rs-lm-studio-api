use crate::prelude::*;

// A message role
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl Role {
    /// Returns true if it's the system role
    pub fn is_system(&self) -> bool {
        self == &Self::System
    }

    /// Returns true if it's the user role
    pub fn is_user(&self) -> bool {
        self == &Self::User
    }

    /// Returns true if it's the assistant role
    pub fn is_assist(&self) -> bool {
        self == &Self::Assistant
    }
}
