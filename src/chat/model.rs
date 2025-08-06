use crate::prelude::*;

#[derive(Debug, Display, Clone, From, Eq, PartialEq, Serialize, Deserialize)]
pub enum Model {
    // Gemma:
    #[serde(rename = "google/gemma-2-2b-it")]
    #[display = "google/gemma-2-2b-it"]
    Gemma2_2b,

    #[serde(rename = "google/gemma-2-9b-it")]
    #[display = "google/gemma-2-9b-it"]
    Gemma2_9b,

    #[serde(rename = "google/gemma-2-27b-it")]
    #[display = "google/gemma-2-27b-it"]
    Gemma2_27b,

    #[serde(rename = "google/gemma-3-1b-it-qat")]
    #[display = "google/gemma-3-1b-it-qat"]
    Gemma3_1b,

    #[serde(rename = "google/gemma-3-4b-it-qat")]
    #[display = "google/gemma-3-4b-it-qat"]
    Gemma3_4b,

    #[serde(rename = "google/gemma-3-12b-it-qat")]
    #[display = "google/gemma-3-12b-it-qat"]
    Gemma3_12b,

    #[serde(rename = "google/gemma-3-27b-it-qat")]
    #[display = "google/gemma-3-27b-it-qat"]
    Gemma3_27b,

    // Qwen:
    #[serde(rename = "qwen/qwen3-1.7b")]
    #[display = "qwen/qwen3-1.7b"]
    Qwen3_1_7b,

    #[serde(rename = "qwen/qwen3-4b")]
    #[display = "qwen/qwen3-4b"]
    Qwen3_4b,
    
    // Custom:
    #[from]
    #[serde(untagged)]
    #[display = "{0}"]
    Custom(String)
}
