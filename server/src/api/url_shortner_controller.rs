
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::store::UrlStore;
use crate::services::url_shortner_service::{Error, UrlShortnerService};


#[rocket::post("/api/shorten", format = "application/json", data = "<long_url_dto>")]
pub fn shorten_url(
    long_url_dto: Json<LongUrlDTO>, 
    db: &State<UrlStore>) -> Result<Json<ShortUrl>, Status>{
        let long_url = long_url_dto.long_url;
        if let Ok(short_url) = UrlShortnerService::shorten_url(long_url, *db.inner()){
   
        }
}

fn error_status(error: Error) -> Status {
    match error {
        Error::Connection => Status::ServiceUnavailable,
        Error::Deserialize => Status::InternalServerError
    }
}