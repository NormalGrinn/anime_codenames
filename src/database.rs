extern crate diesel;
extern crate dotenv;

use diesel::{prelude::*, result::Error};
use dotenv::dotenv;
use std::env;

use crate::{codename_game::types, models::NewGame, schema::games};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL has to be set");

    let res = SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    res
}

pub fn create_game(channel_id: i64) -> Result<usize, Error> {
    let mut conn = establish_connection();
    let newgame = NewGame {
        channel_id: channel_id
    };
    let res = diesel::insert_into(games::table)
        .values(&newgame)
        .execute(&mut conn);
    res
}

pub fn add_player(channel_id: i64, player_id: u64, player_name: String, player_team: types::Team) -> Result<usize, Error> {
    let mut conn = establish_connection();
    let newplayer = types::PlayerInfo {
        player_id: player_id,
        player_name: player_name,
        player_team: player_team,
        player_role: None,
    };
    let player_json = serde_json::to_string(&newplayer).expect("Error serializing player");
    
    todo!()
}