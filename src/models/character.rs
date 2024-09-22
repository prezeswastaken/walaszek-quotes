use crate::models::show::Show;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Associations, Identifiable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Show))]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub show_id: i32,
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::characters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Show))]
pub struct NewCharacter {
    pub name: String,
    pub show_id: i32,
}
