use crate::Shell;
use clap::{arg, Command};
use clap::{Args, Parser, Subcommand};
use clap_complete::{generate, Generator};
use std::io;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Args)]
pub struct ShellName {
    pub shell: Shell,
}

#[derive(Debug, Args)]
pub struct Name {
    /// Name of the pokemon to show
    pub name: String,

    /// Show an alternative form of the pokemon. Can be one of: mega, mega-x,
    /// mega-y, gmax, alola, hisui, galar, paldea
    #[clap(short, long, default_value = "regular")]
    pub form: String,

    /// Show the shiny version of the pokemon instead
    #[clap(short, long)]
    pub shiny: bool,

    /// Print pokedex entry (if it exists)
    #[clap(short, long)]
    pub info: bool,

    /// Pokedex entry for specific game
    #[clap(long, default_value = "")]
    pub game_info: String,

    /// Do not display pokemon name
    #[clap(long)]
    pub no_title: bool,

    /// desc under or not
    #[clap(short, long)]
    pub under: bool,

    /// Left padding
    #[clap(long, default_value = "0")]
    pub padding_left: usize,

    /// Stats
    #[clap(long)]
    pub stats: bool,
}

#[derive(Debug, Args)]
pub struct Random {
    /// Generation number, range (1-9), or list of generations (1,3,6)
    #[clap(default_value = "1-9")]
    pub generations: String,

    /// Print pokedex entry (if it exists)
    #[clap(short, long)]
    pub info: bool,

    #[clap(long, default_value = "")]
    pub game_info: String,

    /// Show the shiny version of the pokemon instead
    #[clap(short, long)]
    pub shiny: bool,

    /// Do not display pokemon name
    #[clap(long)]
    pub no_title: bool,

    /// desc under or not
    #[clap(short, long)]
    pub under: bool,

    /// Do not show mega pokemon
    #[clap(long)]
    pub no_mega: bool,

    /// Do not show gigantamax pokemon
    #[clap(long)]
    pub no_gmax: bool,

    /// Do not show regional pokemon
    #[clap(long)]
    pub no_regional: bool,

    /// Left padding
    #[clap(long, default_value = "0")]
    pub padding_left: usize,

    /// Stats
    #[clap(long)]
    pub stats: bool,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Print list of all pokemon
    List,
    /// Select pokemon by name. Generally spelled like in the games.
    /// A few exceptions are nidoran-f, nidoran-m, mr-mime, farfetchd,
    /// flabebe type-null etc. Perhaps grep the output of list if in doubt.
    Name(crate::cli::Name),
    /// Show a random pokemon. This command can optionally be followed by a
    /// generation number or range (1-9) to show random pokemon from a specific
    /// generation or range of generations. The generations can be provided as
    /// a continuous range (eg. 1-3) or as a list of generations (1,3,6)
    Random(crate::cli::Random),
    /// zsh completions
    Init(crate::cli::ShellName),
}

pub fn build() -> Command {
    let common_args = [
        arg!(-i --info "Print pokedex entry (if it exists)"),
        arg!(-s --shiny "Show the shiny pokemon version instead"),
        arg!(--"game-info" "Choose a game for the pokedex entry"),
        arg!(--"no-title" "Do not display pokemon name"),
        arg!(--"padding-left" "Set amount of padding to the left [default: 0]"),
        arg!(-u --under "Show the pokedex entry under the pokemon"),
        arg!(--stats "Show the pokemon stats"),
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
    let list = Command::new("list").about("List all names of pokemons");
    let name = Command::new("name")
        .about("Select pokemon by name: eg. 'pikachu'")
        .arg(arg!([name] "Who's that pokemon!?"))
        .args(&common_args);

    let random = Command::new("random")
        .about("Show random pokemon")
        .arg(
            arg!([GENERATIONS] "Generation number, range (1-9), or list of generations (1,3,6) [default: 1-9]"),
        )
        .args(common_args)
        .args([
            arg!(--"no-mega" "Do not show mega pokemon"),
             arg!(--"no-gmax" "Do not show gigantamax pokemon"),
            arg!(--"no-regional" "Do not show regional pokemon"),
        ]);

    Command::new("kingler").subcommands([init, list, name, random])
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
