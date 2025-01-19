pub mod validation;

use rand::rngs::OsRng;
use rand::RngCore;

use crate::domain::constants::{CDN_BASE_URL, PROFILE_IMAGES_PATH};

pub fn generate_random_secure_string(length: usize) -> String {
    let mut buffer = vec![0u8; length];
    OsRng.fill_bytes(&mut buffer);
    hex::encode(buffer)
}

pub fn build_cdn_profile_image_uri(hash: &str) -> String {
    format!("{}{}/{}", CDN_BASE_URL, PROFILE_IMAGES_PATH, hash)
}
