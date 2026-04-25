use axum::{Router, routing::get};
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let app = Router::new()
        .route("/hello", get(|| async { "hello world" }))
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    match tokio::signal::ctrl_c().await {
        Ok(_val) => tracing::event!(Level::INFO, "shutting down systems"),
        Err(error) => tracing::event!(Level::DEBUG, "{error:?}"),
    };
}
