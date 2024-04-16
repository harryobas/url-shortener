use crate::error;
use crate::types::UrlInfo;
use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

#[cfg(test)]
use mockall::{predicate::*, *};

#[async_trait]
#[cfg_attr(test, automock)]
pub trait UrlInfoRepository {
    async fn find_by_key(&self, key: &str) -> Result<UrlInfo, error::Error>;
    async fn find_by_long_url(&self, url: &str) -> Result<UrlInfo, error::Error>;
    async fn add_url_info(
        &self,
        key: &str,
        long_url: &str,
        short_url: &str,
    ) -> Result<UrlInfo, error::Error>;
}
#[derive(Clone, Debug)]
pub struct DbUrlInfoRepository {
    pool: Box<SqlitePool>,
}

impl DbUrlInfoRepository {
    pub fn new(pool: Box<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UrlInfoRepository for DbUrlInfoRepository {
    async fn find_by_key(&self, key: &str) -> Result<UrlInfo, error::Error> {
        let q = "SELECT * FROM url_info WHERE key = $1";
        match sqlx::query(q)
            .bind(key)
            .map(|row: sqlx::sqlite::SqliteRow| UrlInfo {
                key: row.get("key"),
                long_url: row.get("long_url"),
                short_url: row.get("short_url"),
            })
            .fetch_one(&*self.pool)
            .await
        {
            Ok(url_info) => Ok(url_info),
            Err(e) => Err(error::Error::DatabaseQueryError(e)),
        }
    }

    async fn find_by_long_url(&self, url: &str) -> Result<UrlInfo, error::Error> {
        let q = "SELECT * FROM UrlInfo WHERE long_url = '$1";

        match sqlx::query(q)
            .bind(url)
            .map(|row: sqlx::sqlite::SqliteRow| UrlInfo {
                key: row.get("key"),
                long_url: row.get("long_url"),
                short_url: row.get("short_url"),
            })
            .fetch_one(&*self.pool)
            .await
        {
            Ok(url_info) => Ok(url_info),
            Err(e) => Err(error::Error::DatabaseQueryError(e)),
        }
    }

    async fn add_url_info(
        &self,
        key: &str,
        long_url: &str,
        short_url: &str,
    ) -> Result<UrlInfo, error::Error> {
        let q =
            "INSERT INTO TABLE (url_info) VALUES ($1, $2, $3) RETURNING key, long_url, short_url";
        match sqlx::query(q)
            .bind(key)
            .bind(long_url)
            .bind(short_url)
            .map(|row: sqlx::sqlite::SqliteRow| UrlInfo {
                key: row.get("key"),
                long_url: row.get("long_url"),
                short_url: row.get("short_url"),
            })
            .fetch_one(&*self.pool)
            .await
        {
            Ok(url_info) => Ok(url_info),
            Err(e) => Err(error::Error::DatabaseQueryError(e)),
        }
    }
}
