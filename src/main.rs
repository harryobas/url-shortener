
use url_shortener::{api, db, AppState};
use axum::routing::{get, post, delete};
//use axum_cors::cors;
use tower_http::cors::{CorsLayer, Any};
use axum::http::Method;
use std::net::SocketAddr;

use log::info;

fn init_logger(port: u16) {
    std::env::set_var("RUST_LOG", "url_shortener=info");
    env_logger::init();

    info!("Starting server on port={}", port);
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_origin(Any)
        .max_age(std::time::Duration::from_secs(3600));

    let pool = db::init_db(db::DATABASE_URL).await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let state = AppState { pool, addr };
    
    let app = axum::Router::new()
        .route("/", post(api::shorten_url))
        .route("/:key", get(api::retrieve_long_url))
        .layer(cors)
        .with_state(state);

    init_logger(addr.port());

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

