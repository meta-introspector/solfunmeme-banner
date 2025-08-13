use solfunmeme_banner::generate_solfunmeme_banner;
use std::time::Duration;
use std::thread;
use clap::Parser;
use std::fs::File;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Git hash to use for deterministic generation
    #[arg(short = 's', long, default_value = "a1b2c3d4e5f67890abcdef1234567890")]
    git_hash: String,

    /// Number of animation frames
    #[arg(short, long, default_value_t = 60)]
    frames: usize,

    /// Delay between frames in milliseconds
    #[arg(short, long, default_value_t = 200)]
    delay: u64,

    /// Growth rate multiplier (e.g., 1.0 for normal, 0.5 for slower, 2.0 for faster)
    #[arg(short = 'r', long, default_value_t = 1.0)]
    growth_rate: f64,

    /// Output file to write ANSI animation to (instead of stdout)
    #[arg(short, long)]
    output_file: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let mut output_buffer = String::new();

    for i in 0..cli.frames {
        // Clear screen and move cursor to home for flipbook effect
        let frame_prefix = format!("\x1b[2J\x1b[H");
        output_buffer.push_str(&frame_prefix);

        let banner = generate_solfunmeme_banner(&cli.git_hash, i, cli.growth_rate);
        output_buffer.push_str(&banner);

        // Add a small delay for visual effect (only if outputting to stdout)
        if cli.output_file.is_none() {
            thread::sleep(Duration::from_millis(cli.delay));
        }
    }

    if let Some(output_path) = cli.output_file {
        let mut file = File::create(output_path).expect("Could not create output file");
        file.write_all(output_buffer.as_bytes()).expect("Could not write to output file");
    } else {
        print!("{}", output_buffer);
    }
}
