pub mod cli;
use driftwood::netlify;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cli::draw_menu();
    netlify::get_site_details("56830fd5-ff33-438e-a0fd-2d68868cb2e6")?;
    Ok(())
}