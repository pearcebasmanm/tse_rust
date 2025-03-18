mod controller;
mod entities;
mod service;

use axum::serve;
use tokio::net::TcpListener;

use controller::controller;

#[tokio::main]
async fn main() {
    let app = controller();
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    serve(listener, app).await.unwrap();
}
