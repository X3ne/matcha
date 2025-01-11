use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::infrastructure::models::oauth::OAuthProviderSqlx;
use crate::shared::types::snowflake::Snowflake;

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct OAuthProvider {
    pub id: Snowflake,
    pub name: String,
    pub active: bool,
}

impl OAuthProvider {
    pub fn from_db(db: &OAuthProviderSqlx) -> Self {
        Self {
            id: db.id,
            name: db.name.clone(),
            active: db.active,
        }
    }
}
