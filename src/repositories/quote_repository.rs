use anyhow::Error;
use diesel::insert_into;
use diesel::SqliteConnection;
use diesel::prelude::*;

use crate::database::DbPool;
use crate::errors::errors::AppError;
use crate::models::character::Character;
use crate::schema;
use anyhow::Result;

use crate::models::quote::Quote;
use crate::models::quote::NewQuote;
use crate::schema::quotes::dsl::*;


pub async fn get_quote_by_id(db: DbPool, id_to_search: i32) -> Result<Quote, AppError> {
    let conn = db.get().await?;

    let quote = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Quote, Error>(quotes
                .find(id_to_search)
                .select(Quote::as_select())
                .load(conn)?
                .get(0)
                .unwrap()
                .clone())
        })
        .await
        .map_err(|_| AppError::NotFound)??;

    Ok(quote)
}

pub async fn get_all_quotes(db: DbPool, page: i64, per_page: i64) -> Result<Vec<(Quote, Character)>, AppError> {
    let conn = db.get().await?;

    let offset = (page - 1) * per_page;

    let quotes_vec = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Vec<(Quote, Character)>, Error>(quotes
                .inner_join(schema::characters::table)
                .select((Quote::as_select(), Character::as_select()))
                .limit(per_page)
                .offset(offset)
                .load(conn)?
                .clone())
        })
        .await
        .map_err(|_| AppError::InternalServerError)??;

    Ok(quotes_vec)
}

pub async fn create_quote(pool: DbPool, quote_to_create: NewQuote) -> Result<Quote, AppError> {
    let conn = pool.get().await?;

    let quote: Quote = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Quote, Error>(insert_into(quotes)
                .values(quote_to_create)
                .returning(Quote::as_returning())
                .get_result(conn)?)
        })
        .await
        .map_err(|_| AppError::InternalServerError)??;

    Ok(quote)

}

pub async fn seed(pool: DbPool, count: u64) -> Result<usize, AppError> {
    let conn = pool.get().await?;

    let quotes_vec: Vec<NewQuote> = (0..count)
        .map(|i| NewQuote {
            text: format!("Quote {}", i),
            character_id: 1,
        })
        .collect();

        let count = conn.interact(move |conn: &mut SqliteConnection| {
            insert_into(quotes)
                .values(quotes_vec)
                .execute(conn)
        }).await.map_err(|_| AppError::InternalServerError)??;

    Ok(count)
}

pub async fn count(pool: DbPool) -> Result<i64, AppError> {
    let conn = pool.get().await?;

    let count = conn.interact(move |conn: &mut SqliteConnection| {
        quotes.count().get_result(conn)
    }).await.map_err(|_| AppError::InternalServerError)??;

    Ok(count)
}
