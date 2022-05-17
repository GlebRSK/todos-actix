# todos-actix

DB: PostgreSQL

for testing
cargo install diesel_cli --no-default-features --features postgres

problem - note: /usr/bin/ld: cannot find -lpq (on debian 10)
              sudo apt install libpq-dev

diesel setup
diesel migration generate create_db
diesel migration run --database-url "postgres://postgres:postgres@localhost:5432/postgres"



REST 
GET /todos - return list of all todo from table todo_list
POST /todos - create new todo