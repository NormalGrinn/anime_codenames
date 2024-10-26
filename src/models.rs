use diesel::Insertable;
use diesel::{prelude::Queryable, Selectable};
use diesel::sqlite::Sqlite;

use crate::codename_game::types;
use crate::schema::games;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub channel_id: u64,
    pub players: String,
    pub board: String,
    pub game: String,
}

#[derive(Insertable)]
#[diesel(table_name = games)]
pub struct NewGame {
    pub channel_id: i64,
}

#[derive(Insertable)]
#[diesel(table_name = games)]
pub struct NewPlayer {
    pub players: String,
}