use crate::providers::weatherapi::WeatherData;
use url::ParseError;

/// Returns a mapping of wind directions to Unicode arrows.
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
pub enum Errors {
    InvalidInput,
    JsonParse,
    ApiKeyError,
    UrlError(ParseError),
}

impl From<ParseError> for Errors {
    fn from(v: ParseError) -> Self {
        Self::UrlError(v)
    }
}

pub fn get_wind_arrows() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("N", "⬆"),
        ("NNE", "↗"),
        ("NE", "↗"),
        ("ENE", "➡"),
        ("E", "➡"),
        ("ESE", "↘"),
        ("SE", "↘"),
        ("SSE", "⬇"),
        ("S", "⬇"),
        ("SSW", "↙"),
        ("SW", "↙"),
        ("WSW", "⬅"),
        ("W", "⬅"),
        ("WNW", "↖"),
        ("NW", "↖"),
        ("NNW", "⬆"),
    ])
}

/// Returns a mapping of US EPA Air Quality Index values to descriptions.
pub fn get_us_epa_index() -> HashMap<u8, &'static str> {
    HashMap::from([
        (1, "Good"),
        (2, "Moderate"),
        (3, "Unhealthy for sensitive group"),
        (4, "Unhealthy"),
        (5, "Very Unhealthy"),
        (6, "Hazardous"),
    ])
}

/// Reads a location query from the console.
///
/// # Returns
/// A `String` containing the user-provided location.
pub fn get_query_from_user() -> Result<String, Errors> {
    print!("Enter Location: ");
    stdout().flush().unwrap();
    let mut input_query = String::new();
    stdin()
        .read_line(&mut input_query)
        .expect("Failed to read line");
    if input_query.trim().is_empty() {
        println!("No Location is provided");
        println!("Entry city name, IP address, Latitude/Longitude (decimal degree)\nUS Zipcode, Uk Postcode, Canada Postalcode.");
        Err(Errors::InvalidInput)
    } else {
        Ok(input_query)
    }
}

/// Loads the Weather API key from the environment or a provided key.
///
/// # Arguments
/// * `user_api_key` - A `String` containing the user-provided API key, or empty to load from `.env`.
///
/// # Returns
/// A `String` containing the validated API key.
pub fn load_api_key(user_api_key: String) -> Result<String, Errors> {
    dotenv::dotenv().ok();
    let user_api_min_length: usize = 20;
    if user_api_key.trim().is_empty() {
        match std::env::var("WEATHER_API_KEY") {
            Ok(val) => Ok(val),
            Err(e) => {
                eprintln!("Error: {}", e.to_string().to_uppercase());
                Err(Errors::ApiKeyError)
            }
        }
    } else if user_api_key.len() < user_api_min_length {
        eprintln!("Invalid User API KEY (Leave empty to load from .env)");
        Err(Errors::ApiKeyError)
    } else {
        Ok(user_api_key)
    }
}

/// Fetches and parses weather data from the Weather API.
///
/// # Arguments
/// * `query` - A `String` representing the location query (e.g., city name, coordinates).
///
/// # Returns
/// A `WeatherData` struct containing the parsed weather information.
pub fn fetch_parsed_json(query: String, days: u32) -> Result<WeatherData, Errors> {
    let api_key = load_api_key("2aed558640c64add927135819250108".to_owned())?; // Provide WeatherAPI Key, or leave empty to load form .env
    let aqi: String = "yes".to_owned();
    let url: String = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={query}&days={days}&aqi={aqi}"
    );
    let url: reqwest::Url = reqwest::Url::parse(&url)?;
    let response: reqwest::blocking::Response =
        reqwest::blocking::get(url).expect("Failed to fetch weather data");
    if response.status() != 200 {
        println!(
            "Failed to fetch weather data, status code {}",
            response.status()
        );
        return Err(Errors::JsonParse);
    }
    let json_body = response.text().expect("Failed to get Json Data");
    let json_parsed: WeatherData =
        serde_json::from_str(&json_body).expect("Failed to parse Json to the structs");
    Ok(json_parsed)
}
