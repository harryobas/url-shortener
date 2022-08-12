pub mod model{
    use serde::{Serialize, Deserialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct LongUrlDTO{
        pub long_url: String
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub struct ShortUrl{
        pub short_url: String
    }

    
}

pub mod routes{
    pub const SHORT_URL : &str = "/api/shorten"; 
}