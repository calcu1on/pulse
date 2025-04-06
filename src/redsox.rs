use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;
use chrono::{DateTime, Utc};
use chrono_tz::US::Eastern;

const TEAM_ID: i32 = 111;

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
    pub official_date: String,
    pub game_date: String,
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
    pub start_time: String,
}

// Gets the full forecast from the response.
pub fn get_schedule() -> Vec<GameInfo> {
    let client = reqwest::blocking::Client::new();
    let schedule_url = build_api_url();
    let schedule_json: String = client.get(&schedule_url).send().expect("Unable to get data").text().unwrap().to_string();
    let schedule: Schedule = serde_json::from_str(&schedule_json).expect("JSON was not well-formatted");
    let mut full_schedule: Vec<GameInfo> = vec![];
    let dates = schedule.dates;
    for date in dates {
        for game in date.games {
            let facing = extract_opponent(&game.teams);
            let game_info = GameInfo {
                opponent: facing,
                date: game.official_date,
                start_time: get_start_time(&game.game_date),
            };
            full_schedule.push(game_info);
        }
    }
    full_schedule
}

// Determine who the opponent is from the teams.
pub fn extract_opponent(teams: &Teams) -> String {
    if teams.home.team.name == "Boston Red Sox" {
        teams.away.team.name.to_string()
    }
    else {
        teams.home.team.name.to_string()
    }
}

// Build the url for the api request.
fn build_api_url() -> String {
    let url_first: String = "https://statsapi.mlb.com/api/v1/schedule?sportId=1&teamId=".to_string();
    let url_second: String= "&startDate=2025-04-01&endDate=2025-09-30".to_string();
    format!("{}{}{}", url_first, TEAM_ID, url_second)
}

fn get_start_time(iso_string: &String) -> String {
    let utc_dt: DateTime<Utc> = iso_string.parse().expect("Invalid ISO8601 string");
    let est_dt = utc_dt.with_timezone(&Eastern);
    est_dt.format("%I:%M").to_string()
}

#[cfg(test)]
mod team_tests {
    use super::*;
    #[test]
    fn check_schedule_retrieval() {
        get_schedule();
    }
}
