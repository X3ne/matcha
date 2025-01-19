use serde::Deserialize;

#[cfg(test)]
fn load_env() {
    dotenvy::from_filename(".env.test").ok();
}

#[cfg(not(test))]
fn load_env() {
    dotenvy::dotenv().ok();
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub version: String,
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    pub origins: Vec<String>,
    pub telemetry_collector_endpoint: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        load_env();

        let config = config::Config::builder()
            .add_source(
                config::Environment::default()
                    .list_separator(",")
                    .with_list_parse_key("origins")
                    .try_parsing(true),
            )
            .set_default("version", env!("CARGO_PKG_VERSION"))
            .unwrap()
            .set_default("host", "0.0.0.0")
            .unwrap()
            .set_default("port", 3000)
            .unwrap()
            .build()?;

        let cfg: Config = config.try_deserialize()?;

        Ok(cfg)
    }
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct OAuth2Config {
    pub ft_client_id: String,
    pub ft_client_secret: String,
    pub ft_redirect_uri: String,
}

impl OAuth2Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        load_env();

        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("oauth2").try_parsing(true))
            .build()?;

        let cfg: OAuth2Config = config.try_deserialize()?;

        Ok(cfg)
    }
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct CookieConfig {
    pub session_secret: String,
    pub session_ttl: i64,
    pub same_site: String,
    pub secure: bool,
    pub http_only: bool,
}

impl CookieConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        load_env();

        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("cookie").try_parsing(true))
            .set_default("same_site", "strict") // strict, lax, none
            .unwrap()
            .set_default("secure", true)
            .unwrap()
            .set_default("http_only", true)
            .unwrap()
            .build()?;

        let cfg: CookieConfig = config.try_deserialize()?;

        Ok(cfg)
    }
}

#[cfg(feature = "mailing")]
#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct SmtpConfig {
    pub email_from_name: String,
    pub email_from_email: String,
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[cfg(feature = "mailing")]
impl SmtpConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        load_env();

        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("smtp").try_parsing(true))
            .build()?;

        let cfg: SmtpConfig = config.try_deserialize()?;

        Ok(cfg)
    }
}

#[derive(Debug, Default, Clone, Deserialize, PartialEq, Eq)]
pub struct S3Config {
    pub access_key_id: String,
    pub secret_access_key: String,
    pub region: String,
    pub endpoint: String,
    pub bucket_name: String,
}

impl S3Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        load_env();

        let config = config::Config::builder()
            .add_source(config::Environment::with_prefix("S3").try_parsing(true))
            .build()?;

        let cfg: S3Config = config.try_deserialize()?;

        Ok(cfg)
    }
}
