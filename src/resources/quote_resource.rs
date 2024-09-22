use serde::Serialize;

use crate::models::{character::Character, quote::Quote};

#[derive(Serialize)]
pub struct QuoteResource {
    pub id: i32,
    pub text: String,
    #[serde(rename = "characterName")]
    pub character_name: String,
    #[serde(rename = "characterId")]
    pub character_id: i32,
}

impl QuoteResource {
    pub fn new(quote: Quote, character: Character) -> Self {
        Self {
            id: quote.id,
            text: quote.text,
            character_name: character.name,
            character_id: character.id,
        }
    }
}
