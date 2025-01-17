use crate::infrastructure::models::user_profile::{ProfileTagSqlx, UserProfileSqlx};
use crate::shared::types::snowflake::Snowflake;
use crate::shared::types::user_profile::{Gender, Orientation};
use chrono::Utc;
use fake::faker::lorem::fr_fr::Paragraph;
use fake::Fake;
use geo_types::Point;
use geozero::wkb;
use geozero::wkb::Decode;
use rand::Rng;
use sqlx::PgPool;
use std::f64::consts::PI;

const DEFAULT_LATITUDE: f64 = 48.85580453177924;
const DEFAULT_LONGITUDE: f64 = 2.3520428683913375;
const EARTH_RADIUS_KM: f64 = 6371.0;

fn random_location_around(latitude: f64, longitude: f64, radius_km: f64) -> Point<f64> {
    let mut rng = rand::thread_rng();

    let angle = rng.gen_range(0.0..2.0 * PI);

    let random_fraction = rng.gen::<f64>();
    let distance_km = radius_km * random_fraction.sqrt();

    let distance_rad = distance_km / EARTH_RADIUS_KM;

    let new_lat = latitude.to_radians() + distance_rad * angle.sin();

    let new_lon = longitude.to_radians() + distance_rad * angle.cos() / latitude.to_radians().cos();

    let final_lat = new_lat.to_degrees();
    let final_lon = new_lon.to_degrees();

    Point::new(final_lon, final_lat)
}

impl UserProfileSqlx {
    pub fn new(user_id: Snowflake, name: String, gender: Gender, orientation: Orientation) -> Self {
        let id = Snowflake::new();
        let bio = Paragraph(1..3).fake();
        let mut rng = rand::thread_rng();
        let age = rng.gen_range(18..50);
        let now = Utc::now().naive_utc();
        let location = random_location_around(DEFAULT_LATITUDE, DEFAULT_LONGITUDE, 20.0);

        Self {
            id,
            user_id,
            name,
            avatar_hash: None,
            picture_hashes: vec![],
            bio,
            age,
            gender,
            sexual_orientation: orientation,
            location: Decode {
                geometry: Some(location.into()),
            },
            rating: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub async fn insert(&self, pool: &PgPool) {
        sqlx::query!(
            r#"
            INSERT INTO user_profile (id, user_id, name, avatar_hash, bio, age, gender, sexual_orientation, location)
            VALUES ($1, $2, $3, $4, $5, $6, $7::gender, $8::sexual_orientation, $9::geometry)
            "#,
            self.id.as_i64(),
            self.user_id.as_i64(),
            self.name,
            self.avatar_hash,
            self.bio,
            self.age,
            self.gender as _,
            self.sexual_orientation as _,
            wkb::Encode(self.location.geometry.clone().unwrap()) as _
        )
        .execute(pool)
        .await
        .expect("Failed to insert user profile");
    }

    pub async fn link_tags(&self, pool: &PgPool, tags: Vec<&ProfileTagSqlx>) {
        for tag in tags {
            let id = Snowflake::new();

            sqlx::query!(
                r#"
                INSERT INTO join_user_profile_tag (id, user_profile_id, profile_tag_id)
                VALUES ($1, $2, $3)
                "#,
                id.as_i64(),
                self.id.as_i64(),
                tag.id.as_i64()
            )
            .execute(pool)
            .await
            .expect("Failed to link tag to user profile");
        }
    }
}
