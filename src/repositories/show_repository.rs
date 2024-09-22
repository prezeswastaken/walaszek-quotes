use diesel::prelude::*;
use std::ops::DerefMut;
use std::sync::Arc;

use diesel::SqliteConnection;
use tokio::sync::Mutex;

use crate::models::show::NewShow;
use crate::models::show::Show;

use crate::schema::shows::dsl::*;

pub async fn get_show_by_id(
    db: Arc<Mutex<SqliteConnection>>,
    id_to_search: i32,
) -> Result<Show, diesel::result::Error> {
    let mut db = db.lock().await;
    let result = shows
        .find(id_to_search)
        .select(Show::as_select())
        .load(db.deref_mut())?
        .get(0)
        .unwrap()
        .clone();

    Ok(result)
}

pub async fn get_all_shows(
    db: Arc<Mutex<SqliteConnection>>,
) -> Result<Vec<Show>, diesel::result::Error> {
    let mut db = db.lock().await;
    let result = shows
        .select(Show::as_select())
        .load(db.deref_mut())?
        .clone();

    Ok(result)
}

pub async fn create_show(
    db: Arc<Mutex<SqliteConnection>>,
    show_to_create: NewShow,
) -> Result<Show, diesel::result::Error> {
    let mut db = db.lock().await;
    let result = diesel::insert_into(shows)
        .values(show_to_create)
        .returning(Show::as_returning())
        .get_result(db.deref_mut())?;

    Ok(result)
}
