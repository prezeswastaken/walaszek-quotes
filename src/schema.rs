// @generated automatically by Diesel CLI.

diesel::table! {
    characters (id) {
        id -> Integer,
        name -> Text,
        show_id -> Integer,
    }
}

diesel::table! {
    quotes (id) {
        id -> Integer,
        text -> Text,
        character_id -> Integer,
    }
}

diesel::table! {
    shows (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(characters -> shows (show_id));
diesel::joinable!(quotes -> characters (character_id));

diesel::allow_tables_to_appear_in_same_query!(characters, quotes, shows,);
