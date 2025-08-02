# mosm-rs, a simple weather CLI

A Rust-based command-line interface (CLI) application for fetching and displaying weather data, including current conditions, air quality, and a 3-day forecast, using the [WeatherAPI](https://www.weatherapi.com/).

## Features

- **Current Weather**: Displays temperature (Celsius and Fahrenheit), weather condition, UV index, humidity, precipitation, wind speed/direction, and dew point.
- **Air Quality**: Shows US EPA Air Quality Index, PM2.5, and PM10 levels.
- **3-Day Forecast**: Provides daily max/min temperatures, weather conditions, precipitation, and UV index.
- **Interactive Input**: Supports location input via command-line arguments or interactive prompt.
- **Error Handling**: Gracefully handles invalid API keys, empty inputs, and API errors.

## Prerequisites

- **Rust**: Install Rust and Cargo from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **WeatherAPI Key**: Sign up at [WeatherAPI](https://www.weatherapi.com/) to obtain a free API key (Not needed if using [binary](https://github.com/ferozeren/mosm-rs/releases/tag/0.1.0) instead).
- **Dependencies**: The project uses `reqwest`, `serde`, `serde_json`, and `dotenv` crates.

## Installation

1. **Clone the Repository**:
   ```bash
   git clone https://github.com/ferozeren/mosm-rs.git
   cd mosm-rs
   ```

2. **Set Up Environment**:
  
   Create a `.env` file in the project root with your WeatherAPI key:
   ```bash
   echo "WEATHER_API_KEY=your_api_key_here" > .env
   ```
   Replace `your_api_key_here` with your actual WeatherAPI key.
   Or
   Provide your WeatherAPI key in src/main.rs
   ```rust
   let api_key: String = load_api_key("YOUR_API_KEY".to_owned()); // Leave "" empty to load from .env file.   
   ```

4. **Add Dependencies**:
   Ensure your `Cargo.toml` includes:
   ```toml
   [dependencies]
   reqwest = { version = "0.11", features = ["blocking", "json"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   dotenv = "0.15"
   ```

5. **Build and run the Project**:
   Build Mode:
   ```bash
   cargo run
   ```
   Release Mode:
   ```bash
   cargo run --release
   ```
7. **Install to the ~/.cargo/bin/ (Your Rust's PATH) **:
   ```bash
   cargo install --path .
   ```
## Usage

Run the application using one of the following methods:

- **With Command-Line Argument** (use quotes for locations with spaces):
  ```bash
  $ mosm-py Tokyo
  ```

- **Interactive Mode** (prompts for location input):
  ```bash
  $ cargo run
  $ Enter Location: Paris
  ```
  Enter a location (e.g., city name, IP address, latitude/longitude, US zip code, UK postcode, or Canada postal code).

### Example Output

For `cargo run --release  "London, UK"`:
```
<>----------------------------------------------------------------------<>
London (City of London, Greater London, United Kingdom)
Local Time: 2025-08-02 16:51

Partly Cloudy | 21.2°C / 70.2°F UV: 3.5

Feels like: 21.2°C / 70.2°F     Humidity: 43%   Precip: 0 mm
Wind: ↖ 11.2kph / 6.9mph        Dew Point: 5.8°C / 42.4°F
AQI: Good       PM2.5: 9.4 μg/m³        PM10: 12.8 μg/m³

▶ Forecast:
  - 2025-08-02: 23.1°C / 73.7°F, Partly Cloudy  (Precip: 0 mm, UV: 1.7)
  - 2025-08-03: 24.8°C / 76.7°F, Patchy rain nearby (Precip: 1.83 mm, UV: 1.3)
  - 2025-08-04: 23.6°C / 74.5°F, Patchy rain nearby (Precip: 1.86 mm, UV: 1.6)
<>----------------------------------------------------------------------<>
```

## Environment Variables

- `WEATHER_API_KEY`: Your WeatherAPI key. If not provided in `.env`, you can pass it as an argument (minimum 20 characters), but this is not recommended for security reasons.

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes and commit (`git commit -m "Add your feature"`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a Pull Request.

Please ensure your code follows Rust best practices and includes tests where applicable.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [WeatherAPI](https://www.weatherapi.com/) for providing the weather data API.
- The Rust community for excellent libraries like `reqwest` and `serde`.
