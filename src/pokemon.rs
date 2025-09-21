use rust_embed::EmbeddedFile;
use serde::Deserialize;

use std::collections::HashMap;
use std::str;

use crate::error::Error;

/// Represents a Pokémon with various attributes including its slug, generation,
/// names in different languages, descriptions, forms, and stats.
///
/// # Fields
/// - `slug`: A unique identifier for the Pokémon, often used in URLs and APIs.
/// - `gen`: The generation of the Pokémon, represented as an unsigned 8-bit integer.
/// - `name`: A hashmap containing the Pokémon's names in various languages,
///   where the key is the language code (e.g., "en" for English).
/// - `desc`: A nested hashmap containing descriptions of the Pokémon for various games
///   and languages. The outer key is the language code, and the inner key is the game
///   name with the description as the value.
/// - `forms`: A vector of strings representing the different forms the Pokémon can take (e.g.,
///   regular, mega, etc.).
/// - `stats`: An optional hashmap that contains various stats of the Pokémon, where
///   the key is the stat name (e.g., "attack") and the value is the stat value.
#[derive(Clone, Debug, Deserialize)]
pub struct Pokemon {
    pub slug: String,
    pub gen: u8,
    pub name: HashMap<String, String>,
    pub desc: HashMap<String, HashMap<String, String>>,
    pub stats: Option<HashMap<String, u32>>,
}

/// Loads a list of Pokémon from an embedded JSON file.
///
/// # Parameters
/// - `pokemon_db`: A reference to an `EmbeddedFile` containing the Pokémon data in JSON format.
///
/// # Returns
/// - `Result<Vec<Pokemon>, Error>`: Returns a vector of `Pokemon` if the loading is successful,
///   or an `Error` if there is an issue parsing the data.
pub fn load_pokemon(pokemon_db: &EmbeddedFile) -> Result<Vec<Pokemon>, Error> {
    let pokemon_json_str = str::from_utf8(&pokemon_db.data).expect("Invalid UTF-8 in pokemon db");
    let pokemon: Vec<Pokemon> = serde_json::from_str(pokemon_json_str)?;
    Ok(pokemon)
}

/// Lists the slugs of all Pokémon in the provided database.
///
/// # Parameters
/// - `pokemon_db`: A vector of `Pokemon` objects from which to list the names.
///
/// This function prints each Pokémon's slug to the standard output.
pub fn list_pokemon_names(pokemon_db: Vec<Pokemon>) {
    pokemon_db.iter().for_each(|p| println!("{}", p.slug));
}
