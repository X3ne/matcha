use crate::domain::repositories::user_repo::UserRepository;
use crate::infrastructure::models::user::UserSqlx;
use crate::infrastructure::models::user_profile::{ProfileTagSqlx, UserProfileSqlx};
use crate::infrastructure::repositories::user_repo::PgUserRepository;
use crate::shared::types::user_profile::{Gender, Orientation};
use rand::prelude::SliceRandom;
use rand::Rng;
use sqlx::PgPool;

mod profile;
mod tag;
mod user;

pub async fn init_fake_data(pool: &PgPool) {
    if let Ok(_) = PgUserRepository::get_by_email(pool, "testmf@test.com").await {
        tracing::info!("Fake data already inserted");
        return;
    }

    let mut rng = rand::thread_rng();

    let users = UserSqlx::insert_fake_users(pool).await;
    let tags = ProfileTagSqlx::create_fake_tags(pool).await;

    for (i, user) in users.iter().enumerate() {
        let (gender, orientation) = match i {
            0 => (Gender::Male, Orientation::Female),
            1 => (Gender::Male, Orientation::Male),
            2 => (Gender::Male, Orientation::Bisexual),
            3 => (Gender::Female, Orientation::Male),
            4 => (Gender::Female, Orientation::Female),
            5 => (Gender::Female, Orientation::Bisexual),
            _ => (rng.gen(), rng.gen()),
        };

        let tags = tags.choose_multiple(&mut rand::thread_rng(), 3).collect::<Vec<_>>();

        let profile = UserProfileSqlx::new(
            user.id,
            format!("{} {}", user.last_name, user.first_name),
            gender,
            orientation,
        );
        profile.insert(pool).await;
        profile.link_tags(pool, tags).await;
    }

    tracing::info!("Fake data inserted");
}
