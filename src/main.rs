mod ascii;
mod cli;
mod config;
mod description;
mod error;
mod pokemon;
mod shiny_hunting;
mod stats;

use config::Config;
use error::Error;
use pokemon::*;

use clap::Parser;
use clap_complete::Shell;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use rust_embed::RustEmbed;
use serde::Deserialize;
use serde::Serialize;

use std::fs;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::str;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct EncounteredPokemon {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EncounteredPokemonTracker {
    encounters: Vec<EncounteredPokemon>,
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

fn display_shiny_log(log_path: &str) -> Result<(), Error> {
    let log_entries = shiny_hunting::load_shiny_log(log_path)?;

    for entry in log_entries {
        println!(
            "{}: {} {} - {}",
            entry.date, entry.pokemon_name, entry.form, entry.details
        );
    }

    Ok(())
}

fn track_encounter(tracker_path: &str, pokemon_name: &str, unique: bool) -> Result<(), Error> {
    // Load existing encounters
    let mut tracker = if let Ok(file_content) = std::fs::read_to_string(tracker_path) {
        serde_json::from_str::<EncounteredPokemonTracker>(&file_content)
            .unwrap_or(EncounteredPokemonTracker { encounters: vec![] })
    } else {
        EncounteredPokemonTracker { encounters: vec![] }
    };

    // Prepare the new encounter
    let new_encounter = EncounteredPokemon {
        name: pokemon_name.to_string(),
    };

    // Check if the Pokémon has already been encountered
    if !tracker
        .encounters
        .iter()
        .any(|e| e.name == new_encounter.name)
    {
        // Add the new encounter to the tracker
        tracker.encounters.push(new_encounter);

        // Save the updated tracker back to the file
        let json = serde_json::to_string(&tracker)?;
        std::fs::write(tracker_path, json)?;
    } else if unique {
        println!("{} has already been encountered.", pokemon_name);
    }
    Ok(())
}

fn show_completion_status(tracker_path: &str, total_pokemon: usize) -> Result<(), Error> {
    // Load existing encounters
    let tracker = if let Ok(file_content) = std::fs::read_to_string(tracker_path) {
        serde_json::from_str::<EncounteredPokemonTracker>(&file_content)
            .unwrap_or(EncounteredPokemonTracker { encounters: vec![] })
    } else {
        EncounteredPokemonTracker { encounters: vec![] }
    };

    let unique_count = tracker.encounters.len();

    // Calculate the percentage of the Pokédex completion
    let completion_percentage = if total_pokemon > 0 {
        (unique_count as f64 / total_pokemon as f64) * 100.0
    } else {
        0.0
    };

    println!("You have encountered {} unique Pokémon.", unique_count);
    println!(
        "Pokedex completion: {:.2}% ({} out of {})",
        completion_percentage, unique_count, total_pokemon
    );

    Ok(())
}
/// Shows a random Pokémon based on user-defined criteria such as generation range, forms, and shiny status.
///
/// This function filters the Pokémon database according to the specified generation range
/// and other preferences provided by the user. It randomly selects a Pokémon that meets
/// these criteria and prepares its representation, including potential shiny variants and
/// form variations. Finally, it calls another function to display the chosen Pokémon's information.
///
/// # Parameters
/// - `random`: A reference to the `cli::Random` struct containing user preferences for random Pokémon selection.
/// - `pokemon_db`: A vector of `Pokemon` objects representing the entire Pokémon database.
/// - `config`: A reference to the `Config` struct containing configuration settings such as shiny rate.
///
/// # Returns
/// - `Result<(), Error>`: Returns an `Ok(())` if successful, or an `Error` if any issues occur
///   during the filtering or selection process.
fn show_random_pokemon(
    random: &cli::Random,
    pokemon_db: Vec<Pokemon>,
    config: &Config,
) -> Result<(), Error> {
    // Determine generation range
    let (start_gen, end_gen) = match random.generations.split_once('-') {
        Some((start, end)) => (start, end),
        None => {
            let gen_list = random.generations.split(',').collect::<Vec<_>>();
            let gen = gen_list.choose(&mut rand::thread_rng()).unwrap_or(&"1");
            (*gen, *gen) // Dereference to convert to (&str, &str)
        }
    };

    // Parse start and end generations
    let start_gen = start_gen
        .parse::<u8>()
        .map_err(|_| Error::InvalidGeneration(random.generations.clone()))?;
    let end_gen = end_gen
        .parse::<u8>()
        .map_err(|_| Error::InvalidGeneration(random.generations.clone()))?;

    // Filter Pokémon by generation
    let pokemon: Vec<&Pokemon> = pokemon_db
        .iter()
        .filter(|p| start_gen <= p.gen && end_gen >= p.gen)
        .collect();

    // Check if there are any Pokémon available after filtering
    let selected_pokemon = match pokemon.choose(&mut rand::thread_rng()) {
        Some(&p) => p,
        None => return Err(Error::InvalidGeneration(random.generations.clone())),
    };

    // After determining the selected Pokémon
    let selected_pokemon_name = match selected_pokemon.name.get(&config.language) {
        Some(name) => name.clone(),
        None => return Err(Error::InvalidLanguage(config.language.clone())),
    };

    let unique = random.unique;
    let pokedex_path = get_pokedex_path()?;
    // Track the encounter
    track_encounter(
        pokedex_path
            .to_str()
            .expect("Failed to convert PathBuf to str"),
        &selected_pokemon_name,
        unique,
    )?;
    // Prepare forms to choose from
    let mut forms = vec!["regular".to_string()];
    forms.extend(selected_pokemon.forms.iter().cloned());

    // Apply optional filters
    if random.no_mega {
        forms.retain(|f| !["mega", "mega-x", "mega-y"].contains(&f.as_str()));
    }
    if random.no_gmax {
        forms.retain(|f| f.as_str() != "gmax");
    }
    if random.no_regional {
        forms.retain(|f| !["alola", "galar", "hisui", "paldea"].contains(&f.as_str()));
    }

    // Choose a form to show
    let default_form = "regular".to_string(); // Create a long-lived string
    let form = forms
        .choose(&mut rand::thread_rng())
        .unwrap_or(&default_form); // Use a reference to the long-lived string

    // Determine if the Pokémon is shiny
    let shiny = rand::thread_rng().gen_bool(config.shiny_rate) || random.shiny;

    // Log shiny captures if shiny is true
    if shiny {
        // Clone the pokemon_name here to avoid ownership issues
        let pokemon_name = match selected_pokemon.name.get(&config.language) {
            Some(name) => name.clone(),
            None => return Err(Error::InvalidLanguage(config.language.clone())),
        };

        // Create a shiny log entry with formatted date
        let now = chrono::Local::now(); // Get the current local date and time
        let date = now.format("%d-%m-%Y %H:%M").to_string(); // Format the date

        let shiny_entry = shiny_hunting::ShinyLogEntry {
            pokemon_name: pokemon_name.clone(), // Clone here for the log entry
            form: form.to_string(),
            date, // Use the formatted date here
            details: format!("Encountered shiny {} {}!", pokemon_name, form),
        };

        // Log the shiny capture
        shiny_hunting::log_shiny_capture(&config.shiny_log_path, &shiny_entry)?;
    }

    // Pass the active game if `game_info` is present; otherwise, default to an empty string
    let game_name = if random.game_info.is_empty() {
        String::new()
    } else {
        random.game_info.clone()
    };

    // Show Pokémon information
    show_pokemon_by_name(
        &cli::Name {
            name: selected_pokemon.slug.clone(),
            form: form.to_string(),
            shiny,
            info: random.info,
            game_info: game_name,
            under: random.under,
            no_title: random.no_title,
            padding_left: random.padding_left,
            stats: random.stats,
            unique: random.unique,
        },
        pokemon_db,
        config,
    )?;

    Ok(())
}

/// Displays information about a Pokémon based on its name and specified form.
///
/// This function searches for a Pokémon in the database using its slug (name).
/// If found, it retrieves and displays the Pokémon's ASCII art, potential
/// descriptions, and stats if requested. The function handles shiny variants
/// and different forms of the Pokémon, as well as optional titles and game
/// information based on user input.
///
/// # Parameters
/// - `name`: A reference to the `cli::Name` struct containing the Pokémon's name,
///   form, shiny status, and other display preferences.
/// - `pokemon_db`: A vector of `Pokemon` objects representing the entire Pokémon database.
/// - `config`: A reference to the `Config` struct containing configuration settings such as language.
///
/// # Returns
/// - `Result<(), Error>`: Returns `Ok(())` if the Pokémon is successfully found and displayed,
///   or an `Error` if the Pokémon is not found, the language is invalid, or other issues occur.
fn show_pokemon_by_name(
    name: &cli::Name,
    pokemon_db: Vec<Pokemon>,
    config: &Config,
) -> Result<(), Error> {
    match pokemon_db.iter().find(|p| p.slug == name.name) {
        Some(pokemon) => {
            let slug = if name.form == "regular" {
                name.name.clone()
            } else {
                format!("{}-{}", name.name, name.form)
            };

            let art_path = if name.shiny {
                format!("colorscripts/shiny/{}", slug)
            } else {
                format!("colorscripts/regular/{}", slug)
            };

            let art = Asset::get(&art_path)
                .unwrap_or_else(|| panic!("Could not read pokemon art of '{}'", slug))
                .data;
            let art = str::from_utf8(&art).expect("Invalid UTF-8 in pokemon art");

            if !name.no_title {
                let pokemon_name = match pokemon.name.get(&config.language) {
                    Some(n) => n,
                    _ => return Err(Error::InvalidLanguage(config.language.clone())),
                };
                print!("{: <1$}", pokemon_name, name.padding_left);
                match name.form.as_str() {
                    "regular" => println!(),
                    other => println!(" ({other})"),
                }
            }
            if name.game_info.is_empty() {
                let desc_lines: Vec<&str> = if name.info {
                    if let Some(game_descriptions) = pokemon.desc.get(&config.language) {
                        let games: Vec<&String> = game_descriptions.keys().collect();
                        if let Some(random_game) = games.choose(&mut thread_rng()) {
                            game_descriptions
                                .get(*random_game)
                                .map(|desc| desc.lines().collect())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
                .unwrap_or_default();

                if name.info {
                    if name.under {
                        ascii::draw_pokemon_art_under(
                            art,
                            desc_lines,
                            name.padding_left,
                            &config.language,
                        );
                    } else {
                        ascii::draw_pokemon_art(
                            art,
                            desc_lines,
                            name.padding_left,
                            &config.language,
                        );
                    }
                } else {
                    ascii::print_ascii_art(art, name.padding_left);
                }
            } else {
                let desc_lines: Vec<&str> = if name.info {
                    if let Some(game_descriptions) = pokemon.desc.get(&config.language) {
                        if let Some(game_desc) = game_descriptions.get(&name.game_info) {
                            game_desc.lines().collect()
                        } else {
                            description::get_random_description(pokemon, config)
                        }
                    } else {
                        description::get_random_description(pokemon, config)
                    }
                } else {
                    Vec::new()
                };
                if name.info {
                    if name.under {
                        ascii::draw_pokemon_art_under(
                            art,
                            desc_lines,
                            name.padding_left,
                            &config.language,
                        );
                    } else {
                        ascii::draw_pokemon_art(
                            art,
                            desc_lines,
                            name.padding_left,
                            &config.language,
                        );
                    }
                } else {
                    ascii::print_ascii_art(art, name.padding_left);
                }
            }

            // Display stats if the `stats` flag is set
            if name.stats {
                stats::display_pokemon_stats(pokemon);
            }
        }
        _ => {
            return Err(Error::InvalidPokemon(name.name.clone()));
        }
    }
    Ok(())
}

fn get_pokedex_path() -> Result<PathBuf, io::Error> {
    if let Some(mut path) = dirs::home_dir() {
        // Attempt to create .config directory
        path.push(".config");
        if let Err(e) = fs::create_dir_all(&path) {
            eprintln!("Failed to create .config directory: {}", e);
            return Err(e);
        }

        // Attempt to create kingler directory
        path.push("kingler");
        if let Err(e) = fs::create_dir_all(&path) {
            eprintln!("Failed to create kingler directory: {}", e);
            return Err(e);
        }

        // Add the file name for the Pokedex
        path.push("pokedex.json");

        Ok(path)
    } else {
        eprintln!("Home directory could not be determined. Defaulting to local path.");
        Ok(PathBuf::from("pokedex.json"))
    }
}

/// Ensures that the `.config/kingler/pokedex.json` file exists and is initialized
/// with an empty `EncounteredPokemonTracker` structure if not already present.
///
/// # Parameters
/// - `tracker_path`: The path to the `pokedex.json` tracker file.
///
/// # Returns
/// - `Result<(), Error>`: Returns `Ok(())` if the file is successfully initialized or already exists,
///   or an `Error` if any issues occur during initialization.
fn initialize_tracker(tracker_path: &PathBuf) -> Result<(), Error> {
    // Ensure the directory exists
    let tracker_dir = tracker_path.parent().unwrap();
    fs::create_dir_all(tracker_dir)?;

    // Check if the tracker file exists; if not, create it with a default empty tracker
    if !tracker_path.exists() {
        let empty_tracker = EncounteredPokemonTracker { encounters: vec![] };
        let json = serde_json::to_string(&empty_tracker)?;
        let mut file = fs::File::create(tracker_path)?;
        file.write_all(json.as_bytes())?;
    }

    Ok(())
}

fn main() -> Result<(), Error> {
    let config = Config::load()?;
    let pokemon_db = Asset::get("pokemon.json").expect("Could not read pokemon db file");
    let pokemon = load_pokemon(&pokemon_db)?;
    let args = cli::Cli::parse();
    // Construct the tracker path starting from the user's home directory
    let pokedex_path = get_pokedex_path()?;

    // Ensure the directory and file exist with proper initialization
    initialize_tracker(&pokedex_path)?;
    match args.command {
        cli::Commands::Init(shell) => cli::print_completions(shell.shell, &mut cli::build()),
        cli::Commands::List => pokemon::list_pokemon_names(pokemon),
        cli::Commands::Name(name) => show_pokemon_by_name(&name, pokemon, &config)?,
        cli::Commands::Random(random) => show_random_pokemon(&random, pokemon, &config)?,
        cli::Commands::ShowShiny => display_shiny_log(&config.shiny_log_path)?,
        cli::Commands::ShowCompletion => {
            show_completion_status(pokedex_path.to_str().expect("None"), 1025)?
        }
    }

    Ok(())
}
