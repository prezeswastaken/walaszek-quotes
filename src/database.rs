use diesel::prelude::*;
use std::{env, sync::Arc};

use deadpool_diesel::sqlite::{Manager, Pool, Runtime};

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type DbPool = Arc<Pool>;
pub type RouterType = axum::Router<std::sync::Arc<deadpool_diesel::Pool<deadpool_diesel::Manager<diesel::prelude::SqliteConnection>>>>;

pub fn get_pool() -> Pool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = Manager::new(db_url, Runtime::Tokio1);
    let pool = Pool::builder(manager)
        .max_size(8)
        .build()
        .expect("Failed to create db pool.");
    pool
}
