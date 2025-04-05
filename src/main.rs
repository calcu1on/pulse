#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
mod weather;
mod redsox;
// use colored::Colorize;
use tabled::{Table, Tabled};
use serde::{Deserialize, Serialize};
use tabled::settings::{
    peaker::Priority, Width, Style, Alignment, object::Columns
};

#[derive(Serialize, Deserialize, Debug, Tabled)]
#[tabled(rename_all = "UPPERCASE")]
struct TableRow {
    date: String,
    time_of_day: String,
    temp: i32,
    red_sox: String,
    forecast: String,
}

#[allow(unreachable_code)]
fn main() {
    // Set the weather location here.
    let location = weather::WeatherOfficeLocation {
        x: 75,
        y: 59,
        code: "GYX".to_string(),
    };
    let entire_forecast: Vec<weather::WeatherPeriod> = weather::get_full_forecast(location);
    let sox_games: Vec<redsox::GameInfo> = redsox::get_schedule();
    let baseball_diamond = '\u{f0852}';
    let mut table_rows: Vec<TableRow> = vec![];
    for i in 0..entire_forecast.len() {
        let forecast_period = &entire_forecast[i];
        let yyyy_mm_dd = &forecast_period.start_time[0..10];
        let mut sox_status = String::new();
        // Check if there is a sox game and print opp.
        for sox_game in &sox_games {
            if sox_game.date == yyyy_mm_dd {
                let mut opp_str = String::from(baseball_diamond);
                opp_str.push_str(" ");
                opp_str.push_str(&sox_game.opponent);
                sox_status = opp_str;
                break;
            }
        }
        let row = TableRow {
            date: yyyy_mm_dd.to_string(),
            time_of_day: forecast_period.name.clone(),
            temp: forecast_period.temperature,
            red_sox: sox_status,
            forecast: forecast_period.detailed_forecast.to_string(),
        };
        table_rows.push(row);
    }
    render_table(&table_rows);
}

fn render_table(rows: &Vec<TableRow>) {
    // here is where we actually render the table.
    let mut table = Table::new(rows);
    table.with(Style::modern());
    table.with((
            Width::wrap(210).priority(Priority::max(true)),
            Width::increase(50).priority(Priority::min(true)),
    ));
    table.modify(Columns::first(), Alignment::right());
    println!("{}", table);   
}

