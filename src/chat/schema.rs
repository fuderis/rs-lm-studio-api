 use crate::prelude::*;

/// Response format JSON schema
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct JsonSchema {
    pub name: String,
    #[serde(rename = "schema")]
    pub schemes: Schemes,
    pub strict: bool,
}


/// Response format schemes
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Schemes {
    #[serde(rename = "oneOf")]
    OneOf(Vec<Schema>),

    #[serde(rename = "anyOf")]
    AnyOf(Vec<Schema>),

    #[serde(rename = "allOf")]
    AllOf(Vec<Schema>),
}


/// Response format schema kind
#[derive(Debug, Display, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SchemaKind {
    Object,
    Array,
    String,
    Integer,
    Float,
    Boolean,
    Null,
}


/// Response format schema
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Schema {
    #[serde(rename = "type")]
    pub kind: SchemaKind,
    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Box<Schema>>>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<String>,
}

impl ::std::default::Default for Schema {
    fn default() -> Self {
        Self {
            kind: SchemaKind::String,
            title: str!(),
            description: str!(),
            format: None,
            properties: None,
            required: vec![],
        }
    }
}

impl Schema {
    /// Creates an object scheme
    pub fn object<S: Into<String>>(name: S, descr: S, props: HashMap<S, Schema>) -> Self {
        Self {
            kind: SchemaKind::Object,
            title: name.into(),
            description: descr.into(),
            format: None,
            properties: Some(
                props.into_iter()
                    .map(|(key, val)| (key.into(), Box::new(val)))
                    .collect::<HashMap<_, _>>()
            ),
            required: vec![],
        }
    }

    /// Creates an array scheme
    pub fn array<S: Into<String>>(name: S, descr: S) -> Self {
        Self {
            kind: SchemaKind::Array,
            title: name.into(),
            description: descr.into(),
            format: None,
            properties: None,
            required: vec![],
        }
    }
    
    /// Creates a string scheme
    pub fn string<S: Into<String>>(descr: S, format: Option<S>) -> Self {
        Self {
            kind: SchemaKind::String,
            title: "".into(),
            description: descr.into(),
            format: format.map(|s| s.into()),
            properties: None,
            required: vec![],
        }
    }
    
    /// Creates an integer scheme
    pub fn integer<S: Into<String>>(descr: S) -> Self {
        Self {
            kind: SchemaKind::Integer,
            title: "".into(),
            description: descr.into(),
            format: None,
            properties: None,
            required: vec![],
        }
    }
    
    /// Creates a float scheme
    pub fn float<S: Into<String>>(descr: S) -> Self {
        Self {
            kind: SchemaKind::Float,
            title: "".into(),
            description: descr.into(),
            format: None,
            properties: None,
            required: vec![],
        }
    }
    
    /// Creates a boolean scheme
    pub fn boolean<S: Into<String>>(descr: S) -> Self {
        Self {
            kind: SchemaKind::Boolean,
            title: "".into(),
            description: descr.into(),
            format: None,
            properties: None,
            required: vec![],
        }
    }
    
    /// Creates a null scheme
    pub fn null<S: Into<String>>(descr: S) -> Self {
        Self {
            kind: SchemaKind::Null,
            title: "".into(),
            description: descr.into(),
            format: None,
            properties: None,
            required: vec![],
        }
    }
}
