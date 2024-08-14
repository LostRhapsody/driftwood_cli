pub mod cli;
pub mod netlify;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  
    cli::draw_menu();
    // let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");

    // vv easy netlify tests
    // Get all the sites
    // let _ = get_sites(netlify);
    // Create a new site
    // let _ = create_site(netlify,String::from("testSite25"));

    Ok(())
}