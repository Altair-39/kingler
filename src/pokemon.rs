use rust_embed::EmbeddedFile;
use serde::Deserialize;

use std::collections::HashMap;
use std::str;

use crate::error::Error;

#[derive(Debug, Deserialize)]
pub struct Pokemon {
    pub slug: String,
    pub gen: u8,
    pub name: HashMap<String, String>,
    pub desc: HashMap<String, HashMap<String, String>>,
    pub forms: Vec<String>,
    pub stats: Option<std::collections::HashMap<String, u32>>,
}

pub fn load_pokemon(pokemon_db: &EmbeddedFile) -> Result<Vec<Pokemon>, Error> {
    let pokemon_json_str = str::from_utf8(&pokemon_db.data).expect("Invalid UTF-8 in pokemon db");
    let pokemon: Vec<Pokemon> = serde_json::from_str(pokemon_json_str)?;
    Ok(pokemon)
}

pub fn list_pokemon_names(pokemon_db: Vec<Pokemon>) {
    pokemon_db.iter().for_each(|p| println!("{}", p.slug));
}
