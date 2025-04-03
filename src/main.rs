#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
mod weather;
mod redsox;
use tabled::Table;
use tabled::settings::{
    peaker::Priority, Width, Style, Alignment, object::Columns
};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled)]
#[tabled(rename_all = "UPPERCASE")]
struct TableRow {
    date: String,
    time_of_day: String,
    temp: u64,
    red_sox: String,
    forecast: String,
}

#[allow(unreachable_code)]
fn main() {
    let baseball_diamond = '\u{f15ec}';
    let sunny = '\u{f0599}';
    // Set the weather location here.
    let location = weather::WeatherOfficeLocation {
        x: 75,
        y: 59,
        code: "GYX".to_string(),
    };
    // @todo - add a way to configure which teams to add
    // @todo - add a way to get news articles?
    let entire_forecast: Vec<weather::WeatherPeriod> = weather::get_full_forecast(location);
    let sox_games: Vec<redsox::Game> = redsox::get_upcoming_games();
    let mut table_rows: Vec<TableRow> = vec![];
    for i in 0..entire_forecast.len() {
        let item = &entire_forecast[i];
        let date = &item.start_time[0..10];
        let mut sox_status = String::new();
        // Check if there is a sox game and print opp.
        for sox_game in &sox_games {
            if sox_game.date == date {
                let mut opp_str = String::new();
                opp_str.push(baseball_diamond);
                opp_str.push_str(" ");
                opp_str.push_str(&sox_game.opponent);
                sox_status = opp_str;
                break;
            }
        }
        let mut forecast_w_icon = String::new();
        forecast_w_icon.push(sunny);
        forecast_w_icon.push_str(" ");
        forecast_w_icon.push_str(&item.detailed_forecast);
        let row = TableRow {
            date: date.to_string(),
            time_of_day: item.name.clone(),
            temp: item.temperature,
            red_sox: sox_status,
            forecast: forecast_w_icon,
        };
        table_rows.push(row);
    }
    let mut table = Table::new(table_rows);
    table.with(Style::modern());
    table.with((
        Width::wrap(210).priority(Priority::max(true)),
        Width::increase(50).priority(Priority::min(true)),
    ));
    table.modify(Columns::first(), Alignment::right());
    println!("{}", table);
}

