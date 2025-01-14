use crate::domain::entities::user::User;
use crate::infrastructure::models::user::UserSqlx;
use crate::shared::types::snowflake::SNOWFLAKE_GENERATOR;
use crate::shared::utils::generate_random_secure_string;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use chrono::Utc;
use fake::faker::name::fr_fr::{FirstName, LastName};
use fake::Fake;
use sqlx::PgPool;

impl UserSqlx {
    pub fn new(username: String, password: String, is_active: bool) -> Self {
        let now = Utc::now().naive_utc();
        let email = format!("{}@test.com", username);
        let first_name: String = FirstName().fake();
        let last_name: String = LastName().fake();
        let id = SNOWFLAKE_GENERATOR.generate();

        let activation_token = match is_active {
            true => None,
            false => Some(generate_random_secure_string(32)),
        };

        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .expect("Failed to hash password")
            .to_string();

        Self {
            id,
            email,
            username,
            last_name,
            first_name,
            password: Some(password_hash),
            is_active,
            activation_token,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn generate_fake_users() -> Vec<Self> {
        let fake_password = "password";

        vec![
            Self::new("testmf".to_string(), fake_password.to_string(), true),
            Self::new("testmm".to_string(), fake_password.to_string(), true),
            Self::new("testmb".to_string(), fake_password.to_string(), true),
            Self::new("testfm".to_string(), fake_password.to_string(), true),
            Self::new("testff".to_string(), fake_password.to_string(), true),
            Self::new("testfb".to_string(), fake_password.to_string(), true),
            Self::new("not_activated".to_string(), fake_password.to_string(), false),
        ]
    }

    pub async fn insert_fake_users(pool: &PgPool) -> Vec<User> {
        let users = Self::generate_fake_users();

        let mut created_users: Vec<User> = vec![];
        for user in users {
            let created_user = sqlx::query_as!(
                User,
                r#"
                INSERT INTO "user" (id, email, username, last_name, first_name, password, is_active, activation_token, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                RETURNING *
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
                .fetch_one(pool)
                .await
                .expect("Failed to insert fake users");

            created_users.push(created_user);
        }

        created_users
    }
}
