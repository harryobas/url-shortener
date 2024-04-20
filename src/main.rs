
use url_shortener::{api, db, AppState};
use axum::routing::{get, post};

use std::net::SocketAddr;

//extern crate env_logger;
use log::info;

fn init_logger(port: u16) {
    std::env::set_var("RUST_LOG", "server=info");
    env_logger::init();

    info!("Starting server on port={}", port);
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {

    init_logger(3000);

    let pool = db::init_db(db::DATABASE_URL).await.unwrap();
    let state = AppState { pool };

    let app = axum::Router::new()
        .route("/", post(api::shorten_url))
        .route("/:key", get(api::retrieve_long_url))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

