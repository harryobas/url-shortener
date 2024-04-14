#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database query error: {0}")]
    DatabaseQueryError(#[from] sqlx::Error),

}