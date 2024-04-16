mod common;

use url_shortener::repositories::*;

#[tokio::test]
async fn test_find_by_key_with_invalid_key() -> anyhow::Result<()> {
    let pool = common::establish_db_connection().await?;
    common::setup_test_database(&pool).await?;

    let repo = DbUrlInfoRepository::new(Box::new(pool.clone()));
    assert!(repo.find_by_key("ide00").await.is_err());
    common::clean_up_test_db(&pool).await;

    Ok(())
}
#[tokio::test]
async fn test_find_by_key_with_valid_key() -> anyhow::Result<()> {
    let pool = common::establish_db_connection().await?;
    common::setup_test_database(&pool).await?;

    let repo = DbUrlInfoRepository::new(Box::new(pool.clone()));
    assert!(repo.find_by_key("key1").await.is_ok());
    common::clean_up_test_db(&pool).await;

    Ok(())
}
