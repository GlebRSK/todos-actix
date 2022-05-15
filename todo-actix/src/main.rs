// psql -h 127.0.0.1 -p 5432 -U postgres postgres
mod models;
mod configs;
mod handlers;
mod db;

use crate::handlers::*;

use actix_web::{HttpServer, App, web};
use dotenv::dotenv;
use tokio_postgres::NoTls;
use deadpool_postgres::{Runtime};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();

    let config = crate::configs::Config::from_env().unwrap();
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
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
