use crate::prelude::*;
use super::schema::*;

/// Response format kind
#[derive(Debug, Display, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum FormatKind {
    #[serde(rename = "json_schema")]
    #[display = "json_schema"]
    Json,
}

/// Response format
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Format {
    #[serde(rename = "type")]
    pub kind: FormatKind,
    #[serde(rename = "json_schema")]
    pub schema: JsonSchema,
}

impl Format {
    pub fn json<S: Into<String>>(name: S, schemes: Vec<Schema>, strict: bool) -> Self {
        Self {
            kind: FormatKind::Json,
            schema: JsonSchema {
                name: name.into(),
                schemes: Schemes::OneOf(schemes),
                strict,
            }
        }
    }
}
