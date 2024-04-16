use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UrlInfo {
    pub key: String,
    pub long_url: String,
    pub short_url: String,
}

impl UrlInfo {
    pub fn new(key: String, long_url: String, short_url: String) -> Self {
        Self {
            key,
            long_url,
            short_url,
        }
    }
}
#[derive(Debug, Clone, Deserialize)]
pub struct UrlDTO {
    pub long_url: String,
}
