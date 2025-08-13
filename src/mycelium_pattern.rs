use super::random_stream::RandomStream;
use super::constants::{MYCELIUM_CHARS, EMOJIS};
use rand::Rng;

pub fn generate_mycelium_pattern(rng: &mut RandomStream, width: usize, height: usize, iteration: usize, growth_rate: f64) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = vec![vec![' '; width]; height];

    // Place a fixed "seed mushroom" emoji at the center for the first few iterations
    let center_x = width / 2;
    let center_y = height / 2;
    if iteration < 5 { // For the first 5 iterations, ensure a central mushroom
        if center_x < width && center_y < height {
            grid[center_y][center_x] = '🍄';
        }
    }

    // Initialize some starting points for mycelium growth, potentially near the center
    // These points will be influenced by the iteration to simulate growth from the seed
    let initial_points = (rng.gen_range(3..=7) as f64 * (iteration as f64 / 60.0).min(1.0)) as usize; // More points as iteration increases
    for _ in 0..initial_points {
        let start_x = (width as f64 / 4.0 + rng.next_f64() * width as f64 / 2.0) as usize;
        let start_y = (height as f64 / 4.0 + rng.next_f64() * height as f64 / 2.0) as usize;
        if start_x < width && start_y < height {
            // Initial nodes can also be emojis, more likely as iteration increases
            if rng.next_f64() < (0.05 + iteration as f64 * 0.005 * growth_rate).min(0.5) { // Increased chance for emoji
                let emoji_str = EMOJIS[rng.gen_range(0..=EMOJIS.len() - 1)];
                grid[start_y][start_x] = emoji_str.chars().next().unwrap_or('O');
            } else {
                grid[start_y][start_x] = 'O'; // Use 'O' for initial nodes
            }
        }
    }

    // Simulate growth for multiple iterations, inner iterations increase with outer iteration
    let inner_iterations = (iteration as f64 * 0.5 * growth_rate).min(20.0) as usize; // Max 20 inner iterations
    for _iter in 0..inner_iterations {
        let mut new_grid = grid.clone(); // Create a copy for next iteration
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] != ' ' { // If a cell has mycelium or a node
                    // Try to grow to neighbors
                    let neighbors = [
                        (0, 1), (0, -1), (1, 0), (-1, 0),
                        (1, 1), (1, -1), (-1, 1), (-1, -1),
                    ];

                    for (dx, dy) in &neighbors {
                        let nx = (x as isize + dx) as usize;
                        let ny = (y as isize + dy) as usize;

                        if nx < width && ny < height && new_grid[ny][nx] == ' ' {
                            // Growth probability increases with iteration
                            if rng.next_f64() < (0.05 + iteration as f64 * 0.008 * growth_rate).min(0.6) { 
                                if rng.next_f64() < (0.02 + iteration as f64 * 0.003 * growth_rate).min(0.2) { // Increased chance to place an emoji
                                    let emoji_str = EMOJIS[rng.gen_range(0..=EMOJIS.len() - 1)];
                                    new_grid[ny][nx] = emoji_str.chars().next().unwrap_or(' ');
                                } else {
                                    let char = MYCELIUM_CHARS[rng.gen_range(0..=MYCELIUM_CHARS.len() - 1)];
                                    new_grid[ny][nx] = char;
                                }

                                // Occasionally create new nodes, which can also be emojis
                                if rng.next_f64() < (0.005 + iteration as f64 * 0.001 * growth_rate).min(0.1) { // Increased probability for new nodes
                                    if rng.next_f64() < 0.5 { // 50% chance for node to be an emoji
                                        let emoji_str = EMOJIS[rng.gen_range(0..=EMOJIS.len() - 1)];
                                        new_grid[ny][nx] = emoji_str.chars().next().unwrap_or('O');
                                    } else {
                                        new_grid[ny][nx] = 'O';
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        grid = new_grid;
    }
    grid
}