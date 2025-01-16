use crate::infrastructure::models::user_profile::ProfileTagSqlx;
use crate::shared::types::snowflake::Snowflake;
use fake::Fake;
use sqlx::PgPool;

impl ProfileTagSqlx {
    pub fn new() -> Self {
        let id = Snowflake::new();
        let name: String = fake::faker::lorem::fr_fr::Word().fake();

        Self {
            id,
            name: format!("{}_{}", name, id),
        }
    }

    pub async fn create_fake_tags(pool: &PgPool) -> Vec<Self> {
        let tags = vec![
            Self::new(),
            Self::new(),
            Self::new(),
            Self::new(),
            Self::new(),
            Self::new(),
            Self::new(),
        ];

        for tag in &tags {
            Self::insert(pool, tag).await;
        }

        tags
    }

    pub async fn insert(pool: &PgPool, tag: &ProfileTagSqlx) {
        sqlx::query!(
            r#"
            INSERT INTO profile_tag (id, name)
            VALUES ($1, $2)
            "#,
            tag.id.as_i64(),
            tag.name
        )
        .execute(pool)
        .await
        .expect("Failed to insert tag");
    }
}
