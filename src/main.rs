pub mod cli;
pub mod netlify;

use netlify::{
    Netlify,
    SiteDetails
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cli::draw_menu();
    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");

    // Get all the sites
    let _ = get_sites(netlify).await;

    // Create a new site
    // let _ = add_site(netlify,String::from("awesome-test-site")).await;

    Ok(())
}

/// Get all the sites for the user
/// netlify: A Netlify instance
/// Returns a vector of SiteDetails
async fn get_sites(netlify: Netlify) -> Vec<SiteDetails> {
    match netlify.get_sites().await {
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

/// Add a new site
/// netlify: A Netlify instance
/// site_name: The name of the site to create
/// Returns a vector of SiteDetails
async fn add_site(netlify: Netlify, site_name: String) -> Vec<SiteDetails> {
    match netlify.add_site(site_name).await {
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