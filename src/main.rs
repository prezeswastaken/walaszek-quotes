use std::sync::Arc;

use dotenvy::dotenv;
use serde_json::json;
use tokio::sync::Mutex;
use walaszek_qutoes::{
    database::{establish_connection, get_pool},
    filters::shows_filter::shows_filter,
};
use warp::{reply::Reply, Filter};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    if let Err(e) = dotenv() {
        panic!("Failed to load .env file: {}", e);
    }

    let db = get_pool();
    let db = Arc::new(db);

    let port = std::env::var("PORT")
        .unwrap_or("8000".to_string())
        .parse::<u16>()
        .unwrap_or(8000u16);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
        .allow_headers(vec!["Content-Type", "Authorization", "Accept"]);

    let hello = warp::path("hello")
        .and(warp::get())
        .map(|| warp::reply::json(&json!({"message": "Hello, World!"})));
    let hi = warp::path("hi")
        .and(warp::get())
        .map(|| warp::reply::json(&json!({"message": "Hi"})));

    let shows_filter = shows_filter(db.clone());

    let api_base = warp::path("api");
    let api_routes = hello.or(hi).or(shows_filter);
    let api_routes = api_base.and(api_routes).with(cors);

    warp::serve(api_routes).run(([0, 0, 0, 0], port)).await;
}
