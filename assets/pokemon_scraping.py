import requests
import json


def fetch_pokemon_data():
    base_url = "https://pokeapi.co/api/v2/pokemon/"
    species_url = "https://pokeapi.co/api/v2/pokemon-species/"
    pokemon_data = []

    # Fetch data for all Pokémon
    for id in range(1, 1009):  # 1008 is the total number of Pokémon as of now
        # Fetch basic Pokémon data
        response = requests.get(f"{base_url}{id}")
        if response.status_code == 200:
            data = response.json()
            slug = data['name']
            idx = data['id']

            # Fetch species data to get the Pokédex entry
            species_response = requests.get(f"{species_url}{id}")
            if species_response.status_code == 200:
                species_data = species_response.json()

                # Collect flavor text for English game versions only
                game_descriptions = {}
                for entry in species_data['flavor_text_entries']:
                    # Get the language of the entry
                    language = entry['language']['name']
                    if language == 'en':  # Check if the language is English
                        version = entry['version']['name']  # Get the game version
                        flavor_text = entry['flavor_text'].replace(
                            '\n', ' ').replace('\f', ' ')  # Clean up the text
                        # Add the flavor text to the respective game version
                        if version not in game_descriptions:
                            game_descriptions[version] = flavor_text

                # Create Pokémon info dictionary
                pokemon_info = {
                    "idx": idx,
                    "slug": slug,
                    "gen": (idx - 1) // 100 + 1,  # Calculate generation
                    "name": {
                        "en": slug.capitalize(),  # Capitalize slug for the English name
                    },
                    "desc": game_descriptions,  # Use descriptions by game version
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
    # Now, instead of wrapping in a dictionary, we write the list directly
    with open('pokemon.json', 'w') as json_file:
        json.dump(all_pokemon, json_file, indent=4)  # Write list directly
    print("Pokédex data saved to pokedex.json")


if __name__ == "__main__":
    main()
