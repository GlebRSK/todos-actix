use crate::models::{Status, CreateTodoList, ResultResponse, AppState}; 
use crate::db;
use crate::errors::{AppError};

use actix_web::{Responder, HttpResponse, web};
use deadpool_postgres::{Pool, Client};
use slog::{Logger, o, crit, error};

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
   
    pool.get().
        await.
        map_err( |err| {
            let sublog = log.new(o!("cause" => err.to_string()));
            crit!(sublog, "Error getting client!");
            AppError::db_error(err)
        })
}

pub fn log_error(log: Logger) -> Box<dyn Fn(AppError) -> AppError> {
    Box::new(move |err| {
        let sublog = log.new(o!("cause" => err.cause.clone()));
        error!(sublog, "{}", err.message());
        err
    })
}

pub async fn status() -> impl Responder {

    HttpResponse::Ok()
        .json(Status { status: "Ok".to_string() })
}

pub async fn get_todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {

    let log = state.log.new( o!("handler" => "get_todos"));
    
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::get_todos(&client).await;

    result.map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}


pub async fn get_items(state: web::Data<AppState>, path: web::Path<(i32, )>) -> Result<impl Responder, AppError> {

    let log = state.log.new( o!("handler" => "get_items"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::get_items(&client, path.0).await;

    result.map(|items| HttpResponse::Ok().json(items))
        .map_err(log_error(log))
}


pub async fn create_todo(state: web::Data<AppState>, json: web::Json<CreateTodoList>) ->  Result<impl Responder, AppError> {

    let log = state.log.new( o!("handler" => "create_todo"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::create_todo(&client, json.title.clone()).await;

    result.map(|todo| HttpResponse::Ok().json(todo))
        .map_err(log_error(log))
}


pub async fn check_todo(state: web::Data<AppState>, path: web::Path<(i32,i32)>) ->  Result<impl Responder, AppError> {

    let log = state.log.new( o!("handler" => "check_todo"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::check_todo(&client, path.0, path.1).await;

    result.map(|updated| HttpResponse::Ok().json(ResultResponse {success: updated}))
        .map_err(log_error(log))
}