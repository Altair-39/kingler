import requests
import json


def wrap_text(text, line_length):
    """
    Wrap the text to a specified line length without breaking words.

    Args:
        text (str): The text to wrap.
        line_length (int): The maximum length of a line.

    Returns:
        str: The wrapped text with new line characters.
    """
    words = text.split()  # Split the text into words
    lines = []
    current_line = ""

    for word in words:
        # If adding the next word would exceed the line length, start a new line
        if len(current_line) + len(word) + 1 > line_length:
            lines.append(current_line)  # Add the current line to lines
            current_line = word  # Start a new line with the current word
        else:
            # Add the word to the current line
            if current_line:
                current_line += " "  # Add a space before the next word
            current_line += word  # Add the word to the current line

    # Add the last line if it's not empty
    if current_line:
        lines.append(current_line)

    return "\n".join(lines)  # Join all lines with newline characters


def fetch_basic_pokemon_data(pokemon_id):
    """
    Fetch basic Pokémon data from the PokéAPI.

    Args:
        pokemon_id (int): The ID of the Pokémon to fetch.

    Returns:
        dict: A dictionary containing the basic Pokémon data.
    """
    base_url = "https://pokeapi.co/api/v2/pokemon/"
    response = requests.get(f"{base_url}{pokemon_id}")

    if response.status_code == 200:
        data = response.json()  # Parse the JSON response
        return {
            "slug": data['name'],  # Get the Pokémon's name (slug)
            "idx": data['id'],  # Get the Pokémon's ID
            "stats": {stat['stat']['name']: stat['base_stat']
                      for stat in data['stats']}  # Fetch stats data
        }
    else:
        print(f"Failed to fetch data for Pokémon ID {pokemon_id}: "
              f"{response.status_code}")
        return None  # Return None if the fetch fails


def fetch_species_data(pokemon_id):
    """
    Fetch species data (flavor text) for a given Pokémon ID.

    Args:
        pokemon_id (int): The ID of the Pokémon.

    Returns:
        dict: A dictionary containing the flavor text for different languages.
    """
    species_url = "https://pokeapi.co/api/v2/pokemon-species/"
    response = requests.get(f"{species_url}{pokemon_id}")

    if response.status_code == 200:
        species_data = response.json()
        language_descriptions = {}  # Initialize a dictionary for language descriptions

        for entry in species_data['flavor_text_entries']:
            language = entry['language']['name']
            version = entry['version']['name']  # Get the game version
            flavor_text = entry['flavor_text'].replace('\n', ' ').replace('\f', ' ')
            formatted_flavor_text = wrap_text(flavor_text, 50)  # Wrap the text

            # Initialize the language entry if it doesn't exist
            if language not in language_descriptions:
                language_descriptions[language] = {}

            # Store the formatted flavor text based on the version
            language_descriptions[language][version] = formatted_flavor_text

        return language_descriptions  # Return the language descriptions
    else:
        print(f"Failed to fetch species data for Pokémon ID {pokemon_id}: "
              f"{response.status_code}")
        return None  # Return None if the fetch fails


def format_pokemon_info(pokemon_data, language_descriptions):
    """
    Create a structured dictionary with relevant Pokémon information.

    Args:
        pokemon_data (dict): Basic Pokémon data including 'slug', 'idx', and 'stats'.
        language_descriptions (dict): Flavor text descriptions by language.

    Returns:
        dict: A structured dictionary containing formatted Pokémon info.
    """
    idx = pokemon_data['idx']
    slug = pokemon_data['slug']
    stats = pokemon_data['stats']

    # Create a Pokémon info dictionary with relevant data
    pokemon_info = {
        "idx": idx,  # Pokémon ID
        "slug": slug,  # Pokémon name (slug)
        "gen": (idx - 1) // 100 + 1,  # Calculate the generation based on ID
        "name": {
            "en": slug.capitalize(),  # Capitalize slug for the English name
        },
        "desc": language_descriptions,  # Use descriptions by language
        "stats": stats,  # Add the stats data
        "forms": [],  # Placeholder for forms
    }
    return pokemon_info  # Return the Pokémon info


def fetch_pokemon_data():
    """
    Fetch Pokémon data from the PokéAPI and format it for storage.

    Returns:
        list: A list of dictionaries containing Pokémon data.
    """
    pokemon_data_list = []  # Initialize an empty list to store Pokémon data

    # Fetch data for Pokémon with IDs from 1 to 1025
    for pokemon_id in range(1, 1026):
        basic_data = fetch_basic_pokemon_data(pokemon_id)  # Fetch basic Pokémon data
        if basic_data:
            species_data = fetch_species_data(pokemon_id)  # Fetch species data
            if species_data:
                # Format and append Pokémon info to the list
                formatted_info = format_pokemon_info(basic_data, species_data)
                pokemon_data_list.append(formatted_info)

    return pokemon_data_list  # Return the list of Pokémon data


def main():
    """
    Main function to fetch Pokémon data and save it to a JSON file.
    """
    all_pokemon = fetch_pokemon_data()  # Fetch all Pokémon data

    # Save the Pokémon data to a JSON file
    with open('pokemon.json', 'w') as json_file:
        json.dump(all_pokemon, json_file, indent=4)  # Write data to file
    print("Pokédex data saved to pokemon.json")


if __name__ == "__main__":
    main()
