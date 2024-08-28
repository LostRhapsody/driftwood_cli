pub mod cli;
pub mod lib;
pub mod netlify;

use dotenv::dotenv;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    cli::draw_menu();
    Ok(())
}