# mosm-rs: A Simple Weather CLI

A Rust-based command-line interface (CLI) application for fetching and displaying weather data, including current conditions, air quality, and a 3-day forecast, using the [WeatherAPI](https://www.weatherapi.com/). Built with Rust for performance and safety, this CLI provides a lightweight way to check weather information.

## Features

- **Current Weather**: Displays temperature (Celsius and Fahrenheit), weather condition, UV index, humidity, precipitation, wind speed/direction, and dew point.
- **Air Quality**: Shows US EPA Air Quality Index, PM2.5, and PM10 levels.
- **3-Day Forecast**: Provides daily max/min temperatures, weather conditions, precipitation, and UV index.
- **Interactive Input**: Supports location input via command-line arguments or an interactive prompt.
- **Flexibility**: Easily customize output by modifying the Rust code, with predefined data templates for console display.

## Prerequisites

- **Rust**: Install Rust and Cargo from [rust-lang.org](https://www.rust-lang.org/tools/install).
- **WeatherAPI Key**: Sign up at [WeatherAPI](https://www.weatherapi.com/) to obtain a free API key. Not required if using the precompiled binary ([v0.1.0](https://github.com/ferozeren/mosm-rs/releases/tag/0.1.0)).
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

   Alternatively, provide your WeatherAPI key in `src/main.rs`:
   ```rust
   let api_key: String = load_api_key("YOUR_API_KEY".to_owned()); // Set your API key or leave empty to load from .env
   ```

3. **Add Dependencies**:
   Ensure your `Cargo.toml` includes:
   ```toml
   [dependencies]
   reqwest = { version = "0.11", features = ["blocking", "json"] }
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   dotenv = "0.15"
   ```

4. **Build and Run the Project**:
   ```bash
   cargo run
   ```
   For optimized performance:
   ```bash
   cargo run --release
   ```

5. **Install Globally**:
   Install to `~/.cargo/bin` (Rust’s PATH) for global access:
   ```bash
   cargo install --path .
   ```

## Usage

Run the application using one of the following methods:

- **With Command-Line Argument** (use quotes for locations with spaces):
  ```bash
  mosm-rs "London, UK"
  ```

- **Interactive Mode**:
  ```bash
  mosm-rs
  ```
  When prompted, enter a location (e.g., city name, IP address, latitude/longitude, US zip code, UK postcode, or Canada postal code):
  ```
  Enter Location: Paris
  ```

### Example Output

#### Location: New York
<img width="894" height="471" alt="New_York" src="https://github.com/user-attachments/assets/7e6f3975-7e88-4136-819e-4b1fc10a1221" />

#### Location: Tokyo
<img width="894" height="469" alt="Tokyo" src="https://github.com/user-attachments/assets/1dd5cd18-848d-4a61-879e-89b79a19163c" />




## Environment Variables

- `WEATHER_API_KEY`: Your WeatherAPI key. Store it in `.env` for security. Hardcoding the key in `main.rs` is not recommended. However, in a secure, private environment, hardcoding is acceptable. The program first checks for the `WEATHER_API_KEY` in `.env`. If not found, it uses the hardcoded key in `main.rs`. If neither is provided, the installed binary will exit with an error: `ERROR: NO_ENV_FOUND`.

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/your-feature`).
3. Make your changes and commit (`git commit -m "Add your feature"`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a Pull Request.

Please ensure your code follows Rust best practices, including running `cargo fmt` and `cargo clippy`, and includes tests where applicable.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [WeatherAPI](https://www.weatherapi.com/) for providing the weather data API.
- The Rust community for excellent libraries like `reqwest` and `serde`.
