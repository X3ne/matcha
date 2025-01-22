pub mod fame;
pub mod validation;

use chrono::Datelike;
use geo::{Distance, Haversine};
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

pub fn approx_distance_km(geom1: &geo_types::Geometry<f64>, geom2: &geo_types::Geometry<f64>) -> f64 {
    match (geom1, geom2) {
        (geo_types::Geometry::Point(p1), geo_types::Geometry::Point(p2)) => Haversine::distance(*p1, *p2) / 1000.0,
        _ => panic!("Only Point geometries are supported for now"),
    }
}

pub fn calculate_age(birth_date: chrono::NaiveDate) -> i32 {
    let now = chrono::Utc::now().naive_utc();
    now.year() - birth_date.year()
}
