mod cli;
mod config;
mod error;
mod pokemon;

use config::Config;
use error::Error;
use pokemon::*;

use clap::{Args, Parser, Subcommand};
use clap_complete::Shell;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use rust_embed::RustEmbed;

use std::str;

/// Print pokemon sprites in your terminal
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Print list of all pokemon
    List,
    /// Select pokemon by name. Generally spelled like in the games.
    /// A few exceptions are nidoran-f, nidoran-m, mr-mime, farfetchd,
    /// flabebe type-null etc. Perhaps grep the output of list if in doubt.
    Name(Name),
    /// Show a random pokemon. This command can optionally be followed by a
    /// generation number or range (1-9) to show random pokemon from a specific
    /// generation or range of generations. The generations can be provided as
    /// a continuous range (eg. 1-3) or as a list of generations (1,3,6)
    Random(Random),
    /// zsh completions
    Init(ShellName),
}

#[derive(Debug, Args)]
struct ShellName {
    shell: Shell,
}

#[derive(Debug, Args)]
struct Name {
    /// Name of the pokemon to show
    name: String,

    /// Show an alternative form of the pokemon. Can be one of: mega, mega-x,
    /// mega-y, gmax, alola, hisui, galar, paldea
    #[clap(short, long, default_value = "regular")]
    form: String,

    /// Show the shiny version of the pokemon instead
    #[clap(short, long)]
    shiny: bool,

    /// Print pokedex entry (if it exists)
    #[clap(short, long)]
    info: bool,

    /// Pokedex entry for specific game
    #[clap(long, default_value = "")]
    game_info: String,

    /// Do not display pokemon name
    #[clap(long)]
    no_title: bool,

    /// desc under or not
    #[clap(short, long)]
    under: bool,

    /// Left padding
    #[clap(long, default_value = "0")]
    padding_left: usize,

    /// Stats
    #[clap(long)]
    stats: bool,
}

#[derive(Debug, Args)]
struct Random {
    /// Generation number, range (1-9), or list of generations (1,3,6)
    #[clap(default_value = "1-9")]
    generations: String,

    /// Print pokedex entry (if it exists)
    #[clap(short, long)]
    info: bool,

    #[clap(long, default_value = "")]
    game_info: String,

    /// Show the shiny version of the pokemon instead
    #[clap(short, long)]
    shiny: bool,

    /// Do not display pokemon name
    #[clap(long)]
    no_title: bool,

    /// desc under or not
    #[clap(short, long)]
    under: bool,

    /// Do not show mega pokemon
    #[clap(long)]
    no_mega: bool,

    /// Do not show gigantamax pokemon
    #[clap(long)]
    no_gmax: bool,

    /// Do not show regional pokemon
    #[clap(long)]
    no_regional: bool,

    /// Left padding
    #[clap(long, default_value = "0")]
    padding_left: usize,

    /// Stats
    #[clap(long)]
    stats: bool,
}

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

fn list_pokemon_names(pokemon_db: Vec<Pokemon>) {
    pokemon_db.iter().for_each(|p| println!("{}", p.slug));
}

fn show_random_pokemon(
    random: &Random,
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
        &Name {
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

fn draw_pokemon_art(art: &str, desc_lines: Vec<&str>, padding_left: usize, language: &str) {
    let lines: Vec<&str> = art.lines().collect();
    let desc_width = desc_lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // Calculate the midpoint of the ASCII art
    let mid_index = lines.len() / 2;

    // Determine the starting index for descriptions
    let start_index = if lines.len() >= 3 {
        mid_index.saturating_sub(1) // Start one line above the midpoint if there are 3 or more lines
    } else {
        mid_index // Use midpoint for less than 3 lines
    };

    // Print the art with descriptions starting from the determined start index
    for (i, line) in lines.iter().enumerate() {
        print!("{: <1$}", line, padding_left);
        print!("\t\t");

        // Print the description if within the range and adjust its starting position
        if i >= start_index && i - start_index < desc_lines.len() {
            // Calculate the padding for the description to start at the determined index
            let description_padding = padding_left + desc_width + 1; // Add extra space for visual separation
            println!(
                "\x1b[37m{: <1$}\x1b[0m",
                desc_lines[i - start_index],
                description_padding
            );
        } else {
            println!();
        }
    }

    // Inform if there are no descriptions available
    if desc_lines.is_empty() {
        println!(
            "{: <1$}No descriptions available for language: {} {}",
            "", padding_left, language
        );
    }
}

fn draw_pokemon_art_under(art: &str, desc_lines: Vec<&str>, padding_left: usize, language: &str) {
    let lines: Vec<&str> = art.lines().collect();
    let desc_width = desc_lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // Print the ASCII art
    for line in lines.iter() {
        print!("{: <1$}", line, padding_left);
        println!(); // New line after each art line
    }

    // Print descriptions if available
    if !desc_lines.is_empty() {
        let description_padding = padding_left + desc_width + 1; // Add extra space for visual separation
        for desc in desc_lines {
            println!("\x1b[37m{: <1$}\x1b[0m", desc, description_padding);
        }
    } else {
        // Inform if there are no descriptions available
        println!(
            "{: <1$}No descriptions available for language: {} {}",
            "", padding_left, language
        );
    }
}

fn print_ascii_art(art: &str, padding_left: usize) {
    for line in art.lines() {
        print!("{: <1$}", line, padding_left);
        println!(); // New line after each art line
    }
}
fn get_random_description<'a>(pokemon: &'a Pokemon, config: &'a Config) -> Vec<&'a str> {
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

fn show_pokemon_by_name(
    name: &Name,
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
                        draw_pokemon_art_under(
                            art,
                            desc_lines,
                            name.padding_left,
                            &config.language,
                        );
                    } else {
                        draw_pokemon_art(art, desc_lines, name.padding_left, &config.language);
                    }
                } else {
                    print_ascii_art(art, name.padding_left);
                }
            } else {
                let desc_lines: Vec<&str> = if name.info {
                    if let Some(game_descriptions) = pokemon.desc.get(&config.language) {
                        if let Some(game_desc) = game_descriptions.get(&name.game_info) {
                            game_desc.lines().collect()
                        } else {
                            get_random_description(pokemon, config)
                        }
                    } else {
                        get_random_description(pokemon, config)
                    }
                } else {
                    Vec::new()
                };
                if name.info {
                    if name.under {
                        draw_pokemon_art_under(
                            art,
                            desc_lines,
                            name.padding_left,
                            &config.language,
                        );
                    } else {
                        draw_pokemon_art(art, desc_lines, name.padding_left, &config.language);
                    }
                } else {
                    print_ascii_art(art, name.padding_left);
                }
            }

            // Display stats if the `stats` flag is set
            if name.stats {
                if let Some(stats) = &pokemon.stats {
                    // Define pairs of stats to display on the same line
                    let stat_pairs = [
                        ("hp", "speed"),
                        ("attack", "special-attack"),
                        ("defense", "special-defense"),
                    ];

                    // Print each pair of stats on the same line
                    for &(stat1, stat2) in &stat_pairs {
                        // Get the values for each stat, if they exist
                        let value1 = stats.get(stat1).unwrap_or(&0);
                        let value2 = stats.get(stat2).unwrap_or(&0);

                        // Format the output with the stat names followed by their values
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
    let args = Cli::parse();

    match args.command {
        Commands::Init(shell) => cli::print_completions(shell.shell, &mut cli::build()),
        Commands::List => list_pokemon_names(pokemon),
        Commands::Name(name) => show_pokemon_by_name(&name, pokemon, &config)?,
        Commands::Random(random) => show_random_pokemon(&random, pokemon, &config)?,
    }

    Ok(())
}
