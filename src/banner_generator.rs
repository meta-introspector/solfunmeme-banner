use figlet_rs::FIGfont;
use super::mycelium_pattern::generate_mycelium_pattern;
use super::constants::{FONTS, COLORS};
use super::random_stream::RandomStream;

pub fn generate_solfunmeme_banner(git_hash: &str, iteration: usize, growth_rate: f64) -> String {
    // Convert the Git hash to a number to use as a seed.
    let seed: u64 = git_hash.chars().map(|c| c as u64).sum();

    // Create a random stream for deterministic generation.
    let mut rng = RandomStream::new(seed);

    // Use the seed to select a font deterministically.
    let selected_font_name = FONTS[seed as usize % FONTS.len()];
    let font = FIGfont::from_content(selected_font_name).unwrap_or_else(|_| FIGfont::standard().unwrap());

    // Generate the ASCII art using figlet.
    let banner_text = font.convert("SOLFUNMEME").unwrap();

    let banner_string = banner_text.to_string();
    let banner_lines: Vec<&str> = banner_string.lines().collect();
    let banner_height = banner_lines.len();
    let banner_width = banner_lines.get(0).map_or(0, |s| s.len());

    let mycelium_height = banner_height + 4; // Add some padding
    let mycelium_width = banner_width + 4; // Add some padding

    let mut mycelium_grid = generate_mycelium_pattern(&mut rng, mycelium_width, mycelium_height, iteration, growth_rate);

    // Overlay the figlet banner onto the mycelium grid
    let start_y = (mycelium_height - banner_height) / 2;
    let start_x = (mycelium_width - banner_width) / 2;

    for (y_offset, line) in banner_lines.iter().enumerate() {
        for (x_offset, char) in line.chars().enumerate() {
            if char != ' ' { // Only overlay non-space characters
                let target_y = start_y + y_offset;
                let target_x = start_x + x_offset;
                if target_y < mycelium_height && target_x < mycelium_width {
                    mycelium_grid[target_y][target_x] = char;
                }
            }
        }
    }

    // Convert the grid to a string and apply colors
    let mycelium_color_code = COLORS[seed as usize % COLORS.len()];
    let banner_color_code = COLORS[(seed as usize + 1) % COLORS.len()]; // A different color for the banner text

    let mut final_banner = String::new();
    for y in 0..mycelium_height {
        for x in 0..mycelium_width {
            let char = mycelium_grid[y][x];
            if char == ' ' || char == '_' || char == '-' || char == '/' || char == '\\'|| char == '+' || char == '.' || char == ':' || char == ';' || char == '~' {
                final_banner.push_str(&format!("{}{}", mycelium_color_code, char));
            } else {
                final_banner.push_str(&format!("{}{}", banner_color_code, char));
            }
        }
        final_banner.push_str("\x1b[0m\n"); // Reset color and new line
    }

    final_banner
}
