use serde::Deserialize;
use config::{ConfigError, Config as RConfig, Environment};
use slog::{Logger, Drain, o};
use slog_term;
use slog_async;
use tokio_postgres::NoTls;
use deadpool_postgres::{Pool, Runtime};

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

    pub fn configure_pool(&self) -> Pool {
        self.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
    }

    pub fn configure_log() -> Logger {
        let decorator = slog_term::TermDecorator::new().build();
        let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
        let console_drain = slog_async::Async::new(console_drain).build().fuse();
        slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
    }
}