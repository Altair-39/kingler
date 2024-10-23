mod ascii;
mod cli;
mod config;
mod description;
mod error;
mod pokemon;
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

use std::str;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

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
        Some(gens) => gens,
        _none => {
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
    let pokemon = match pokemon.choose(&mut rand::thread_rng()) {
        Some(&p) => p,
        _none => return Err(Error::InvalidGeneration(random.generations.clone())),
    };

    // Prepare forms to choose from
    let mut forms = vec!["regular".to_string()];
    forms.extend(pokemon.forms.iter().cloned());

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

    let shiny = rand::thread_rng().gen_bool(config.shiny_rate) || random.shiny;

    // Pass the active game if `game_info` is present; otherwise, default to an empty string
    let game_name = if random.game_info.is_empty() {
        String::new()
    } else {
        random.game_info.clone()
    };

    show_pokemon_by_name(
        &cli::Name {
            name: pokemon.slug.clone(),
            form: form.to_string(),
            shiny,
            info: random.info,
            game_info: game_name,
            under: random.under,
            no_title: random.no_title,
            padding_left: random.padding_left,
            stats: random.stats,
        },
        pokemon_db,
        config,
    )
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

fn main() -> Result<(), Error> {
    let config = Config::load()?;
    let pokemon_db = Asset::get("pokemon.json").expect("Could not read pokemon db file");
    let pokemon = load_pokemon(&pokemon_db)?;
    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Init(shell) => cli::print_completions(shell.shell, &mut cli::build()),
        cli::Commands::List => pokemon::list_pokemon_names(pokemon),
        cli::Commands::Name(name) => show_pokemon_by_name(&name, pokemon, &config)?,
        cli::Commands::Random(random) => show_random_pokemon(&random, pokemon, &config)?,
    }

    Ok(())
}
