use macron::{ Display, From, Error };

/// Std Result alias
pub type StdResult<T, E> = std::result::Result<T, E>;
/// Result alias
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

// The error
#[derive(Debug, Display, Error, From)]
pub enum Error {
    #[from]
    Io(std::io::Error),

    #[from]
    Reqwest(reqwest::Error),

    #[from]
    Json(serde_json::Error),

    #[display = "Encoded base64 string is invalid"]
    InvalidBase64Url,
}
