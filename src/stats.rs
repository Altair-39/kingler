use crate::Pokemon;

/// Displays the stats of a given Pokémon.
///
/// This function checks if the Pokémon has stats available. If stats are present,
/// it prints the HP, Attack, Defense, Special Attack, Special Defense, and Speed
/// in a formatted manner. If stats are not available, a message is printed
/// indicating that stats are not available for the Pokémon.
///
/// # Parameters
/// - `pokemon`: A reference to a `Pokemon` struct containing the stats to be displayed.
pub fn display_pokemon_stats(pokemon: &Pokemon) {
    if let Some(stats) = &pokemon.stats {
        let stat_pairs = [
            ("hp", "speed"),
            ("attack", "special-attack"),
            ("defense", "special-defense"),
        ];

        for &(stat1, stat2) in &stat_pairs {
            let value1 = stats.get(stat1).unwrap_or(&0);
            let value2 = stats.get(stat2).unwrap_or(&0);

            println!(
                "{:<15} {:<5}  {:<15} {}",
                format!("{}:", stat1),
                value1,
                format!("{}:", stat2),
                value2
            );
        }
    } else {
        println!("\nStats not available for this Pokémon.");
    }
}
