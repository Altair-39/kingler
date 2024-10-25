use serde::{Deserialize, Serialize};

use dirs::home_dir;

use std::env;
use std::fs;
use std::io::ErrorKind::NotFound;

use crate::error::Error;

const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

/// Represents the configuration settings for the Pokémon application.
#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Language used when printing Pokémon names and descriptions.
    pub language: String,

    /// The probability of displaying a shiny Pokémon with the random command.
    pub shiny_rate: f64,

    /// The path to the file with the shiny catched
    pub shiny_log_path: String,
}

/// Provides default values for the configuration settings.
impl Default for Config {
    fn default() -> Self {
        // Construct the default path for the shiny log file
        let shiny_log_path = match home_dir() {
            Some(mut path) => {
                path.push(".config"); // Ensure you are in the config directory
                fs::create_dir_all(&path).expect("Failed to create config directory"); // Ensure the directory exists
                path.push("kingler"); // Add your application-specific directory
                fs::create_dir_all(&path).expect("Failed to create kingler directory"); // Ensure this directory exists
                path.push("shiny_log.json"); // Set the filename for the log
                path.to_str()
                    .expect("Failed to convert path to string")
                    .to_string() // Convert PathBuf to String
            }
            None => "shiny_log.json".to_string(), // Fallback if home directory cannot be determined
        };

        Self {
            language: "en".to_string(), // Default language is English.
            shiny_rate: 3.0 / 4096.0,   // Default shiny rate is 1 in 128.
            shiny_log_path,             // Use the constructed path
        }
    }
}

impl Config {
    /// Loads the configuration from a `config.toml` file.
    ///
    /// If the configuration file does not exist, a default configuration file
    /// is created in the application's config directory.
    ///
    /// # Returns
    /// - `Ok(Config)`: The loaded configuration.
    /// - `Err(Error)`: An error if the configuration could not be loaded or created.
    pub fn load() -> Result<Self, Error> {
        // Get the configuration directory path.
        let config_dir = match dirs::config_dir() {
            Some(dir) => dir.join(BINARY_NAME), // Join with the binary name to get the config path.
            _none => {
                return Err(Error::Configuration(
                    "Failed to get config directory".to_string(),
                ));
            }
        };

        // Define the path to the config file.
        let config_file = config_dir.join("config.toml");

        // Try to read the config file.
        let config = match fs::read_to_string(&config_file) {
            Ok(c) => {
                // Parse the contents of the config file as TOML.
                toml::from_str(&c).expect("Failed to parse TOML in configuration file")
            }

            // Handle case where the config file does not exist.
            Err(ref e) if e.kind() == NotFound => {
                let config = Config::default(); // Create a default configuration.
                let toml =
                    toml::to_string_pretty(&config).expect("Failed to convert config to TOML");

                // Create the config directory if it does not exist.
                fs::create_dir_all(config_dir).expect("Failed to create config directory");

                // Write the default configuration to the config file.
                fs::write(&config_file, toml).expect("Failed to write config file");
                config
            }

            // Handle any other errors encountered while reading the file.
            Err(_) => {
                return Err(Error::Configuration(
                    "Failed to load configuration file".to_string(),
                ));
            }
        };

        Ok(config) // Return the loaded or default configuration.
    }
}
