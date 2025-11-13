use crate::prelude::*;
use base64::{ engine::general_purpose, Engine as _ };
use std::fs;

/// The image base64 URL
#[derive(Debug, Clone, From, Serialize, Deserialize, Eq, PartialEq)]
pub struct Image {
    pub url: String,
}

impl Image {
    /// Validates a base64 URL
    pub fn validate_base64(base64_url: &str) -> bool {
        general_purpose::STANDARD.decode(base64_url).is_ok()
    }
    
    /// Creates a new image from base64 url (example: "data:image/png;base64,iVBORw0KGgoA...")
    pub fn from_base64<S: Into<String>>(base64_url: S) -> Result<Self> {
        let base64_url = base64_url.into();
        
        if Self::validate_base64(
            &base64_url.split_once(",").ok_or(Error::InvalidBase64Url)?.1
        ) {
            Ok(Self {
                url: base64_url,
            })
        } else {
            Err(Error::InvalidBase64Url.into())
        }
    }
    
    /// Creates a new image from file path
    pub fn from_file<P: Into<PathBuf>>(file_path: P) -> Result<Self> {
        // reading file:
        let file_path = file_path.into();
        let file_content = fs::read(&file_path)?;

        // reading mime-type:
        let mime_type = match file_path.extension().and_then(|e| e.to_str()) {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            _ => "application/octet-stream",
        };

        // encoding into base64:
        let encoded = general_purpose::STANDARD.encode(&file_content);
        let base64_url = format!("data:{};base64,{}", mime_type, encoded);

        Ok(Self { url: base64_url })
    }
}
