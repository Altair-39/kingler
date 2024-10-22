use crate::Config;
use crate::Pokemon;

use rand::prelude::SliceRandom;

pub fn get_random_description<'a>(pokemon: &'a Pokemon, config: &'a Config) -> Vec<&'a str> {
    if let Some(descriptions) = pokemon.desc.get(&config.language) {
        let game_keys: Vec<&String> = descriptions.keys().collect();
        if let Some(random_game) = game_keys.choose(&mut rand::thread_rng()) {
            if let Some(desc) = descriptions.get(*random_game) {
                return desc.lines().collect(); // Return lines from the selected description.
            }
        }
    }
    Vec::new() // Return an empty vector if no descriptions are found.
}
