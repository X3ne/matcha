use crate::infrastructure::models::profile_tag::ProfileTagSqlx;
use crate::shared::types::snowflake::Snowflake;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ProfileTag {
    pub id: Snowflake,
    pub name: String,
}

impl From<ProfileTagSqlx> for ProfileTag {
    fn from(tag: ProfileTagSqlx) -> Self {
        Self {
            id: tag.id,
            name: tag.name,
        }
    }
}
