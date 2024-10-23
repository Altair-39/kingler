use crate::Config;
use crate::Pokemon;
use rand::prelude::SliceRandom;

/// Retrieves a random description for a given Pokémon based on the configured language.
///
/// This function selects a random game from the available descriptions in the
/// specified language and returns the lines of that description.
///
/// # Parameters
/// - `pokemon`: A reference to a `Pokemon` instance containing possible descriptions.
/// - `config`: A reference to a `Config` instance that includes the preferred language.
///
/// # Returns
/// - `Vec<&str>`: A vector containing the lines of the selected description if found,
///   or an empty vector if no descriptions are available in the specified language.
pub fn get_random_description<'a>(pokemon: &'a Pokemon, config: &'a Config) -> Vec<&'a str> {
    // Attempt to get the descriptions for the specified language from the Pokémon.
    if let Some(descriptions) = pokemon.desc.get(&config.language) {
        // Collect the keys (game names) from the descriptions.
        let game_keys: Vec<&String> = descriptions.keys().collect();

        // Randomly choose a game from the available keys.
        if let Some(random_game) = game_keys.choose(&mut rand::thread_rng()) {
            // Retrieve and return the lines of the chosen description.
            if let Some(desc) = descriptions.get(*random_game) {
                return desc.lines().collect(); // Return lines from the selected description.
            }
        }
    }
    Vec::new() // Return an empty vector if no descriptions are found.
}
