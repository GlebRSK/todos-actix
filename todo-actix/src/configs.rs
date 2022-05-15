use serde::Deserialize;
use config::{ConfigError, Config as RConfig, Environment};


#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32
}


#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: deadpool_postgres::Config,
}


impl Config {

    pub fn from_env() -> Result<Self, ConfigError> {

        let builder = RConfig::builder()
            .set_default("default", "1")?
            .add_source(Environment::default())
            .build()?;
        
        builder.try_deserialize::<Config>()
    }
}