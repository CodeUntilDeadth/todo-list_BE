mod api;
mod config;

use std::net::SocketAddr;

use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();

    let listener = TcpListener::bind(SocketAddr::new([0, 0, 0, 0].into(), *config::SERVER_PORT))
        .await
        .unwrap();

    info!("OpenAPI is opened!");

    info!(
        "Application is started and listening on port {:?}",
        *config::SERVER_PORT
    );

    axum::serve(listener, api::build()).await.unwrap();
}
