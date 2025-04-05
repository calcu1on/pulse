use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;
use std::fs;
use iso8601;

const SOX_ID: i32 = 111;
const SOX_SCHEDULE_URL: &str = "https://statsapi.mlb.com/api/v1/schedule?sportId=1&teamId=111&startDate=2025-04-01&endDate=2025-09-30";
const SOX_SCHEDULE_LOCAL: &str = "/Users/danchadwick/Projects/rust/pulse/assets/mlb-response.json";

#[serde_alias(CamelCase,SnakeCase)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Schedule {
    pub dates: Vec<Date>, 
}

#[serde_alias(CamelCase,SnakeCase)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Date {
    pub date: String,
    pub games: Vec<Game>,
}

#[serde_alias(CamelCase,SnakeCase)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub season: String,
    pub teams: Teams,
    // pub date: String,
    pub official_date: String,
    // pub start_time: String,
    // pub opponent: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_alias(CamelCase,SnakeCase)]
pub struct Teams {
    away: Team,
    home: Team,
}

#[serde_alias(CamelCase,SnakeCase)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Team {
    team: TeamInfo,
    league_record: TeamRecord,
}

#[serde_alias(CamelCase,SnakeCase)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamInfo {
    id: i32,
    name: String,
}

#[serde_alias(CamelCase,SnakeCase)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeamRecord {
    wins: i32,
    losses: i32,
}

pub struct GameInfo {
    pub opponent: String,
    pub date: String,
    // pub time: String,
}

// Gets the full forecast from the response.
pub fn get_schedule() -> Vec<GameInfo> {
    let client = reqwest::blocking::Client::new();
    // let schedule_json: String = fs::read_to_string(SOX_SCHEDULE_LOCAL).expect("Unable to read file").to_owned();
    let schedule_json: String = client.get(SOX_SCHEDULE_URL).send().expect("Unable to get data").text().unwrap().to_string();
    let schedule: Schedule = serde_json::from_str(&schedule_json).expect("JSON was not well-formatted");
    // Iterate over the schedule, extract datapoints to create new struct
    // Return a vec of the new structs.
    let mut full_schedule: Vec<GameInfo> = vec![];
    let dates = schedule.dates;
    for date in dates {
        for game in date.games {
            let facing = extract_opponent(&game.teams);
            let game_info = GameInfo {
                opponent: facing,
                date: game.official_date,
            };
            full_schedule.push(game_info);
        }
    }
    full_schedule
}

pub fn extract_opponent(teams: &Teams) -> String {
    if teams.home.team.name == "Boston Red Sox" {
        teams.away.team.name.to_string()
    }
    else {
        teams.home.team.name.to_string()
    }
}

#[cfg(test)]
mod sox_tests {
    use super::*;

    #[test]
    fn check_schedule_retrieval() {
        get_schedule();
    }
}
