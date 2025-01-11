use std::time::Duration;

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool};

pub async fn connect(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let database_url = database_url.replace("\"", "");
    tracing::info!("Connecting to database: {}", database_url);

    let pool = PgPoolOptions::new()
        .min_connections(0)
        .max_connections(5)
        .max_lifetime(Some(Duration::from_secs(60 * 60)))
        .connect(&database_url)
        .await?;

    Ok(pool)
}
