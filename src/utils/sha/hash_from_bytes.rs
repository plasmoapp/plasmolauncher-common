use sha1::{Sha1, Digest};

pub fn hash_from_bytes(bytes: Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    hex::encode(result)
}