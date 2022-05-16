use crate::models::{TodoList, TodoItem};
use crate::errors::{AppError, AppErrorType};

use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;


/*
inp: pg client
return: vector of todo or custom error
*/
pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, AppError> {

    let statement = client
        .prepare("select * from todo_list order by id desc limit 10") // get query
        .await // waiting
        .map_err(AppError::db_error)?; // return instance for AppErrorType

    let todos = client.query(&statement, &[])
        .await
        .expect("Error getting todo list")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();
    
    
    Ok(todos)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, AppError> {

    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id")
        .await
        .map_err(AppError::db_error)?;

    let items = client.query(&statement, &[&list_id])
        .await
        .expect("Error getting items")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(items)
}

pub async fn create_todo(client: &Client, title: String) -> Result<TodoList, AppError> {

    let statement = client
        .prepare("insert into todo_list (title) values ($1) returning id, title")
        .await
        .map_err(AppError::db_error)?;

    client.query(&statement, &[&title])
        .await
        .expect("Error getting items")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>()
        .pop()
        .ok_or(AppError {
            message: Some("Error creating TODO list".to_string()),
            cause: Some("Unknown error".to_string()),
            error_type: AppErrorType::DBError
        })
}

pub async fn check_todo(client: &Client, list_id: i32, item_id: i32) -> Result<bool, AppError> {

    let statement = client
        .prepare("update todo_item set checked=true where list_id=$1 and id=$2 and checked=false")
        .await
        .map_err(AppError::db_error)?;

    let result = client.query(&statement, &[&list_id, &item_id])
        .await
        .expect("Error getting items");

    match result  {
        ref updated => Ok(true),
        _           => Ok(false)
    }
}