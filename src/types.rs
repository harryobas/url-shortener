use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum:: Json;
use axum::response::Response;
use axum::body::Body;


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

#[derive(Debug, Clone, Serialize)]
pub enum ApiError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
}


impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg).into_response()
            }
            ApiError::InternalServerError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            ApiError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, msg).into_response()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ApiResponse {
    Ok,
    Created,
    Redirected {
        status_code: u16,
        headers: HashMap<String, String>,
        body: Vec<u8>,
    },
    JsonData(UrlInfo),
}

impl IntoResponse for ApiResponse { 
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::Ok => (StatusCode::OK).into_response(),
            ApiResponse::Created => (StatusCode::CREATED).into_response(),
            ApiResponse::Redirected{status_code, headers, body} =>{
                let headers_key = headers.keys()
                    .map(|s|s.clone())
                    .collect::<Vec<String>>()
                    .first().unwrap().clone();

                let headers_value = headers.values()
                    .map(|s| s.clone())
                    .collect::<Vec<String>>()
                    .first().unwrap().clone();
            

                Response::builder()
                    .status(StatusCode::from_u16(status_code).unwrap())
                    .header(&headers_key, &headers_value)
                    .body(Body::from(body)).unwrap()       
            } 
            ApiResponse::JsonData(url_info) => (StatusCode::OK, Json(url_info)).into_response(),
        }
    }
    
}

