use std::sync::{Arc, Mutex};

use apistos::ApiComponent;
use garde::Validate;
use once_cell::sync::Lazy;
use redis::{FromRedisValue, RedisWrite, ToRedisArgs};
use schemars::JsonSchema;
use schemars::_serde_json::Value;
use schemars::schema::{InstanceType, Metadata, SchemaObject, SingleOrVec};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use snowflake::SnowflakeIdGenerator;

static SNOWFLAKE_GENERATOR: Lazy<SnowflakeGenerator> = Lazy::new(|| SnowflakeGenerator::new(1, 1));

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, sqlx::Type, Validate, ApiComponent)]
#[sqlx(transparent)]
pub struct Snowflake(#[garde(custom(validate_snowflake))] i64);

impl Snowflake {
    pub fn new() -> Self {
        SNOWFLAKE_GENERATOR.generate()
    }

    pub fn as_i64(&self) -> i64 {
        self.0
    }
}

impl From<i64> for Snowflake {
    fn from(value: i64) -> Self {
        Snowflake(value)
    }
}

impl std::fmt::Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ToRedisArgs for Snowflake {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + RedisWrite,
    {
        self.0.write_redis_args(out)
    }
}

impl FromRedisValue for Snowflake {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        i64::from_redis_value(v).map(Snowflake)
    }
}

// Snowflake ID are serialized as strings to avoid overflow issues
impl Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let value = s.parse::<i64>().map_err(serde::de::Error::custom)?;
        Ok(Snowflake(value))
    }
}

fn validate_snowflake(value: &i64, context: &()) -> garde::Result {
    if value.to_string().len() != 19 {
        return Err(garde::Error::new("Snowflake ID must be 19 characters long"));
    }
    Ok(())
}

impl JsonSchema for Snowflake {
    fn schema_name() -> String {
        "Snowflake".to_string()
    }

    fn json_schema(_: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        let schema = SchemaObject {
            instance_type: Some(SingleOrVec::Single(Box::new(InstanceType::String))),
            format: Some("snowflake".to_string()),
            string: Some(Default::default()),
            metadata: Some(Box::new(Metadata {
                description: Some(
                    "A 64 bit integer unique identifier (serialized as string to avoid overflow issues)".to_string(),
                ),
                examples: vec![Value::String("1869760527605956608".to_string())],
                ..Default::default()
            })),
            ..Default::default()
        };
        schema.into()
    }
}

#[derive(Clone)]
pub struct SnowflakeGenerator {
    inner: Arc<Mutex<SnowflakeIdGenerator>>,
}

impl SnowflakeGenerator {
    pub fn new(machine_id: i32, node_id: i32) -> Self {
        let generator = SnowflakeIdGenerator::new(machine_id, node_id);
        SnowflakeGenerator {
            inner: Arc::new(Mutex::new(generator)),
        }
    }

    pub fn generate(&self) -> Snowflake {
        let mut generator = self.inner.lock().unwrap();
        Snowflake(generator.generate())
    }
}
