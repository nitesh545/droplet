use axum::{Router, routing::get};
use sqlx::{Pool, Postgres};
use std::{
    env,
    sync::{Arc, Mutex},
};
use tower_http::trace::TraceLayer;
use tracing::Level;

#[allow(dead_code)]
struct AppState {
    pub db_pool: Pool<Postgres>,
    pub config: Config,
}

#[allow(dead_code)]
struct Config {
    pub database_url: String,
    pub port: String,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let database_url = env::var("DATABASE_URL").expect("No DATABASE_URL");
    let port = env::var("PORT").expect("No PORT");
    let host = env::var("URL").expect("No URL");
    let config = Config {
        database_url: database_url.clone(),
        port: port.clone(),
    };

    let pg_pool = Pool::<Postgres>::connect_lazy(&config.database_url)
        .expect("could not connect with postgresql db");
    let app_state = Arc::new(Mutex::new(AppState {
        db_pool: pg_pool,
        config,
    }));

    let app = Router::new()
        .route("/hello", get(|| async { "hello world" }))
        .with_state(app_state)
        .layer(TraceLayer::new_for_http());
    let listener = tokio::net::TcpListener::bind(&format!("{:?}/hello", host))
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
