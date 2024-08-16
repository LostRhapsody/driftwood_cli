pub mod cli;
pub mod netlify;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  
    cli::draw_menu();
    Ok(())
}