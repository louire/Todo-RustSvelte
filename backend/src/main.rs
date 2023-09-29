use std::net::SocketAddr;
//SocketAddr is a type that represents a socket address, which is a combination of an IP address 
//and a port number
use axum::{Router, routing::{get, post}, extract::{State, Path}, Json, Form};
//axum is a web framework that is built on top of hyper, the low-level HTTP library for Rust
use axum_error::Result;
//axum_error is a crate that provides a custom error type for axum applications
use sqlx::{sqlite::SqlitePool, pool};
//sqlx is a database toolkit for Rust . . . maybe i dont need this comment but i'll leave it here for now
use tower_http::cors::CorsLayer;
//tower_http is a crate that provides middleware for HTTP applications
use serde::{Serialize, Deserialize};
//serde is a crate that provides a framework for serializing and deserializing Rust data structures

#[tokio::main]

async fn main() -> Result<()>{
    //result is a type that represents either success (Ok) or failure (Err)
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?; 
    //USE THE ? OPERATOR TO RETURN AN ERROR IF THE DATABASE URL IS NOT FOUND

    let pool = SqlitePool::connect(&url).await?;

    //Create router for server
    let app = Router::new()
                                    .route("/", get(list))
                                    .route("/create", get(create))
                                    .route("/delete/:id", get(delete))
                                    .route("/update", get(update))
                                    .with_state(pool)
                                    .layer(CorsLayer::very_permissive());
    //with_state() is used to pass the database pool to the handler
    let address = SocketAddr::from(([0, 0, 0, 0], 9990));
    println!("Starting server on http://{address}");
    //ok() is used to convert the Result<T, E> to Result<T, Infallible>
    Ok((axum::Server::bind(&address)
    .serve(app.into_make_service())
    .await
    .unwrap()))
    //unwrap() is used to extract the value from a Result<T, E> if it is Ok
    
}


#[derive(Serialize, Deserialize)]
struct Todo{
    id: i64,
    description: String,
    done: bool,
}
//derive is used to automatically implement traits for structs

#[derive(Deserialize)]
struct NewTodo{
    description: String,
}


async fn list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Todo>>> {
    // List all notes
    let todos = sqlx::query_as!(Todo, "SELECT id, description, done FROM todos ORDER BY id")
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create(State(pool): State<SqlitePool>, Form(todo): Form<NewTodo>) -> Result<String> {
    sqlx::query!("INSERT INTO todos (description) VALUES (?)", todo.description).execute(&pool).await?;
    Ok(format!("Created todo with description: {}", todo.description))
}

async fn delete(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> Result<String> {
    sqlx::query!("DELETE FROM todos WHERE id = ?", id).execute(&pool).await?;
    Ok(format!("Deleted todo with id: {}", id))
}

async fn update(State(pool): State<SqlitePool>, Form(todo): Form<Todo>) -> Result<String> {

    sqlx::query!("UPDATE todos SET description = ?, done = ? WHERE id = ?", todo.description, todo.done, todo.id).execute(&pool).await?;
    Ok(format!("Succesfully Updated todo with id: {}", todo.id))
}