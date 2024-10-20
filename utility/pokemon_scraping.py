import requests
import json


def wrap_text(text, line_length):
    """
    Wrap the text to a specified line length without breaking words.
    """
    words = text.split()  # Split the text into words
    lines = []
    current_line = ""

    for word in words:
        # If adding the next word would exceed the line length, start a new line
        if len(current_line) + len(word) + 1 > line_length:
            lines.append(current_line)
            current_line = word
        else:
            # Add the word to the current line
            if current_line:
                current_line += " "
            current_line += word

    # Add the last line
    if current_line:
        lines.append(current_line)

    return "\n".join(lines)


def fetch_pokemon_data():
    base_url = "https://pokeapi.co/api/v2/pokemon/"
    species_url = "https://pokeapi.co/api/v2/pokemon-species/"
    pokemon_data = []

    # Fetch data for all Pokémon
    for id in range(1, 1026):
        # Fetch basic Pokémon data
        response = requests.get(f"{base_url}{id}")
        if response.status_code == 200:
            data = response.json()
            slug = data['name']
            idx = data['id']

            # Fetch stats data
            stats = {stat['stat']['name']: stat['base_stat'] for stat in data['stats']}

            # Fetch species data to get the Pokédex entry
            species_response = requests.get(f"{species_url}{id}")
            if species_response.status_code == 200:
                species_data = species_response.json()

                language_descriptions = {}
                for entry in species_data['flavor_text_entries']:
                    # Get the language of the entry
                    language = entry['language']['name']
                    version = entry['version']['name']  # Get the game version
                    flavor_text = entry['flavor_text'].replace(
                        '\n', ' ').replace('\f', ' ')  # Clean up the text

                    # Wrap the text without breaking words
                    formatted_flavor_text = wrap_text(flavor_text, 50)

                    # Initialize the language entry if it doesn't exist
                    if language not in language_descriptions:
                        language_descriptions[language] = {}

                    # Store the formatted flavor text based on the version
                    language_descriptions[language][version] = formatted_flavor_text

                # Create Pokémon info dictionary
                pokemon_info = {
                    "idx": idx,
                    "slug": slug,
                    "gen": (idx - 1) // 100 + 1,
                    "name": {
                        "en": slug.capitalize(),  # Capitalize slug for the English name
                    },
                    "desc": language_descriptions,  # Use descriptions by language
                    "stats": stats,  # Add the stats data
                    "forms": [],  # Placeholder for forms
                }
                pokemon_data.append(pokemon_info)
            else:
                print(f"Failed to fetch species data for {
                      slug}: {species_response.status_code}")
        else:
            print(f"Failed to fetch data for Pokémon ID {id}: {response.status_code}")

    return pokemon_data


def main():
    all_pokemon = fetch_pokemon_data()

    with open('pokemon.json', 'w') as json_file:
        json.dump(all_pokemon, json_file, indent=4)
    print("Pokédex data saved to pokemon.json")


if __name__ == "__main__":
    main()
