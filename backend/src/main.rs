use std::net::SocketAddr;
//SocketAddr is a type that represents a socket address, which is a combination of an IP address 
//and a port number

use axum::{Router, routing::get};
//axum is a web framework that is built on top of hyper, the low-level HTTP library for Rust
use axum_error::Result;
//axum_error is a crate that provides a custom error type for axum applications

#[tokio::main]

async fn main() -> Result<()>{
    //result is a type that represents either success (Ok) or failure (Err)
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?; 
    //USE THE ? OPERATOR TO RETURN AN ERROR IF THE DATABASE URL IS NOT FOUND


    let app = Router::new().route("/", get(index));
    let address = SocketAddr::from(([0, 0, 0, 0], 5050));
    //ok() is used to convert the Result<T, E> to Result<T, Infallible>
    Ok((axum::Server::bind(&address)
    .serve(app.into_make_service())
    .await
    .unwrap()))
    
}
async fn index() -> &'static str {
    "Hello, World!"
}


