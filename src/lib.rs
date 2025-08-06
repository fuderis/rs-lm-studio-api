#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

pub mod error;      pub use error::{ StdResult, Result, Error };
pub mod prelude;

pub mod chat;  pub use chat::*;
