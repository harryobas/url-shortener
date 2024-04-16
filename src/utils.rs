use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn generate_unique_key() -> String {
    let rng = thread_rng();
    let key: String = rng
        .sample_iter(&Alphanumeric)
        .take(5)
        .map(char::from)
        .collect();

    key
}
