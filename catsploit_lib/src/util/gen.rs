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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_alphanumeric() {
        let res = random_alphanumeric(6);
        assert_eq!(res.len(), 6);
        assert!(res.chars().all(char::is_alphanumeric));
    }
}
