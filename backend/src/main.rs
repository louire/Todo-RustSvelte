use std::net::SocketAddr;

use axum::{Router, routing::get};


#[tokio::main]

async fn main() {
    let app = Router::new().route("/", get(index));
    let address = SocketAddr::from(([0, 0, 0, 0], 5050));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();

}
async fn index() -> &'static str {
    "Hello, World!"
}


