use thiserror::Error;

/// An enumeration representing errors that can occur in the application.
#[derive(Error)]
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
    /// The valid options are: [en, fr, de, it, es, ko, ja, ja_hrkt, zh_hans, zh_hant].
    #[error("Invalid language `{0}`, should be one of [en, fr, de, it, es, ko, ja, ja_hrkt, zh_hans, zh_hant]")]
    InvalidLanguage(String),

    /// Indicates that the specified generations are invalid.
    /// The valid generations are integers between 1 and 9.
    #[error("Invalid generations `{0}`, should be integers between 1 and 9")]
    InvalidGeneration(String),
}

impl std::fmt::Debug for Error {
    /// Custom debug implementation to format the error as a string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
