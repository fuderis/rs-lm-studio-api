use crate::prelude::*;

/// The AI models
#[allow(non_camel_case_types)]
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
    #[serde(rename = "qwen/qwen2.5-vl-7b")]
    #[display = "qwen/qwen2.5-vl-7b"]
    Qwen2_5_Vl_7b,
    
    #[serde(rename = "qwen/qwen3-1.7b")]
    #[display = "qwen/qwen3-1.7b"]
    Qwen3_1_7b,

    #[serde(rename = "qwen/qwen3-4b")]
    #[display = "qwen/qwen3-4b"]
    Qwen3_4b,

    // Code Gemma:
    #[serde(rename = "google/codegemma-2b-GGUF")]
    #[display = "google/codegemma-2b-GGUF"]
    CodeGemma_2b,

    #[serde(rename = "google/codegemma-7b-GGUF")]
    #[display = "google/codegemma-7b-GGUF"]
    CodeGemma_7b,

    // Stable Code:
    #[serde(rename = "stabilityai/stable-code-3b")]
    #[display = "stabilityai/stable-code-3b"]
    StableCode_3b,

    #[serde(rename = "stabilityai/stable-code-instruct-3b")]
    #[display = "stabilityai/stable-code-instruct-3b"]
    StableCodeInstruct_3b,

    // Kimiko:
    #[serde(rename = "mythomax-l2-kimiko-v2-13b")]
    #[display = "mythomax-l2-kimiko-v2-13b"]
    Kimiko_13b,

    // Llama:
    #[serde(rename = "llama-3.1-Nemotron-Nano-4B-v1.1-GGUF")]
    #[display = "llama-3.1-Nemotron-Nano-4B-v1.1-GGUF"]
    Llama3_1_4b,
    
    #[serde(rename = "meta-llama-3.1-8b-instruct")]
    #[display = "meta-llama-3.1-8b-instruct"]
    Llama3_1_8b,

    // Embedding:
    #[serde(rename = "nomic-ai/nomic-embed-text-v1.5")]
    #[display = "nomic-ai/nomic-embed-text-v1.5"]
    NomicEmbedText,

    #[serde(rename = "text-embedding-all-minilm-l6-v2-embedding")]
    #[display = "text-embedding-all-minilm-l6-v2-embedding"]
    AllMiniLmL6,
    
    // Other models:
    #[from]
    #[serde(untagged)]
    #[display = "{0}"]
    Other(String)
}
