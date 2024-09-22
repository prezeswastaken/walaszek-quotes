use diesel::prelude::*;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use warp::Filter;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub type DB = Arc<Mutex<SqliteConnection>>;
