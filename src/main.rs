use std::sync::Arc;

use axum::Router;
use dotenvy::dotenv;
use walaszek_quotes::{
    database::get_pool, routers::{characters_router::get_characters_router, quotes_router::get_quotes_router, shows_router::get_shows_router},
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    if let Err(e) = dotenv() {
        panic!("Failed to load .env file: {}", e);
    }

    let pool = get_pool();
    let pool = Arc::new(pool);

    let port = std::env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000u16);

    let router = Router::new()
        .nest("/api/shows", get_shows_router())
        .nest("/api/characters", get_characters_router())
        .nest("/api/quotes", get_quotes_router())
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await.expect("failed to bind address");
        axum::serve(listener, router).await.expect("server failed");
}
