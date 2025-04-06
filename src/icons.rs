#[allow(dead_code)]
pub enum Icons {
    Fahrenheight,
    Clock,
    Baseball,
    Sunny,
    Mixed,
    Rain,
    Snow,
    Clear,
    Cloudy,
}

impl Icons {

    pub fn get_icon_str(&self) -> String {
        match self {
            Icons::Fahrenheight => Self::get_icon("e341").unwrap().to_string(),
            Icons::Clock => Self::get_icon("e641").unwrap().to_string(),
            Icons::Baseball => Self::get_icon("f0852").unwrap().to_string(),
            Icons::Sunny => Self::get_icon("f0599").unwrap().to_string(),
            Icons::Mixed => Self::get_icon("f067f").unwrap().to_string(),
            Icons::Rain => Self::get_icon("e239").unwrap().to_string(),
            Icons::Snow => Self::get_icon("f0f36").unwrap().to_string(),
            Icons::Clear => Self::get_icon("e30d").unwrap().to_string(),
            Icons::Cloudy => Self::get_icon("e312").unwrap().to_string(),
        }
    }

    fn get_icon(code: &str) -> Option<char> {
        u32::from_str_radix(&code, 16).ok().and_then(char::from_u32)
    }
}

