mod api;
mod config;

use std::net::SocketAddr;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), *config::SERVER_PORT))
        .await
        .unwrap();

    axum::serve(listener, api::build()).await.unwrap();
}