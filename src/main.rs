pub mod cli;
pub mod netlify;

use netlify::{
    Netlify,
    SiteDetails,
};

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

/// Get all the sites for the user
/// netlify: A Netlify instance
/// Returns a vector of SiteDetails
fn get_sites(netlify: Netlify) -> Vec<SiteDetails> {
    match netlify.get_sites() {
        Ok(sites) => {
            println!("Done");
            for each in &sites {
                println!("\nSite Details:");
                println!("{:?}", each);
            }
            sites
        }
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    }
}