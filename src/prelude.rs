#![allow(unused_imports)]

pub use crate::{ Result, Error };

pub(crate) use macron::*;
pub(crate) use serde::{ Serialize, Deserialize };

pub(crate) use std::collections::HashMap;
pub(crate) use std::format as fmt;

pub(crate) use serde_json as json;
pub(crate) use json::{ json };
