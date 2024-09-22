use anyhow::Error;
use diesel::insert_into;
use diesel::SqliteConnection;
use diesel::prelude::*;

use crate::database::DbPool;
use crate::errors::errors::AppError;
use anyhow::Result;

use crate::models::show::Show;
use crate::models::show::NewShow;
use crate::schema::shows::dsl::*;


pub async fn get_show_by_id(db: DbPool, id_to_search: i32) -> Result<Show, AppError> {
    let conn = db.get().await?;

    let show = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Show, Error>(shows
                .find(id_to_search)
                .select(Show::as_select())
                .load(conn)?
                .get(0)
                .unwrap()
                .clone())
        })
        .await
        .map_err(|_| AppError::NotFound)??;

    Ok(show)
}

pub async fn get_all_shows(db: DbPool, page: i64, per_page: i64) -> Result<Vec<Show>, AppError> {
    let conn = db.get().await?;

    let offset = (page - 1) * per_page;

    let shows_vec = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Vec<Show>, Error>(shows
                .select(Show::as_select())
                .limit(per_page)
                .offset(offset)
                .load(conn)?
                .clone())
        })
        .await
        .map_err(|_| AppError::InternalServerError)??;

    Ok(shows_vec)
}

pub async fn create_show(pool: DbPool, show_to_create: NewShow) -> Result<Show, AppError> {
    let conn = pool.get().await?;

    let show: Show = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Show, Error>(insert_into(shows)
                .values(show_to_create)
                .returning(Show::as_returning())
                .get_result(conn)?)
        })
        .await
        .map_err(|_| AppError::InternalServerError)??;

    Ok(show)

}

pub async fn seed(pool: DbPool, count: u64) -> Result<usize, AppError> {
    let conn = pool.get().await?;

    let shows_vec: Vec<NewShow> = (0..count)
        .map(|i| NewShow {
            name: format!("Show {}", i),
        })
        .collect();

        let count = conn.interact(move |conn: &mut SqliteConnection| {
            insert_into(shows)
                .values(shows_vec)
                .execute(conn)
        }).await.map_err(|_| AppError::InternalServerError)??;

    Ok(count)
}

pub async fn count(pool: DbPool) -> Result<i64, AppError> {
    let conn = pool.get().await?;

    let count = conn.interact(move |conn: &mut SqliteConnection| {
        shows.count().get_result(conn)
    }).await.map_err(|_| AppError::InternalServerError)??;

    Ok(count)
}
