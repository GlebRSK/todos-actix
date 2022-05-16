// psql -h 127.0.0.1 -p 5432 -U postgres postgres
mod models;
mod configs;
mod handlers;
mod db;
mod errors;

use crate::handlers::*;
use crate::models::AppState;

use actix_web::{HttpServer, App, web};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use deadpool_postgres::{Runtime};
use slog::{Logger, Drain, o, info};
use slog_term;
use slog_async;

fn configure_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let console_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let console_drain = slog_async::Async::new(console_drain).build().fuse();
    slog::Logger::root(console_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();

    let config = crate::configs::Config::from_env().unwrap();
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let log = configure_log();

    info!(log, "Starting web server at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(
                    AppState {
                        pool: pool.clone(),
                        log: log.clone()
                    }
                )
            )
            .route("/", web::get().to(status))
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todos{_:/?}", web::post().to(create_todo))
            .route("/todos/{list_id}/items", web::get().to(get_items))
            .route("/todos/{list_id}/items/{item_id}{_:/?}", web::get().to(check_todo))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
