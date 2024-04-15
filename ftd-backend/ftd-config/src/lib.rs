use serde::Deserialize;
use std::fmt;

const DEFAULT_CONFIG_DIR: &str = "./config";
const DEV_CONFIG_DIR: &str = "../_config";
const DEFAULT_NETWORK: &str = "polkadot";

#[derive(Clone, Debug, Deserialize)]
pub enum Environment {
    Development,
    Test,
    Production,
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Environment::Development => write!(f, "Development"),
            Environment::Test => write!(f, "Test"),
            Environment::Production => write!(f, "Production"),
        }
    }
}

impl From<&str> for Environment {
    fn from(env: &str) -> Self {
        match env.to_lowercase().as_str() {
            "testing" | "test" => Environment::Test,
            "production" | "prod" => Environment::Production,
            "development" | "dev" => Environment::Development,
            _ => panic!("Unknown environment: {env}"),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CommonConfig {
    pub recovery_retry_seconds: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SubstrateConfig {
    pub chain: String,
    pub chain_display: String,
    pub sidecar_url: String,
    pub connection_timeout_seconds: u64,
    pub request_timeout_seconds: u64,
    pub token_ticker: String,
    pub token_decimals: usize,
    pub token_format_decimal_points: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LogConfig {
    pub ftd_level: String,
    pub other_level: String,
}

/// PostgreSQL configuration. PostgreSQL is used for historical indexed blockchain data storage.
#[derive(Clone, Debug, Deserialize)]
pub struct PostgreSQLConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: String,
    pub pool_max_connections: u32,
    pub connection_timeout_seconds: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MetricsConfig {
    pub host: String,
    pub indexer_port: u16,
}

/// Whole configuration.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub common: CommonConfig,
    pub log: LogConfig,
    pub postgres: PostgreSQLConfig,
    pub substrate: SubstrateConfig,
    pub metrics: MetricsConfig,
}

impl Config {
    fn new() -> Result<Self, config::ConfigError> {
        let env = Environment::from(
            std::env::var("FTD_ENV")
                .unwrap_or_else(|_| "Production".into())
                .as_str(),
        );
        let network = std::env::var("FTD_NETWORK").unwrap_or_else(|_| DEFAULT_NETWORK.into());
        let config_dir = if cfg!(debug_assertions) {
            std::env::var("FTD_CONFIG_DIR").unwrap_or_else(|_| DEV_CONFIG_DIR.into())
        } else {
            std::env::var("FTD_CONFIG_DIR").unwrap_or_else(|_| DEFAULT_CONFIG_DIR.into())
        };
        let config = config::Config::builder()
            .set_default("env", env.to_string())?
            .add_source(config::File::with_name(&format!("{config_dir}/base")))
            .add_source(config::File::with_name(&format!(
                "{config_dir}/network/{network}",
            )))
            .add_source(config::File::with_name(&format!(
                "{}/env/{}",
                config_dir,
                env.to_string().to_lowercase()
            )))
            .add_source(config::Environment::with_prefix("ftd").separator("__"))
            .build()?;
        config.try_deserialize()
    }

    pub fn get_postgres_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?sslmode=disable",
            self.postgres.username,
            self.postgres.password,
            self.postgres.host,
            self.postgres.port,
            self.postgres.database_name,
        )
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new().expect("Config can't be loaded.")
    }
}