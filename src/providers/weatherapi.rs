pub use serde::{Deserialize, Serialize};

pub fn get_day_limit() -> u32 {
    // free limit: 3
    3
}

/// Single day's forecast data.
#[derive(Serialize, Deserialize, Debug)]
pub struct ForecastDay {
    /// Date of the forecast (YYYY-MM-DD).
    pub date: String,
    /// UNIX timestamp of the date.
    pub date_epoch: i64,
    /// Daily weather summary.
    pub day: Day,
    /// Astronomical data for the day.
    pub astro: Astro,
    /// Hourly forecast data.
    pub hour: Vec<Hour>,
}

/// Weather forecast data for multiple days.
#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    /// List of daily forecasts.
    pub forecastday: Vec<ForecastDay>,
}

/// Weather data combining location and current conditions.
#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    /// Location details.
    pub location: Location,
    /// Current weather conditions.
    pub current: Current,
    /// Weather forecast data.
    pub forecast: Forecast,
}

/// Air quality data for a location, including pollutant levels and indices.
#[derive(Serialize, Deserialize, Debug)]
pub struct AirQuality {
    pub co: f64,
    pub no2: f64,
    pub o3: f64,
    pub so2: f64,
    pub pm2_5: f64,
    pub pm10: f64,
    /// US EPA Air Quality Index (1-6).
    #[serde(rename = "us-epa-index")]
    pub us_epa_index: i32,
    /// UK DEFRA Air Quality Index.
    #[serde(rename = "gb-defra-index")]
    pub gb_defra_index: i32,
}

/// Weather condition details.
#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i32,
}

/// Current weather data for a location.
#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    pub last_updated_epoch: i32,
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i32,
    pub condition: Condition,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: i32,
    pub wind_dir: String,
    pub pressure_mb: f64,
    pub pressure_in: f64,
    pub precip_mm: f64,
    pub precip_in: f64,
    pub humidity: i32,
    pub cloud: i32,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub windchill_c: f64,
    pub windchill_f: f64,
    pub heatindex_c: f64,
    pub heatindex_f: f64,
    pub dewpoint_c: f64,
    pub dewpoint_f: f64,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub uv: f64,
    pub gust_mph: f64,
    pub gust_kph: f64,
    pub air_quality: AirQuality,
}

/// Location data for weather information.
#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub tz_id: String,
    pub localtime_epoch: i32,
    pub localtime: String,
}

/// Daily weather forecast data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Day {
    pub maxtemp_c: f64,
    pub maxtemp_f: f64,
    pub mintemp_c: f64,
    pub mintemp_f: f64,
    pub avgtemp_c: f64,
    pub avgtemp_f: f64,
    pub maxwind_mph: f64,
    pub maxwind_kph: f64,
    pub totalprecip_mm: f64,
    pub totalprecip_in: f64,
    pub totalsnow_cm: f64,
    pub avgvis_km: f64,
    pub avgvis_miles: f64,
    pub avghumidity: u32,
    pub daily_will_it_rain: i32,
    pub daily_chance_of_rain: i32,
    pub daily_will_it_snow: i32,
    pub daily_chance_of_snow: i32,
    pub condition: Condition,
    pub uv: f64,
}

/// Astronomical data for a specific day.
#[derive(Serialize, Deserialize, Debug)]
pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String,
    pub moon_phase: String,
    pub moon_illumination: u32,
    pub is_moon_up: i32,
    pub is_sun_up: i32,
}

/// Hourly weather forecast data.
#[derive(Serialize, Deserialize, Debug)]
pub struct Hour {
    pub time_epoch: i64,
    pub time: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i32,
    pub condition: Condition,
    pub wind_mph: f64,
    pub wind_kph: f64,
    pub wind_degree: i32,
    pub wind_dir: String,
    pub pressure_mb: f64,
    pub pressure_in: f64,
    pub precip_mm: f64,
    pub precip_in: f64,
    pub snow_cm: f64,
    pub humidity: i32,
    pub cloud: i32,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
    pub windchill_c: f64,
    pub windchill_f: f64,
    pub heatindex_c: f64,
    pub heatindex_f: f64,
    pub dewpoint_c: f64,
    pub dewpoint_f: f64,
    pub will_it_rain: i32,
    pub chance_of_rain: i32,
    pub will_it_snow: i32,
    pub chance_of_snow: i32,
    pub vis_km: f64,
    pub vis_miles: f64,
    pub gust_kph: f64,
    pub gust_mph: f64,
    pub uv: f64,
}
