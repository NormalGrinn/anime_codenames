use diesel::prelude::AsChangeset;
use diesel::Insertable;
use diesel::{prelude::Queryable, Selectable};
use diesel::sqlite::Sqlite;

use crate::codename_game::types;
use crate::schema::{games, clue_info};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub channel_id: i64,
    pub players: String,
    pub board: String,
    pub game: String,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::clue_info)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ClueInfo {
    pub clue_id: i32,
    pub channel_id: i64,
    pub clue_type: String,
    pub clue_body: String,
}

#[derive(Insertable)]
#[diesel(table_name = games)]
pub struct NewGame {
    pub channel_id: i64,
    pub players: String,
}

#[derive(Insertable, Clone, Debug, AsChangeset)]
#[diesel(table_name = games)]
pub struct NewPlayer {
    pub players: String,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = clue_info)]
pub struct NewClue {
    pub channel_id: i64,
    pub clue_type: String,
    pub clue_body: String,
}