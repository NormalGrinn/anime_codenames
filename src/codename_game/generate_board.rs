use crate::database;

use super::types;
use rand::{seq::SliceRandom, thread_rng, Rng};

pub fn generate_board(id: i64) -> Result<Vec<types::Card>, diesel::result::Error> {
    let db_clues = database::get_clue_info(id).expect("Error getting clues");
    let mut clues: Vec<types::Card> = Vec::new();
    let mut clue_names: Vec<String> = Vec::new();
    for clue in db_clues {
        let clue_type: types::ClueType = serde_json::from_str(&clue.clue_type).expect("Error deserializing clue type");
        match clue_type {
            types::ClueType::AnimeNames => {
                let mut clue_body: Vec<String> = serde_json::from_str(&clue.clue_body).expect("Error deserialzing clue body");
                clue_names.append(&mut clue_body);
            },
        }
    }
    let mut colours: Vec<types::Colour> = vec![types::Colour::Red; 8];
    colours.extend(vec![types::Colour::Blue; 8]);
    colours.extend(vec![types::Colour::Black; 1]);
    colours.extend(vec![types::Colour::Gray; 8]);

    let mut rng = thread_rng();
    colours.shuffle(&mut rng);

    for i in 0..25 {
        let index = rng.gen_range(0..clue_names.len());
        let name = clue_names.remove(index);
        let new_card = types::Card {
            location: i,
            names: name,
            colour: colours[i as usize],
            exposed: false,
            selected: None,
        };
        clues.push(new_card);
    }
    Ok(clues)
}