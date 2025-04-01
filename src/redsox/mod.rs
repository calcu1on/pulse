use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    date: String,
    start_time: String,
    opponent: String,
}

pub fn get_upcoming_games() -> Vec<Game> {
    let schedule_json: String = fs::read_to_string("./assets/sox-schedule.json").expect("Unable to read file").to_owned();
    let json: Vec<Game> = serde_json::from_str(&schedule_json).expect("Something not good?");
    let upcoming_games: &Vec<Game> = &json.into_iter().take(7).collect();
    upcoming_games.to_owned()
}
