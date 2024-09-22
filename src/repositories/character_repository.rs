use anyhow::Error;
use diesel::insert_into;
use diesel::SqliteConnection;
use diesel::prelude::*;

use crate::database::DB;
use anyhow::Result;

use crate::models::character::Character;
use crate::models::character::NewCharacter;
use crate::schema::characters::dsl::*;


pub async fn get_character_by_id(db: DB, id_to_search: i32) -> Result<Character> {
    let conn = db.get().await?;

    let result = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok(characters
                .find(id_to_search)
                .select(Character::as_select())
                .load(conn)?
                .get(0)
                .unwrap()
                .clone())
        })
        .await
        .map_err(|_| anyhow::anyhow!("Error getting character by id"))?;

    result
}

pub async fn get_all_characters(db: DB) -> Result<Vec<Character>> {
    let conn = db.get().await?;

    let result = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok(characters
                .select(Character::as_select())
                .load(conn)?
                .clone())
        })
        .await
        .map_err(|_| anyhow::anyhow!("Error getting all characters"))?;

    result
}

pub async fn create_character(pool: DB, character_to_create: NewCharacter) -> Result<Character> {
    let conn = pool.get().await?;

    let character: Character = conn
        .interact(move |conn: &mut SqliteConnection| {
            Ok::<Character, Error>(insert_into(characters)
                .values(character_to_create)
                .returning(Character::as_returning())
                .get_result(conn)?)
        })
        .await
        .map_err(|_| anyhow::anyhow!("Error creating character"))??;

    Ok(character)

}
