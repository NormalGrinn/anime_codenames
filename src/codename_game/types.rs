use diesel::{prelude::Queryable, sql_types::Bool};

#[derive(Debug)]
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

#[derive(Debug)]
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
    anime_id: i32,
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

#[derive(Queryable, Debug)]
pub struct PlayerInfo {
    player_id: i32,
    player_name: String,
    player_list: Option<Vec<String>>,
    player_team: Option<Team>,
    player_role: Option<Role>,
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

// game:
// current_team colour,
// current_role role,
// guesses_left int,
// current_phase phase