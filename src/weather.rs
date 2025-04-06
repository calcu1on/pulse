use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;
#[path = "icons.rs"] mod icons;
use icons::Icons;

#[derive(Debug, Serialize, Deserialize)]
struct ForecastWrapper {
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
    pub temperature: i32,
    pub wind_direction: String,
    pub wind_speed: String,
    pub detailed_forecast: String,
    pub short_forecast: String,
    pub start_time: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherOfficeLocation {
    pub x: i32,
    pub y: i32,
    pub code: String,
}

impl WeatherOfficeLocation {
    // Build api request URL.
    pub fn build_url(&self) -> String {
        format!(
            "https://api.weather.gov/gridpoints/{}/{},{}/forecast",
            self.code,
            self.x,
            self.y,
        )
    }
    
    // Get the full forecast for the location.
    pub fn get_full_forecast(&self) -> Vec<WeatherPeriod> {
        let url = WeatherOfficeLocation::build_url(&self);
        let client = reqwest::blocking::Client::new();
        let forecast = client.get(&url)
            .header(USER_AGENT, "My SuperAwesome Weather App")
            .send()
            .expect("Unable to get data")
            .text().unwrap().to_string();
        let ForecastWrapper { 
            properties: Properties { 
                mut periods 
            } 
        } = serde_json::from_str(&forecast).expect("JSON was not well-formatted");
        for period in periods.iter_mut() {
            match detect_icon(&period.short_forecast) {
                None => println!("There was an issue detecting the correct icon!!!"),
                Some(icon) => {
                    period.detailed_forecast = format!("{} {}", icon, &period.detailed_forecast);
                }
            };
        }
        periods
    }
}

// Detect which icon to display based on short forecast.
pub fn detect_icon(short_forecast: &str) -> Option<String> {
    match true {
        _ if short_forecast.contains("Sunny") => Some(Icons::Sunny.get_icon_str()),
        _ if short_forecast.contains("Rain") && short_forecast.contains("Snow") => {
            Some(Icons::Mixed.get_icon_str())
        }
        _ if short_forecast.contains("Snow") => Some(Icons::Snow.get_icon_str()),
        _ if short_forecast.contains("Rain") => Some(Icons::Rain.get_icon_str()),
        _ if short_forecast.contains("Cloudy") => Some(Icons::Cloudy.get_icon_str()),
        _ if short_forecast.contains("Clear") => Some(Icons::Clear.get_icon_str()),
        _ => None,
    }
}
