use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;

#[derive(Debug, Serialize, Deserialize)]
pub struct ForecastWrapper {
    properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
struct Properties {
    periods: Vec<WeatherPeriod>,
}

#[serde_alias(CamelCase,SnakeCase)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherPeriod {
    pub name: String,
    pub temperature: u64,
    pub wind_direction: String,
    pub wind_speed: String,
    pub detailed_forecast: String,
    pub start_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherOfficeLocation {
    pub x: i32,
    pub y: i32,
    pub code: String,
}

impl WeatherOfficeLocation {
    pub fn build_url(&self) -> String {
        format!(
            "https://api.weather.gov/gridpoints/{}/{},{}/forecast",
            self.code,
            self.x,
            self.y
        )
    }
}

// Gets the full forecast from the response.
pub fn get_full_forecast(location: WeatherOfficeLocation) -> Vec<WeatherPeriod> {
    let url = WeatherOfficeLocation::build_url(&location);
    let client = reqwest::blocking::Client::new();
    let forecast = client.get(&url).header(USER_AGENT, "My SuperAwesome Weather App").send().expect("Unable to get data").text().unwrap().to_string();
    let json: ForecastWrapper = serde_json::from_str(&forecast).expect("JSON was not well-formatted");
    let periods: Vec<WeatherPeriod> = json.properties.periods.into_iter().collect();

    periods
}

