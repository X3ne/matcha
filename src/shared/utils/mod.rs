use rand::rngs::OsRng;
use rand::RngCore;

pub fn generate_random_secure_string(length: usize) -> String {
    let mut buffer = vec![0u8; length];
    OsRng.fill_bytes(&mut buffer);
    hex::encode(buffer)
}
