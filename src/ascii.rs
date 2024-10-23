/// Draws an ASCII art representation of a Pokémon, aligning the description next to the art.
///
/// The function displays the provided ASCII art and aligns the given description text
/// next to the art. The description starts from a calculated index based on the
/// midpoint of the ASCII art. If there are no descriptions, a message indicating
/// the lack of descriptions for the specified language is displayed.
///
/// # Arguments
///
/// * `art` - A string slice that holds the ASCII art to be displayed.
/// * `desc_lines` - A vector of string slices containing the description lines to be shown next to the art.
/// * `padding_left` - The number of spaces to pad to the left of each line of art.
/// * `language` - The language of the description; used in the message when no descriptions are available.
pub fn draw_pokemon_art(art: &str, desc_lines: Vec<&str>, padding_left: usize, language: &str) {
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

/// Draws an ASCII art representation of a Pokémon, displaying the description below the art.
///
/// The function displays the provided ASCII art and shows the given description
/// text below it. If no descriptions are available, a message indicating the lack
/// of descriptions for the specified language is displayed.
///
/// # Arguments
///
/// * `art` - A string slice that holds the ASCII art to be displayed.
/// * `desc_lines` - A vector of string slices containing the description lines to be shown below the art.
/// * `padding_left` - The number of spaces to pad to the left of each line of art.
/// * `language` - The language of the description; used in the message when no descriptions are available.
pub fn draw_pokemon_art_under(
    art: &str,
    desc_lines: Vec<&str>,
    padding_left: usize,
    language: &str,
) {
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

/// Prints an ASCII art representation of a Pokémon with optional left padding.
///
/// The function simply displays the provided ASCII art with each line padded
/// to the left by the specified number of spaces.
///
/// # Arguments
///
/// * `art` - A string slice that holds the ASCII art to be displayed.
/// * `padding_left` - The number of spaces to pad to the left of each line of art.
pub fn print_ascii_art(art: &str, padding_left: usize) {
    for line in art.lines() {
        print!("{: <1$}", line, padding_left);
        println!(); // New line after each art line
    }
}
