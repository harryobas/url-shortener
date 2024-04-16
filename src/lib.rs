
pub mod types;
pub mod db;
mod error;
pub mod repositories;
pub mod utils;
pub mod use_cases;
mod api;

use axum::routing::{get, post};
use sqlx::SqlitePool;
use std::net::SocketAddr;

use log::info;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

pub async fn run(pool: SqlitePool, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting server. port={}", port);

    let state = AppState{pool};
    let app = axum::Router::new().route(
        "/api/v1/", 
        post(api::shorten_url)
    ).with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

    
    
