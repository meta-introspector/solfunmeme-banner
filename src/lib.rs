pub mod constants;
pub mod mycelium_pattern;
pub mod banner_generator;
pub mod random_stream;

pub use banner_generator::generate_solfunmeme_banner;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_solfunmeme_banner() {
        let git_hash = "a1b2c3d4e5f67890abcdef1234567890";
        let banner = generate_solfunmeme_banner(git_hash);
        println!("{}", banner);
        assert!(!banner.is_empty());
        assert!(banner.contains("SOLFUNMEME"));
    }
}
