use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn random_alphanumeric(length: usize) -> String {
    let random_alphanumeric: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();
    random_alphanumeric
}
