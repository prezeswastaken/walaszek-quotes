use crate::models::character::Character;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Associations, Identifiable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::quotes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Character))]
pub struct Quote {
    pub id: i32,
    pub text: String,
    pub character_id: i32,
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::quotes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewQuote {
    pub text: String,
    pub character_id: i32,
}
