use base64::engine::{general_purpose, Engine};
use sha2::{Digest, Sha256};

pub fn random_bytes(amount: usize) -> Vec<u8> {
    (0..amount).map(|_| fastrand::u8(..)).collect()
}

pub fn url_encode(bytes: &[u8]) -> String {
    general_purpose::STANDARD
        .encode(bytes)
        .replace('+', "-")
        .replace('/', "_")
        .replace('=', "")
}

pub fn sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    return result.as_slice().to_vec();
}

pub fn between(input: &str, prefix: &str, postfix: &str) -> String {
    let mut b = 0;
    while !input[b..].starts_with(prefix) {
        b += 1;
    }
    b += prefix.len();

    let mut e = b;
    while !input[e..].starts_with(postfix) {
        e += 1;
    }

    input[b..e].to_owned()
}
