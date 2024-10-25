use std::io;
use thiserror::Error;

/// An enumeration representing errors that can occur in the application.
#[derive(Error, Debug)]
pub enum Error {
    /// Represents a configuration error with a descriptive message.
    #[error("Configuration error: {0}")]
    Configuration(String),

    /// Indicates failure in loading the Pokémon database, encapsulating the underlying JSON error.
    #[error("Failed to load pokemon db: {0}")]
    PokemonDb(#[from] serde_json::Error),

    /// Signifies that an invalid Pokémon name was provided.
    #[error("Invalid pokemon `{0}`")]
    InvalidPokemon(String),

    /// Represents an error due to an invalid language code.
    #[error("Invalid language `{0}`, should be one of [en, fr, de, it, es, ko, ja, ja_hrkt, zh_hans, zh_hant]")]
    InvalidLanguage(String),

    /// Indicates that the specified generations are invalid.
    #[error("Invalid generations `{0}`, should be integers between 1 and 9")]
    InvalidGeneration(String),

    /// Indicates an IO error occurred.
    #[error("I/O error: {0}")]
    IoError(String), // Here you could keep it as a String to handle the error message
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error::IoError(error.to_string()) // Convert io::Error to String
    }
}
