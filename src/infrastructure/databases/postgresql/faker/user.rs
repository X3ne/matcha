use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use chrono::Utc;
use fake::faker::internet::fr_fr::Username;
use fake::faker::name::fr_fr::{FirstName, LastName};
use fake::Fake;
use sqlx::PgPool;

use crate::domain::entities::user::User;
use crate::infrastructure::models::user::UserSqlx;
use crate::shared::types::snowflake::Snowflake;
use crate::shared::utils::generate_random_secure_string;

impl UserSqlx {
    pub fn new(id: Snowflake, username: String, password: String, is_active: bool) -> Self {
        let now = Utc::now().naive_utc();
        let email = format!("{}@test.com", username);
        let first_name: String = FirstName().fake();
        let last_name: String = LastName().fake();

        Self {
            id,
            email,
            username,
            last_name,
            first_name,
            password: Some(password),
            is_active,
            activation_token: generate_random_secure_string(32),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn new_random() -> Self {
        let id = Snowflake::new();
        let now = Utc::now().naive_utc();
        let username: String = Username().fake();
        let email = format!("{}_{}@test.com", username, id);
        let first_name: String = FirstName().fake();
        let last_name: String = LastName().fake();
        let is_active = true;
        let password = "password";

        Self {
            id,
            email,
            username: format!("{}_{}", username, id),
            last_name,
            first_name,
            password: Some(password.to_string()),
            is_active,
            activation_token: generate_random_secure_string(32),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn generate_fake_users() -> Vec<Self> {
        let fake_password = "password";
        let start_id = 7284668132873080833;

        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(fake_password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();

        vec![
            Self::new(
                Snowflake(start_id),
                "testmf".to_string(),
                password_hash.to_string(),
                true,
            ),
            Self::new(
                Snowflake(start_id + 1),
                "testmm".to_string(),
                password_hash.to_string(),
                true,
            ),
            Self::new(
                Snowflake(start_id + 2),
                "testmb".to_string(),
                password_hash.to_string(),
                true,
            ),
            Self::new(
                Snowflake(start_id + 3),
                "testfm".to_string(),
                password_hash.to_string(),
                true,
            ),
            Self::new(
                Snowflake(start_id + 4),
                "testff".to_string(),
                password_hash.to_string(),
                true,
            ),
            Self::new(
                Snowflake(start_id + 5),
                "testfb".to_string(),
                password_hash.to_string(),
                true,
            ),
            Self::new(
                Snowflake(start_id + 6),
                "not_activated".to_string(),
                password_hash.to_string(),
                false,
            ),
        ]
    }

    pub async fn insert_fake_users(pool: &PgPool) -> Vec<User> {
        let mut users = Self::generate_fake_users();

        for _ in 0..1000 {
            users.push(Self::new_random());
        }

        for user in users.clone() {
            sqlx::query!(
                r#"
                INSERT INTO "user" (id, email, username, last_name, first_name, password, is_active, activation_token, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                "#,
                user.id.as_i64(),
                user.email,
                user.username,
                user.last_name,
                user.first_name,
                user.password,
                user.is_active,
                user.activation_token,
                user.created_at,
                user.updated_at
            )
                .execute(pool)
                .await
                .expect("Failed to insert fake users");
        }

        users.into_iter().map(|u| u.into()).collect()
    }
}
