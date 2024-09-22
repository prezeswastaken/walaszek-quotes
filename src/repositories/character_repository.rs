use diesel::prelude::*;
use std::ops::DerefMut;
use std::sync::Arc;

use diesel::SqliteConnection;
use tokio::sync::Mutex;

use crate::models::character::Character;
use crate::models::character::NewCharacter;
use crate::models::show::NewShow;
use crate::models::show::Show;

use crate::schema::characters::dsl::*;

pub async fn get_character_by_id(
    db: Arc<Mutex<SqliteConnection>>,
    id_to_search: i32,
) -> Result<Character, diesel::result::Error> {
    let mut db = db.lock().await;
    let result = characters
        .find(id_to_search)
        .select(Character::as_select())
        .load(db.deref_mut())?
        .get(0)
        .unwrap()
        .clone();

    Ok(result)
}

pub async fn get_all_characters(
    db: Arc<Mutex<SqliteConnection>>,
) -> Result<Vec<Character>, diesel::result::Error> {
    let mut db = db.lock().await;
    let result = characters
        .select(Character::as_select())
        .load(db.deref_mut())?
        .clone();

    Ok(result)
}

pub async fn create_character(
    db: Arc<Mutex<SqliteConnection>>,
    character_to_create: NewCharacter,
) -> Result<Character, diesel::result::Error> {
    let mut db = db.lock().await;
    let result = diesel::insert_into(characters)
        .values(character_to_create)
        .returning(Character::as_returning())
        .get_result(db.deref_mut())?;

    Ok(result)
}
