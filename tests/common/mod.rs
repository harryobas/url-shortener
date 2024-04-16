use dotenv::dotenv;
use sqlx::Executor;
use sqlx::SqlitePool;
use url_shortener::types::UrlInfo;

pub async fn establish_db_connection() -> Result<SqlitePool, sqlx::error::Error> {
    dotenv().ok();

    let pool = SqlitePool::connect(&std::env::var("TEST_DATABASE_URL").unwrap()).await?;

    Ok(pool)
}

pub async fn setup_test_database(pool: &SqlitePool) -> anyhow::Result<()> {
    let url_infos = vec![
        UrlInfo {
            key: String::from("key1"),
            long_url: String::from("https://google.com"),
            short_url: String::from("https://google.com/key1"),
        },
        UrlInfo {
            key: String::from("key2"),
            long_url: String::from("https://yahoo.com"),
            short_url: String::from("https://yahoo.com/key2"),
        },
        UrlInfo {
            key: String::from("key3"),
            long_url: String::from("https://facebook.com"),
            short_url: String::from("https://facebook.com/key3"),
        },
        UrlInfo {
            key: String::from("key4"),
            long_url: String::from("https://twitter.com"),
            short_url: String::from("https://twitter.com/key4"),
        },
    ];

    for url_info in url_infos {
        let q = "INSERT INTO url_info (key, long_url, short_url) VALUES ($1, $2, $3)";
        sqlx::query(q)
            .bind(&url_info.key)
            .bind(&url_info.long_url)
            .bind(&url_info.short_url)
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn clean_up_test_db(pool: &SqlitePool) {
    pool.execute("DELETE FROM url_info").await.unwrap();
}
