use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

/// Air quality data for a location, including pollutant levels and indices.
#[derive(Serialize, Deserialize, Debug)]
struct AirQuality {
    co: f64,
    no2: f64,
    o3: f64,
    so2: f64,
    pm2_5: f64,
    pm10: f64,
    /// US EPA Air Quality Index (1-6).
    #[serde(rename = "us-epa-index")]
    us_epa_index: i32,
    /// UK DEFRA Air Quality Index.
    #[serde(rename = "gb-defra-index")]
    gb_defra_index: i32,
}

/// Weather condition details.
#[derive(Serialize, Deserialize, Debug)]
struct Condition {
    text: String,
    icon: String,
    code: i32,
}

/// Current weather data for a location.
#[derive(Serialize, Deserialize, Debug)]
struct Current {
    last_updated_epoch: i32,
    last_updated: String,
    temp_c: f64,
    temp_f: f64,
    is_day: i32,
    condition: Condition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: i32,
    wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    humidity: i32,
    cloud: i32,
    feelslike_c: f64,
    feelslike_f: f64,
    windchill_c: f64,
    windchill_f: f64,
    heatindex_c: f64,
    heatindex_f: f64,
    dewpoint_c: f64,
    dewpoint_f: f64,
    vis_km: f64,
    vis_miles: f64,
    uv: f64,
    gust_mph: f64,
    gust_kph: f64,
    air_quality: AirQuality,
    short_rad: f64,
    diff_rad: f64,
    dni: f64,
    gti: f64,
}

/// Location data for weather information.
#[derive(Serialize, Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: i32,
    localtime: String,
}

/// Daily weather forecast data.
#[derive(Serialize, Deserialize, Debug)]
struct Day {
    maxtemp_c: f64,
    maxtemp_f: f64,
    mintemp_c: f64,
    mintemp_f: f64,
    avgtemp_c: f64,
    avgtemp_f: f64,
    maxwind_mph: f64,
    maxwind_kph: f64,
    totalprecip_mm: f64,
    totalprecip_in: f64,
    totalsnow_cm: f64,
    avgvis_km: f64,
    avgvis_miles: f64,
    avghumidity: u32,
    daily_will_it_rain: i32,
    daily_chance_of_rain: i32,
    daily_will_it_snow: i32,
    daily_chance_of_snow: i32,
    condition: Condition,
    uv: f64,
    air_quality: AirQuality,
}

/// Astronomical data for a specific day.
#[derive(Serialize, Deserialize, Debug)]
struct Astro {
    sunrise: String,
    sunset: String,
    moonrise: String,
    moonset: String,
    moon_phase: String,
    moon_illumination: u32,
    is_moon_up: i32,
    is_sun_up: i32,
}

/// Hourly weather forecast data.
#[derive(Serialize, Deserialize, Debug)]
struct Hour {
    time_epoch: i64,
    time: String,
    temp_c: f64,
    temp_f: f64,
    is_day: i32,
    condition: Condition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: i32,
    wind_dir: String,
    pressure_mb: f64,
    pressure_in: f64,
    precip_mm: f64,
    precip_in: f64,
    snow_cm: f64,
    humidity: i32,
    cloud: i32,
    feelslike_c: f64,
    feelslike_f: f64,
    windchill_c: f64,
    windchill_f: f64,
    heatindex_c: f64,
    heatindex_f: f64,
    dewpoint_c: f64,
    dewpoint_f: f64,
    will_it_rain: i32,
    chance_of_rain: i32,
    will_it_snow: i32,
    chance_of_snow: i32,
    vis_km: f64,
    vis_miles: f64,
    gust_kph: f64,
    gust_mph: f64,
    uv: f64,
    air_quality: AirQuality,
    short_rad: f64,
    diff_rad: f64,
    dni: f64,
    gti: f64,
}

/// Single day's forecast data.
#[derive(Serialize, Deserialize, Debug)]
struct ForecastDay {
    /// Date of the forecast (YYYY-MM-DD).
    date: String,
    /// UNIX timestamp of the date.
    date_epoch: i64,
    /// Daily weather summary.
    day: Day,
    /// Astronomical data for the day.
    astro: Astro,
    /// Hourly forecast data.
    hour: Vec<Hour>,
}

/// Weather forecast data for multiple days.
#[derive(Serialize, Deserialize, Debug)]
struct Forecast {
    /// List of daily forecasts.
    forecastday: Vec<ForecastDay>,
}

/// Weather data combining location and current conditions.
#[derive(Serialize, Deserialize, Debug)]
struct WeatherData {
    /// Location details.
    location: Location,
    /// Current weather conditions.
    current: Current,
    /// Weather forecast data.
    forecast: Forecast,
}

/// Returns a mapping of wind directions to Unicode arrows.
fn get_wind_arrows() -> HashMap<&'static str, &'static str> {
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
fn get_us_epa_index() -> HashMap<u8, &'static str> {
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
fn get_query_from_user() -> String {
    print!("Enter Location: ");
    stdout().flush().unwrap();
    let mut input_query = String::new();
    stdin()
        .read_line(&mut input_query)
        .expect("Failed to read line");
    if input_query.trim().is_empty() {
        println!("No Location is provided");
        println!("Entry city name, IP address, Latitude/Longitude (decimal degree)\nUS Zipcode, Uk Postcode, Canada Postalcode.");
        std::process::exit(0);
    } else {
        input_query
    }
}

/// Loads the Weather API key from the environment or a provided key.
///
/// # Arguments
/// * `user_api_key` - A `String` containing the user-provided API key, or empty to load from `.env`.
///
/// # Returns
/// A `String` containing the validated API key.
fn load_api_key(user_api_key: String) -> String {
    dotenv::dotenv().ok();
    let user_api_min_length: usize = 20;
    if user_api_key.trim().is_empty() {
        match std::env::var("WEATHER_API_KEY") {
            Ok(val) => val,
            Err(e) => {
                eprintln!("Error: {}", e.to_string().to_uppercase());
                std::process::exit(0);
            }
        }
    } else if user_api_key.len() < user_api_min_length {
        eprintln!("Invalid User API KEY (Leave empty to load from .env)");
        std::process::exit(0);
    } else {
        user_api_key
    }
}

/// Fetches and parses weather data from the Weather API.
///
/// # Arguments
/// * `query` - A `String` representing the location query (e.g., city name, coordinates).
///
/// # Returns
/// A `WeatherData` struct containing the parsed weather information.
fn fetch_parsed_json(query: String, days: u32) -> WeatherData {
    let api_key = load_api_key("2aed558640c64add927135819250108".to_owned()); // Provide WeatherAPI Key, or leave empty to load form .env
    let aqi: String = "yes".to_owned();
    let url: String = format!(
        "https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={query}&days={days}&aqi={aqi}"
    );
    let url: reqwest::Url = reqwest::Url::parse(&url).unwrap();
    let response: reqwest::blocking::Response =
        reqwest::blocking::get(url).expect("Failed to fetch weather data");
    if response.status() != 200 {
        println!(
            "Failed to fetch weather data, status code {}",
            response.status()
        );
        std::process::exit(0);
    }
    let json_body = response.text().unwrap();
    let json_parsed: WeatherData =
        serde_json::from_str(&json_body).expect("Failed to parse Json to the structs");
    json_parsed
}

/// Main function to run the weather application.
fn main() {
    let query: String;
    let args: Vec<String> = std::env::args().collect();
    let days: u32 = 3; // Free limit: 3
    if args.len() > 2 {
        println!("Invalid argument!, Use \"\" quotations if location have whitespace.");
        std::process::exit(0);
    } else if args.len() == 2 && !args[1].trim().is_empty() {
        query = args[1].clone();
    } else {
        query = get_query_from_user();
    }
    let weather: WeatherData = fetch_parsed_json(query, days);

    println!("<>{}<>", "-".repeat(70));
    println!(
        "{} ({}, {})\nLocal Time: {}\n",
        weather.location.name,
        weather.location.region,
        weather.location.country,
        weather.location.localtime,
    );

    println!(
        "{} | {}°C / {}°F\tUV: {}\n",
        weather.current.condition.text,
        weather.current.temp_c,
        weather.current.temp_f,
        weather.current.uv
    );

    println!(
        "Feels like: {}°C / {}°F\tHumidity: {}%\tPrecip: {} mm",
        weather.current.feelslike_c,
        weather.current.feelslike_f,
        weather.current.humidity,
        weather.current.precip_mm
    );

    let wind_dir: &str = weather.current.wind_dir.as_str();
    println!(
        "Wind: {} {}kph / {}mph \tDew Point: {}°C / {}°F",
        get_wind_arrows().get(wind_dir).unwrap_or(&"❓"),
        weather.current.wind_kph,
        weather.current.wind_mph,
        weather.current.dewpoint_c,
        weather.current.dewpoint_f
    );

    println!(
        "AQI: {}\tPM2.5: {:.1} μg/m³\tPM10: {:.1} μg/m³",
        get_us_epa_index()
            .get(&(weather.current.air_quality.us_epa_index as u8))
            .unwrap_or(&"Unknown"),
        weather.current.air_quality.pm2_5,
        weather.current.air_quality.pm10,
    );

    println!("\n▶ Forecast:");
    for forecast_day in weather.forecast.forecastday {
        println!(
            "  - {}: {}°C / {}°F, {} (Precip: {} mm, UV: {})",
            forecast_day.date,
            forecast_day.day.maxtemp_c,
            forecast_day.day.maxtemp_f,
            forecast_day.day.condition.text,
            forecast_day.day.totalprecip_mm,
            forecast_day.day.uv
        );
    }
    println!("<>{}<>", "-".repeat(70));
}
