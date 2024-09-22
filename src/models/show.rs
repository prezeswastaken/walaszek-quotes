use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::shows)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Show {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable, Clone, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::shows)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewShow {
    pub name: String,
}
