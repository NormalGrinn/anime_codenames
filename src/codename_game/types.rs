use diesel::{prelude::Queryable, sql_types::Bool};
use strum_macros::{EnumString, Display};
use serde::{Serialize, Deserialize};
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use std::io::Write;
use std::borrow::Cow;

#[derive(Debug, Deserialize, Serialize)]
pub enum ClueType {
    AnimeNames
}

#[derive(EnumString, Display, Debug, Serialize, Deserialize, Clone)]
#[strum(serialize_all = "snake_case")] 
pub enum Team {
    Red,
    Blue
}

#[derive(Debug, Clone, Copy)]
pub enum Colour {
    Red,
    Blue,
    Gray,
    Black,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    Player,
    Spymaster
}

#[derive(Debug)]
pub enum Phase {
    Join,
    Guessing,
}

#[derive(Queryable, Debug)]
pub struct Name {
    english: String,
    romaji: String,
}

#[derive(Queryable, Debug)]
pub struct Anime {
    anime_id: u64,
    anime_names: Name,
}

#[derive(Queryable, Debug)]
pub struct Card {
    pub location: i32,
    pub names: String,
    pub colour: Colour,
    pub exposed: bool,
    pub selected: Option<Team>,
}

#[derive(Serialize, Deserialize, Queryable, Debug, Clone)]
pub struct PlayerInfo {
    pub player_id: u64,
    pub player_name: String,
    pub player_team: Team,
    pub player_role: Option<Role>,
}

#[derive(Queryable, Debug)]
pub struct GameInfo {
    current_team: Team,
    current_role: Role,
    current_phase: Phase,
    current_selection: Vec<Card>,
    round_guesses_left: i32,
    blue_guesses_left: i32,
    red_guesses_left: i32,
}

impl PlayerInfo {
    pub fn update_or_insert(players: &mut Vec<PlayerInfo>, new_player: PlayerInfo) {
        if players.is_empty() {
            players.push(new_player);
            return;
        }
        match players.iter_mut().find(|p| p.player_id == new_player.player_id) {
            Some(existing_player) => {
                *existing_player = new_player;
            }
            None => {
                players.push(new_player);
            }
        }
    }
}