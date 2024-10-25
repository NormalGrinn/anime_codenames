use diesel::{prelude::Queryable, Selectable};

use crate::codename_game::types;

#[derive(Queryable)]
#[diesel(table_name = crate::schema::games)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Game {
    pub channel_id: i32,
    pub players: types::PlayerInfo,
    pub board: Vec<types::Card>,
    pub game: types::GameInfo,
}
