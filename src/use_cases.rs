
use crate::types::*;
use crate::repositories::*;
use crate::error::Error;
pub async fn create_url_info<T: UrlInfoRepository>(
    url_repo: T, 
    key: &str,
    long_url: &str,
    short_url: &str
) -> Result<UrlInfo, Error>
{
    let url_info = url_repo.add_url_info(key, long_url, short_url).await?;
    Ok(url_info)
}

pub async fn get_url_info_with_key<T: UrlInfoRepository>(
    url_repo: T,
    key: &str
) -> Result<UrlInfo, Error> 
{
    let url_info = url_repo.find_by_key(key).await?;
    Ok(url_info)
}

pub async fn get_url_info_with_long_url<T: UrlInfoRepository>(
    url_repo: T,
    long_url: &str
) -> Result<UrlInfo, Error>
{
    let url_info = url_repo.find_by_long_url(long_url).await?;
    Ok(url_info)
}

#[cfg(test)]
mod tests {
    use mockall::predicate::eq;
    use super::*;

    fn url_info() -> UrlInfo{
        let key = "key1".to_owned();
        let long_url = "http://example.com".to_owned();
        let short_url = "http://example.com/key1".to_owned();

        UrlInfo {
            key,
            long_url,
            short_url
        }
    }

    #[tokio::test]
    async fn test_create_url_info() {
        let url_info = url_info();

        let url_info = UrlInfo::new(
            url_info.key.clone(), 
            url_info.long_url.clone(), 
            url_info.short_url.clone()
        );
        let mut url_repo = MockUrlInfoRepository::new();

        url_repo
            .expect_add_url_info()
            .with(
                eq(url_info.key.clone()), 
                eq(url_info.long_url.clone()),
                 eq(url_info.short_url.clone())
                )
            .once()
            .returning(move |_,_,_| {

                let url_info = url_info.clone();
                
                Box::pin(async move {
                    Ok(url_info)
                })
            });
            

        let result = create_url_info(
            url_repo, 
            "key1", 
            "http://example.com", 
            "http://example.com/key1"
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_url_info_with_key() {
        let url_info =  url_info();
        let mut url_repo = MockUrlInfoRepository::new();

        url_repo
            .expect_find_by_key()
            .with(eq(url_info.key.clone()))
            .once()
            .returning(move |_| {

                let url_info = url_info.clone();

                Box::pin(async move {
                    Ok(url_info)
                })

            });

            let result = get_url_info_with_key(url_repo, "key1").await;
            assert!(result.is_ok());

    }

    #[tokio::test]
    async fn test_get_url_info_with_long_url() {
        let url_info =  url_info();
        let mut url_repo = MockUrlInfoRepository::new();

        url_repo
            .expect_find_by_long_url()
            .with(eq(url_info.long_url.clone()))
            .once()
            .returning(move |_| {

                let url_info = url_info.clone();

                Box::pin(async move {
                    Ok(url_info)
                })

            });

            let result = get_url_info_with_long_url(url_repo, "http://example.com").await;
            assert!(result.is_ok());
    }


}
