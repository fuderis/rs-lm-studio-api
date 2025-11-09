use crate::prelude::*;

// Response stream delta
#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct Delta {
    pub content: Option<String>,
}
