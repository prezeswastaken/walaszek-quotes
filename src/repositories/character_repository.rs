use anyhow::Error;
use diesel::insert_into;
use diesel::SqliteConnection;
use diesel::prelude::*;

use crate::database::DbPool;
use crate::errors::errors::AppError;
use anyhow::Result;

use crate::models::character::Character;
use crate::models::character::NewCharacter;
use crate::schema::characters::dsl::*;


pub async fn get_character_by_id(db: DbPool, id_to_search: i32) -> Result<Character, AppError> {
    let conn = db.get().await?;

    let character = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Character, Error>(characters
                .find(id_to_search)
                .select(Character::as_select())
                .load(conn)?
                .get(0)
                .unwrap()
                .clone())
        })
        .await
        .map_err(|_| AppError::NotFound)??;

    Ok(character)
}

pub async fn get_all_characters(db: DbPool, page: i64, per_page: i64) -> Result<Vec<Character>, AppError> {
    let conn = db.get().await?;

    let offset = (page - 1) * per_page;

    let characters_vec = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Vec<Character>, Error>(characters
                .select(Character::as_select())
                .limit(per_page)
                .offset(offset)
                .load(conn)?
                .clone())
        })
        .await
        .map_err(|_| AppError::InternalServerError)??;

    Ok(characters_vec)
}

pub async fn create_character(pool: DbPool, character_to_create: NewCharacter) -> Result<Character, AppError> {
    let conn = pool.get().await?;

    let character: Character = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Character, Error>(insert_into(characters)
                .values(character_to_create)
                .returning(Character::as_returning())
                .get_result(conn)?)
        })
        .await
        .map_err(|_| AppError::InternalServerError)??;

    Ok(character)

}

pub async fn seed(pool: DbPool, count: u64) -> Result<usize, AppError> {
    let conn = pool.get().await?;

    let characters_vec: Vec<NewCharacter> = (0..count)
        .map(|i| NewCharacter {
            name: format!("Character {}", i),
            show_id: 1,
        })
        .collect();

        let count = conn.interact(move |conn: &mut SqliteConnection| {
            insert_into(characters)
                .values(characters_vec)
                .execute(conn)
        }).await.map_err(|_| AppError::InternalServerError)??;

    Ok(count)
}

pub async fn count(pool: DbPool) -> Result<i64, AppError> {
    let conn = pool.get().await?;

    let count = conn.interact(move |conn: &mut SqliteConnection| {
        characters.count().get_result(conn)
    }).await.map_err(|_| AppError::InternalServerError)??;

    Ok(count)
}
