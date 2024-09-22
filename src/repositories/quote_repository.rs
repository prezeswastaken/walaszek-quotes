use diesel::prelude::*;
use std::ops::DerefMut;
use std::sync::Arc;

use diesel::SqliteConnection;
use tokio::sync::Mutex;

use crate::models::quote::NewQuote;
use crate::models::quote::Quote;

use crate::schema::quotes::dsl::*;

pub async fn get_quote_by_id(
    db: Arc<Mutex<SqliteConnection>>,
    id_to_search: i32,
) -> Result<Quote, diesel::result::Error> {
    let mut db = db.lock().await;
    let result = quotes
        .find(id_to_search)
        .select(Quote::as_select())
        .load(db.deref_mut())?
        .get(0)
        .unwrap()
        .clone();

    Ok(result)
}

pub async fn get_all_quotes(
    db: Arc<Mutex<SqliteConnection>>,
) -> Result<Vec<Quote>, diesel::result::Error> {
    let mut db = db.lock().await;
    let result = quotes
        .select(Quote::as_select())
        .load(db.deref_mut())?
        .clone();

    Ok(result)
}

pub async fn create_quote(
    db: Arc<Mutex<SqliteConnection>>,
    quote_to_create: NewQuote,
) -> Result<Quote, diesel::result::Error> {
    let mut db = db.lock().await;
    let result = diesel::insert_into(quotes)
        .values(quote_to_create)
        .returning(Quote::as_returning())
        .get_result(db.deref_mut())?;

    Ok(result)
}
