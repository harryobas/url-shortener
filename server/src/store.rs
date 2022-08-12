use std::{collections::HashMap, 
    sync::{Arc, Mutex}
};


pub type UrlDb = Arc<Mutex<HashMap<String, String>>>;

pub trait Store{
    fn retrive(&self, long_url: &str) -> Option<String>;
    fn insert(&self, long_url: &str, short_url: &str);

}
#[derive(Clone, Debug)]
pub struct UrlStore{
    db: UrlDb
}

impl UrlStore{
    pub fn new(db: UrlDb) -> Self{
        UrlStore { db }
    }
}

impl Store for UrlStore{
    fn retrive(&self, long_url: &str) -> Option<String> {
        let guard = self.db.lock().unwrap();
        let url = guard.get(long_url)
        .map(|x| x.to_string());

        match url{
            Some( val) => Some(val),
            _ => None
        }
 
    }

    fn insert(&self, long_url: &str, short_url: &str){
        let mut v = self.db.lock().unwrap();
        let _ = v.insert(String::from(long_url), String::from(short_url));
    }
}