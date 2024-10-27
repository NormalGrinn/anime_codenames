extern crate diesel;
extern crate dotenv;

use diesel::{prelude::*, result::Error};
use dotenv::dotenv;
use std::env;

use crate::codename_game::anilist_api;
use crate::codename_game::types::PlayerInfo;
use crate::models::{NewClue, NewPlayer};
use crate::schema::clue_info::*;
use crate::schema::games::dsl::*;
use crate::{codename_game::types, models::NewGame, schema::{clue_info, games}};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL has to be set");

    let res = SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
    res
}

pub fn create_game(id: i64) -> Result<usize, Error> {
    let mut conn = establish_connection();
    let empty_players: Vec<types::PlayerInfo> = Vec::new();
    let empty_players_json: String = serde_json::to_string(&empty_players).expect("Error serialzing players");
    let newgame = NewGame {
        channel_id: id,
        players: empty_players_json,
    };
    let res = diesel::insert_into(games::table)
        .values(&newgame)
        .execute(&mut conn);
    res
}

pub fn add_player(id: i64, player_id: u64, player_name: &String, player_team: types::Team) -> Result<usize, Error> {
    let mut conn = establish_connection();
    let newplayer = types::PlayerInfo {
        player_id: player_id,
        player_name: player_name.to_string(),
        player_team: player_team,
        player_role: None,
    };
    let player_json = serde_json::to_string(&newplayer).expect("Error serializing player");
    let res = games.select(games::players)
    .filter(games::channel_id.eq(id))
    .load::<String>(&mut conn)?;
    let mut current_players: Vec<types::PlayerInfo> = serde_json::from_str(&res[0]).expect("Error deserialzing players");
    PlayerInfo::update_or_insert(&mut current_players, newplayer);
    let updated_players = serde_json::to_string(&current_players).expect("Error serializing players");
    let new_players = NewPlayer {
        players: updated_players,
    };
    let res = diesel::update(games::table.filter(games::channel_id.eq(id)))
    .set(&new_players)
    .execute(&mut conn)?;
    
    Ok(res)
}

pub async fn add_anime_clue(id: i64, username: &str) -> Result<usize, Error>{
    let anime_names = serde_json::to_string(&anilist_api::get_list::get_anime_names(username).await).expect("Error serialzing anime names");
    let clue_type_string = serde_json::to_string(&types::ClueType::AnimeNames).expect("Error serialzing clue type");
    let mut conn = establish_connection();
    let newclue = NewClue {
        channel_id: id,
        clue_type: clue_type_string,
        clue_body: anime_names,
    };
    let res = diesel::insert_into(clue_info::table)
    .values(newclue)
    .execute(&mut conn);
    res
}