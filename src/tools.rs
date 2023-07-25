use base64::engine::{general_purpose, Engine};
use regex::Regex;
use sha2::{Digest, Sha256};

pub fn random_bytes(amount: usize) -> Vec<u8> {
    (0..amount).map(|_| rand::random::<u8>()).collect()
}

pub fn url_encode(bytes: &[u8]) -> String {
    let mut based = general_purpose::STANDARD.encode(bytes);

    for (pattern, replacement) in [(r"\+", "-"), (r"\/", "_"), ("=", "")] {
        let regex = Regex::new(pattern).unwrap();
        based = regex.replace_all(&based, replacement).to_string();
    }

    based
}

pub fn sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    return result.as_slice().to_vec();
}
