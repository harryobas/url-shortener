use crate::types::{ApiError, ApiResponse, UrlDTO};
use crate::utils::generate_unique_key;
use crate::{
    repositories::DbUrlInfoRepository, 
    use_cases::{get_url_info_with_key, create_url_info, get_url_info_with_long_url},
    AppState
};

use std::collections::HashMap;
use axum::{extract::{State, Path},  Json};

pub async fn shorten_url(
    State(state): State<AppState>,
    Json(url_dto): Json<UrlDTO>
) -> Result<ApiResponse, ApiError> {
    match url_dto {
        UrlDTO{long_url: url} => {
            let conn = Box::new(state.pool);
            let url_repo = DbUrlInfoRepository::new(conn);

            if let Ok(url_info) = get_url_info_with_long_url(
                url_repo.clone(), &url
            ).await {
                return Ok(ApiResponse::JsonData(url_info));
            }else {
                let key = generate_unique_key();
                let short_url = format!("http://localhost:/{key}");
                match create_url_info(url_repo,&key, &url, &short_url)
                    .await {
                        Ok(url_info) => return Ok(ApiResponse::JsonData(url_info)),
                        Err(_e) =>  return Err(ApiError::InternalServerError("something went wrong".to_string())),
                    }
                }
            }
    }
}

pub async fn retrieve_long_url(
    State(state): State<AppState>,
     Path(key): Path<String>
     ) -> Result<ApiResponse, ApiError> {
        let conn = Box::new(state.pool);
        let url_repo = DbUrlInfoRepository::new(conn);

        if let Ok(url_info) = get_url_info_with_key(
            url_repo.clone(), &key
        ).await {
            let status_code: u16 = 307;
            let mut headers: HashMap<String, String> = HashMap::new();
            let body = b"".to_vec();
            headers.insert("Location".into(), url_info.long_url).unwrap();
            
            return Ok(ApiResponse::Redirected{status_code, headers, body});
        }else {
            return Err(ApiError::NotFound("url not found".to_string()));
        }
}





       
    
 

