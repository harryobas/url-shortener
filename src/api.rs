use crate::types::{UrlDTO, UrlInfo};
use crate::utils::generate_unique_key;
use crate::{repositories, use_cases, AppState};
use axum::{extract::State, http::StatusCode, Json};

pub async fn shorten_url(
    State(state): State<AppState>,
    Json(url_dto): Json<UrlDTO>,
) -> (StatusCode, Json<UrlInfo>) {
    let conn = Box::new(state.pool);
    let url_repo = repositories::DbUrlInfoRepository::new(conn);
    let long_url = url_dto.long_url;

    if let Ok(url_info) = use_cases::get_url_info_with_long_url(url_repo.clone(), &long_url).await {
        return (StatusCode::CREATED, Json(url_info));
    } else {
        let key = generate_unique_key();
        let short_url = format!("http://localhost/{key}");
        let url_info = use_cases::create_url_info(url_repo, &key, &long_url, &short_url)
            .await
            .unwrap();
        (StatusCode::CREATED, Json(url_info))
    }
}
