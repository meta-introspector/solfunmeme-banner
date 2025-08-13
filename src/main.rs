use solfunmeme_banner::generate_solfunmeme_banner;
use std::time::Duration;
use std::thread;

fn main() {
    let git_hash = "a1b2c3d4e5f67890abcdef1234567890"; 

    for i in 0..60 { // Generate 60 frames for the movie
        // Clear screen and move cursor to home for flipbook effect
        print!("\x1b[2J\x1b[H");

        let banner = generate_solfunmeme_banner(git_hash, i); // Pass iteration
        println!("{}", banner);

        // Add a small delay for visual effect
        thread::sleep(Duration::from_millis(200)); // 200ms delay per frame
    }
}
