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
