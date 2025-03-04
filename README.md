# kingler

![Crates.io version](https://img.shields.io/crates/v/kingler?v=1)
![AUR version](https://img.shields.io/aur/version/kingler-bin?v=1)

Kingler is an enhanced version of Krabby [krabby](https://github.com/yannjor/krabby) that is mostly a Rust rewrite of phoney badger's [pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts)
with some extra features.

![](https://art.pixilart.com/4cd54b1841a761c.png)

## Table of contents
* [Features of Kingler](#features)
* [Installation](#installation)
  * [Arch Linux x86_64 (and derivatives)](#arch-linux-x86_64-and-derivatives)
  * [Ubuntu/Debian x86_64 (and derivatives)](#ubuntudebian-x86_64-and-derivatives)
  * [Installing from source (other distros and MacOS/Windows)](#installing-from-source-other-distros-and-macoswindows)
* [Usage](#usage)
  * [Examples](#examples)
* [Configuration](#configuration)
* [Credits](#credits)
* [Similar projects](#similar-projects)


## Features of Kingler
- Pokemon from every generation, including shinies, megas, gigantamax, and regional variants
- Print random pokemon (with filters for generations and different forms)
- Print pokemon by name
- Print pokedex entry together with the sprite
- "Catch" the shiny and display your collection
- Show pokedex progression
- Configuration file, for language, shiny rate and shiny hunting JSON
- The text of the pokedex entry was moved. With the right flag you can choose if you want it under the pokemon or alongside
- There are more pokedex description: every description of every games about that pokemon, you can choose the language and
  the game
- To make this program more pokedex-like it is added a command to see the stats
- It is added a script for scraping of the pokemon API so everyone can create a custom JSON
- Custom shell completions


## Installation

### Arch Linux x86_64 (and derivatives)

From the AUR using your favorite AUR helper

```
yay -S kingler-bin
```

Or alternatively you can manually download the PKGBUILD file from the repository, then run
```
makepkg -si
```

There is also the development package [kingler-git](https://aur.archlinux.org/packages/kingler-git) that tracks the main branch.

### Ubuntu/Debian x86_64 (and derivatives) ----TODO

Download the latest `.deb` release. Then run (replacing v.v.v with the version number)
```
dpkg -i kingler_v.v.v_amd64.deb
```

### Homebrew ----TODO

Add the tap:
```
brew tap Luca-Barra/kingler
```

Install:
```
brew install kingler
```

### Installing from source (other distros and MacOS/Windows)

To install kingler from source, you will need Rust. Installation instructions can be found [here](https://www.rust-lang.org/learn/get-started).

Now using cargo, run
```
cargo install kingler
```
Make sure you have `.cargo/bin` added to your shell `PATH`. This can be done by adding the following to your `.profile`, `.bash_profile` or `.zprofile`
```sh
export PATH="$PATH:$HOME/.cargo/bin"
```

## Usage
Run the help command `kingler help` to see the following help message.

```
Usage: kingler <COMMAND>

Commands:
  list        Print a list of all Pokémon
  name        Select Pokémon by name. Generally spelled like in the games. A few exceptions are nidoran-f, nidoran-m, mr-mime, farfetchd, flabebe type-null etc. Perhaps grep the output of list if in doubt
  random      Show a random Pokémon. This command can optionally be followed by a generation number or range (1-9) to show random Pokémon from a specific generation or range of generations. The generations can be provided as a continuous range (e.g., 1-3) or as a list of generations (1,3,6)
  init        Generate shell completions
  show-shiny  Show shiny
  help        Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```
To get more detailed information about a subcommand you can also view its help, for example
```
kingler help random
```
To get the help of the random subcommand.

### Examples
Print a specific pokemon
```
kingler name charizard
```
Print a specific shiny pokemon
```
kingler name spheal -s
```
Print a specific pokemon together with a random pokedex entry
```
kingler name mudkip -i
```
Print a specific pokemon together with a specific pokedex entry
```
kingler name mudkip -i --game-info=ruby
```
Print an alternative form of a pokemon
```
kingler name blastoise -f mega
```
Print a random pokemon (gens 1-9)
```
kingler random
```
Print random pokemon from generations 1-3
```
kingler random 1-3
```
Print a random pokemon from generations 1,3 and 6
```
kingler random 1,3,6
```
Print a random pokemon excluding megas, gigantamax and regional variants
```
kingler random --no-mega --no-gmax --no-regional
```
Print a random pokemon with his stats
```
kingler random --stats
```
Print a random pokemon with stats and info under
```
kingler random -iu --stats
```
Print the list of shiny pokemon "catch"
```
kingler show-shiny
```
## Configuration
When the program is run, a TOML config file will automatically be created in the user's config
directory (usually `~/.config`) under `kingler/config.toml` if it doesn't exist already. 

On MacOS the config will be in: `/Users/<username>/Library/Application Support/kingler`
On Windows this will be: `C:\Users\<username>\AppData\Roaming\kingler`

```toml
# The language to use when printing the pokemon's name and/or description.
# Possible options include en (English), fr (French), de (German), ja (Japanese),
# ko (Korean), es (Espanol), it (Italian), ja-Hrkt (Japanese Hiragana) 
# zh_hans (Chinese with simplified characters), zh_hant (Chinese with traditional characters)
language = 'en'

# The probability to show a shiny pokemon when using the random command
shiny_rate = 0.0078125
```

## Credits
The pokemon sprites for kingler were generated using sprites from [PokéSprite](https://msikma.github.io/pokesprite/)
and converted to unicode using Phoney Badger's [pokemon-generator-scripts](https://gitlab.com/phoneybadger/pokemon-generator-scripts).
The code that got me started is from yannjor's [krabby](https://github.com/yannjor/krabby) 
The pokemon data was obtained from [PokéAPI](https://github.com/PokeAPI/pokeapi).


## Similar projects
- [pokemon-colorscripts](https://gitlab.com/phoneybadger/pokemon-colorscripts)
- [krabby](https://github.com/yannjor/krabby)
- [pokeget](https://github.com/talwat/pokeget)
- [pokeshell](https://github.com/acxz/pokeshell)
