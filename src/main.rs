mod providers;
mod utils;

use crate::providers::weatherapi::{self, WeatherData};

/// Main function to run the weather application.
fn main() -> Result<(), utils::Errors> {
    let user_query: String;
    let args: Vec<String> = std::env::args().collect();
    let days: u32 = weatherapi::get_day_limit(); // Free limit: 3

    if args.len() > 2 {
        println!("Invalid argument!, Use \"\" quotations if location have whitespace.");
        return Ok(());
    } else if args.len() == 2 && !args[1].trim().is_empty() {
        user_query = args[1].clone();
    } else {
        user_query = utils::get_query_from_user()?;
    }
    let weather: WeatherData = utils::fetch_parsed_json(user_query, days)?;

    let dash_line: String = "-".repeat(80);
    println!("<>{}<>", dash_line);

    print_location(&weather);
    print_current_weather(&weather);
    print_current_air_quality(&weather);
    print_forcast(weather);

    println!("<>{}<>", dash_line);
    Ok(())
}

fn print_location(weather: &WeatherData) {
    println!(
        "{} ({}, {})\nLocal Time: {}\n",
        weather.location.name,
        weather.location.region,
        weather.location.country,
        weather.location.localtime,
    );
}

fn print_current_weather(weather: &WeatherData) {
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
        utils::get_wind_arrows().get(wind_dir).unwrap_or(&"❓"),
        weather.current.wind_kph,
        weather.current.wind_mph,
        weather.current.dewpoint_c,
        weather.current.dewpoint_f
    );
}
fn print_current_air_quality(weather: &WeatherData) {
    println!(
        "AQI: {}\tPM2.5: {:.1} μg/m³\tPM10: {:.1} μg/m³",
        utils::get_us_epa_index()
            .get(&(weather.current.air_quality.us_epa_index as u8))
            .unwrap_or(&"Unknown"),
        weather.current.air_quality.pm2_5,
        weather.current.air_quality.pm10,
    );
}

fn print_forcast(weather: WeatherData) {
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
}
