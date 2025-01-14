use crate::infrastructure::models::user_profile::{ProfileTagSqlx, UserProfileSqlx};
use crate::shared::types::snowflake::{Snowflake, SNOWFLAKE_GENERATOR};
use crate::shared::types::user_profile::{Gender, Orientation};
use chrono::Utc;
use fake::faker::lorem::fr_fr::Paragraph;
use fake::Fake;
use geo_types::Point;
use geozero::wkb;
use geozero::wkb::Decode;
use rand::Rng;
use sqlx::PgPool;

impl UserProfileSqlx {
    pub fn new(user_id: Snowflake, name: String, gender: Gender, orientation: Orientation) -> Self {
        let id = SNOWFLAKE_GENERATOR.generate();
        let bio = Paragraph(1..3).fake();
        let mut rng = rand::thread_rng();
        let age = rng.gen_range(18..50);
        let now = Utc::now().naive_utc();

        Self {
            id,
            user_id,
            name,
            avatar_hash: None,
            bio,
            age,
            gender,
            sexual_orientation: orientation,
            location: Decode {
                geometry: Some(Point::new(0.0, 0.0).into()),
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
            let id = SNOWFLAKE_GENERATOR.generate();

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
