use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;
use std::fs;

const SOX_ID: i32 = 111;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_alias(CamelCase,SnakeCase)]
pub struct Schedule {
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_alias(CamelCase,SnakeCase)]
pub struct Game {
    // pub teams: Teams,
    pub date: String,
    // pub start_time: String,
    pub opponent: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_alias(CamelCase,SnakeCase)]
pub struct Teams {
    away: Team,
    home: Team,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_alias(CamelCase,SnakeCase)]
pub struct Team {
    team: TeamInfo,
    league_record: TeamRecord,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_alias(CamelCase,SnakeCase)]
pub struct TeamInfo {
    id: i32,
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_alias(CamelCase,SnakeCase)]
pub struct TeamRecord {
    wins: i32,
    losses: i32,
}

pub fn get_upcoming_games() -> Vec<Game> {
    // @todo - change this to be a dynamic request from the API endpoint instead of a local file.
    let schedule_json: String = fs::read_to_string("/Users/danchadwick/Projects/rust/weather/assets/sox-schedule.json").expect("Unable to read file").to_owned();
    let json: Vec<Game> = serde_json::from_str(&schedule_json).expect("Something not good?");
    let upcoming_games: &Vec<Game> = &json.into_iter().take(7).collect();
    upcoming_games.to_owned()
}

// Gets the full forecast from the response.
pub fn get_schedule() -> Schedule {
    let url = "https://statsapi.mlb.com/api/v1/schedule?sportId=1&teamId=111&startDate=2025-04-01&endDate=2025-09-30".to_string();
    let client = reqwest::blocking::Client::new();
    let schedule_string: String = client.get(&url).send().expect("Unable to get data").text().unwrap().to_string();
    let json: Schedule = serde_json::from_str(&schedule_string).expect("JSON was not well-formatted");

    dbg!(json);
    let schedule = Schedule {
        games: vec![],
    };

    schedule
    // json
}
