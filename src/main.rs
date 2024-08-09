pub mod cli;
use driftwood::netlify;

/// TODO: Update the Netlify lib so it uses OAuth2 instead of a token

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cli::draw_menu();
    netlify::connect_to_api()?;
    Ok(())
}