use crate::Shell;
use clap::{arg, Command};
use clap::{Args, Parser, Subcommand};
use clap_complete::{generate, Generator};
use std::io;

/// Represents the command-line interface (CLI) for the Pokémon application.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

/// Represents a shell name for generating completions.
#[derive(Debug, Args)]
pub struct ShellName {
    pub shell: Shell,
}

/// Represents the options for selecting a Pokémon by name.
///
/// # Fields
/// - `name`: The name of the Pokémon to show.
/// - `form`: An optional parameter for specifying an alternative form of the Pokémon.
/// - `shiny`: A flag indicating whether to show the shiny version of the Pokémon.
/// - `info`: A flag indicating whether to print the Pokédex entry if it exists.
/// - `game_info`: The specific game for which to retrieve the Pokédex entry.
/// - `no_title`: A flag that, if set, will prevent displaying the Pokémon's name.
/// - `under`: A flag indicating whether to display the description under the Pokémon art.
/// - `padding_left`: An integer specifying the amount of left padding for display.
/// - `stats`: A flag indicating whether to show the Pokémon's stats.
#[derive(Debug, Args)]
pub struct Name {
    /// Name of the Pokémon to show
    pub name: String,

    /// Show an alternative form of the Pokémon. Can be one of: mega, mega-x,
    /// mega-y, gmax, alola, hisui, galar, paldea
    #[clap(short, long, default_value = "regular")]
    pub form: String,

    /// Show the shiny version of the Pokémon instead
    #[clap(short, long)]
    pub shiny: bool,

    /// Print Pokédex entry (if it exists)
    #[clap(short, long)]
    pub info: bool,

    /// Pokédex entry for a specific game
    #[clap(long, default_value = "")]
    pub game_info: String,

    /// Do not display Pokémon name
    #[clap(long)]
    pub no_title: bool,

    /// Description under or not
    #[clap(short, long)]
    pub under: bool,

    /// Left padding
    #[clap(long, default_value = "0")]
    pub padding_left: usize,

    /// Show Pokémon stats
    #[clap(long)]
    pub stats: bool,
}

/// Represents options for showing a random Pokémon.
///
/// # Fields
/// - `generations`: A string specifying the generation number, range (1-9), or list of generations (1,3,6).
/// - `info`: A flag indicating whether to print the Pokédex entry if it exists.
/// - `game_info`: The specific game for which to retrieve the Pokédex entry.
/// - `shiny`: A flag indicating whether to show the shiny version of the Pokémon.
/// - `no_title`: A flag that, if set, will prevent displaying the Pokémon's name.
/// - `under`: A flag indicating whether to display the description under the Pokémon art.
/// - `no_mega`: A flag indicating whether to exclude mega Pokémon.
/// - `no_gmax`: A flag indicating whether to exclude Gigantamax Pokémon.
/// - `no_regional`: A flag indicating whether to exclude regional forms.
/// - `padding_left`: An integer specifying the amount of left padding for display.
/// - `stats`: A flag indicating whether to show the Pokémon's stats.
#[derive(Debug, Args)]
pub struct Random {
    /// Generation number, range (1-9), or list of generations (1,3,6)
    #[clap(default_value = "1-9")]
    pub generations: String,

    /// Print Pokédex entry (if it exists)
    #[clap(short, long)]
    pub info: bool,

    #[clap(long, default_value = "")]
    pub game_info: String,

    /// Show the shiny version of the Pokémon instead
    #[clap(short, long)]
    pub shiny: bool,

    /// Do not display Pokémon name
    #[clap(long)]
    pub no_title: bool,

    /// Description under or not
    #[clap(short, long)]
    pub under: bool,

    /// Do not show mega Pokémon
    #[clap(long)]
    pub no_mega: bool,

    /// Do not show Gigantamax Pokémon
    #[clap(long)]
    pub no_gmax: bool,

    /// Do not show regional Pokémon
    #[clap(long)]
    pub no_regional: bool,

    /// Left padding
    #[clap(long, default_value = "0")]
    pub padding_left: usize,

    /// Show Pokémon stats
    #[clap(long)]
    pub stats: bool,
}

/// Represents the various commands available in the CLI.
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print a list of all Pokémon
    List,
    /// Select Pokémon by name. Generally spelled like in the games.
    /// A few exceptions are nidoran-f, nidoran-m, mr-mime, farfetchd,
    /// flabebe type-null etc. Perhaps grep the output of list if in doubt.
    Name(crate::cli::Name),
    /// Show a random Pokémon. This command can optionally be followed by a
    /// generation number or range (1-9) to show random Pokémon from a specific
    /// generation or range of generations. The generations can be provided as
    /// a continuous range (e.g., 1-3) or as a list of generations (1,3,6).
    Random(crate::cli::Random),
    /// Generate shell completions
    Init(crate::cli::ShellName),
    /// Show shiny
    ShowShiny,
}

/// Builds the command structure for the CLI, including subcommands and common arguments.
///
/// # Returns
/// - `Command`: The built command structure, ready for use in the CLI.
pub fn build() -> Command {
    let common_args = [
        arg!(-i --info "Print Pokédex entry (if it exists)"),
        arg!(-s --shiny "Show the shiny Pokémon version instead"),
        arg!(--"game-info" "Choose a game for the Pokédex entry"),
        arg!(--"no-title" "Do not display Pokémon name"),
        arg!(--"padding-left" "Set amount of padding to the left [default: 0]"),
        arg!(-u --under "Show the Pokédex entry under the Pokémon"),
        arg!(--stats "Show the Pokémon stats"),
    ];
    let init = Command::new("init")
        .about("Generate shell completions")
        .args([
            arg!(["bash"]),
            arg!(["zsh"]),
            arg!(["fish"]),
            arg!(["powershell"]),
            arg!(["elvish"]),
        ]);
    let list = Command::new("list").about("List all names of Pokémon");
    let name = Command::new("name")
        .about("Select Pokémon by name: e.g., 'pikachu'")
        .arg(arg!([name] "Who's that Pokémon!?"))
        .args(&common_args);

    let random = Command::new("random")
        .about("Show a random Pokémon")
        .arg(
            arg!([GENERATIONS] "Generation number, range (1-9), or list of generations (1,3,6) [default: 1-9]"),
        )
        .args(common_args)
        .args([
            arg!(--"no-mega" "Do not show mega Pokémon"),
            arg!(--"no-gmax" "Do not show Gigantamax Pokémon"),
            arg!(--"no-regional" "Do not show regional Pokémon"),
        ]);

    Command::new("kingler").subcommands([init, list, name, random])
}

/// Prints the completions for the specified command to the standard output.
///
/// # Parameters
/// - `gen`: The completion generator for the desired shell.
/// - `cmd`: The command for which to generate completions.
pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
