
mod store;
mod services;
mod api;


use rocket::launch;
use rocket::routes;

use crate::store::{UrlStore, UrlDb};
use crate::api::url_shortner_controller::shorten_url;
use std::{collections::HashMap, sync:: Mutex};


#[launch]
fn rocket() -> _ {
    let db = UrlDb::new(Mutex::new(HashMap::new()));
    let store = UrlStore::new(db);

    rocket::build()
        .manage(store)
        .mount("/", routes![shorten_url])

}
