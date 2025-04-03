use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use serde_alias::serde_alias;
#[path = "nerdfont.rs"] mod nerdfont;

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
    pub temperature: u64,
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
    pub fn build_url(&self) -> String {
        format!(
            "https://api.weather.gov/gridpoints/{}/{},{}/forecast",
            self.code,
            self.x,
            self.y,
        )
    }
}

// Gets the full forecast from the response.
pub fn get_full_forecast(location: WeatherOfficeLocation) -> Vec<WeatherPeriod> {
    let url = WeatherOfficeLocation::build_url(&location);
    let client = reqwest::blocking::Client::new();
    let forecast = client.get(&url)
        .header(USER_AGENT, "My SuperAwesome Weather App")
        .send()
        .expect("Unable to get data")
        .text().unwrap().to_string();
    let json: ForecastWrapper = serde_json::from_str(&forecast).expect("JSON was not well-formatted");
    let weather_periods: Vec<WeatherPeriod> = json.properties.periods.into_iter().collect();
    let icon_forecasts = enhance_forecasts(&weather_periods); 

    icon_forecasts
}

pub fn enhance_forecasts(periods: &Vec<WeatherPeriod>) -> Vec<WeatherPeriod> {
    let mut rebuilt_periods: Vec<WeatherPeriod> = vec![];
    for period in periods {
        let icon = detect_icon(&period.short_forecast).unwrap();
        let icon_forecast = format!("{} {}", icon, &period.detailed_forecast);
        let rebuilt_period = WeatherPeriod {
            name: period.name.to_string(),
            temperature: period.temperature,
            wind_direction: period.wind_direction.to_string(),
            wind_speed: period.wind_speed.to_string(),
            detailed_forecast: icon_forecast,
            short_forecast: period.short_forecast.to_string(),
            start_time: period.start_time.to_string(),     
        };
        rebuilt_periods.push(rebuilt_period);
    }

    rebuilt_periods
}

pub fn detect_icon(short_forecast: &String) -> Option<char> {
    if short_forecast.contains("Sunny") {
        let icon = nerdfont::NerdFontIcon {
            icon_code: "f0599".to_string(),
        };
        let icon_code = icon.get_icon();
        icon_code
    } 
    else if short_forecast.contains("Rain") && short_forecast.contains("Snow") {
        let icon = nerdfont::NerdFontIcon {
            icon_code: "f067f".to_string(),
        };
        let icon_code = icon.get_icon();
        icon_code
    }
    else if short_forecast.contains("Snow") {
        let icon = nerdfont::NerdFontIcon {
            icon_code: "f0f36".to_string(),
        };
        let icon_code = icon.get_icon();
        icon_code
    }
    else if short_forecast.contains("Rain") {
        let icon = nerdfont::NerdFontIcon {
            icon_code: "e239".to_string(),
        };
        let icon_code = icon.get_icon();
        icon_code
    }
    else if short_forecast.contains("Cloudy") {
        let icon = nerdfont::NerdFontIcon {
            icon_code: "e312".to_string(),
        };
        let icon_code = icon.get_icon();
        icon_code
    }
    else {
        None
    }
}


