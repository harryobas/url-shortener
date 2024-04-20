pub mod api;
pub mod db;
mod error;
pub mod repositories;
pub mod types;
pub mod use_cases;
pub mod utils;

use sqlx::SqlitePool;

#[derive(Debug, Clone)]
pub struct AppState {
    pub pool: SqlitePool
}


