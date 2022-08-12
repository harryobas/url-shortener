
extern crate urlshortener;
use urlshortener::client::UrlShortener;

use crate::store::Store;

pub type Error = urlshortener::providers::ProviderError;

pub struct UrlShortnerService{}

impl UrlShortnerService{

    pub fn shorten_url<T:Store>(long_url: &str, store: T)-> Result<String, Error>{
        if let Some(short_url) = store.retrive(long_url){
            return Ok(short_url)  
        }else{
            let us = UrlShortener::new().unwrap();
            let provider = urlshortener::providers::Provider::IsGd;

            match us.generate(long_url, &provider){
                Ok(short_url) => {
                    store.insert(long_url, &short_url);
                    Ok(short_url)
                }
                Err(e) => Err(e)
            }

        }
    }
}