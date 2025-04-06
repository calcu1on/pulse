mod weather;
mod redsox;
mod icons;
use icons::Icons;
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
    temp: String,
    red_sox: String,
    forecast: String,
}

fn main() {
    // Get forecast & schedule.
    let entire_forecast: Vec<weather::WeatherPeriod> = weather::WeatherOfficeLocation {
        x: 75,
        y: 59,
        code: "GYX".to_string(),
    }.get_full_forecast();
    let sox_games: Vec<redsox::GameInfo> = redsox::get_schedule();
    // Build icons.
    let baseball_icon = Icons::Baseball.get_icon_str();
    let clock_icon = Icons::Clock.get_icon_str();
    let fahrenheight_icon = Icons::Fahrenheight.get_icon_str();
    // Build the rows for the table.
    let mut table_rows: Vec<TableRow> = vec![];
    for i in 0..entire_forecast.len() {
        let forecast_period = &entire_forecast[i];
        let yyyy_mm_dd = &forecast_period.start_time[0..10];
        let mut sox_status = String::new();
        // Check if there is a sox game and print opp.
        for sox_game in &sox_games {
            if sox_game.date == yyyy_mm_dd {
                sox_status = format!("{} {}\n{} {}", &baseball_icon, &sox_game.opponent, &clock_icon, &sox_game.start_time);
                break;
            }
        }
        let row = TableRow {
            date: yyyy_mm_dd.to_string(),
            time_of_day: forecast_period.name.clone(),
            temp: format!("{}{}", forecast_period.temperature, fahrenheight_icon),
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
    table.with((Style::modern(), Alignment::center()));
    table.modify(Columns::last(), Alignment::left());
    table.with((
            Width::wrap(200).priority(Priority::max(true)),
            Width::increase(60).priority(Priority::min(true)),
    ));
    table.modify(Columns::single(3), Alignment::left());
    println!("{}", table);   
}

