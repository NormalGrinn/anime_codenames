use diesel::{prelude::Queryable, sql_types::Bool};
use strum_macros::{EnumString, Display};
use serde::{Serialize, Deserialize};
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, ToSql, Output};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use std::io::Write;

#[derive(EnumString, Display, Debug, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")] 
pub enum Team {
    Red,
    Blue
}

#[derive(Debug)]
pub enum Colour {
    Red,
    Blue,
    Gray,
    Black,
}

#[derive(Debug, Serialize, Deserialize)]
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
    location: i32,
    names: Vec<String>,
    colour: Colour,
    exposed: Bool,
    selected: Bool,
}

#[derive(Serialize, Deserialize, Queryable, Debug)]
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