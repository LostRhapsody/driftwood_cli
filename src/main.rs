pub mod cli;
pub mod netlify;

use netlify::{
    Netlify,
    SiteDetails
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cli::draw_menu();
    println!("new netlify");
    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    println!("new netlify done");
    let _ = get_sites(netlify).await;
    // let _sites = get_sites(netlify);

    // let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    // get_site_details(netlify,"56830fd5-ff33-438e-a0fd-2d68868cb2e6");
    Ok(())
}

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

async fn get_site_details(netlify: Netlify, id: &str) {
    Netlify::get_site_details(
    &netlify,
    id
    )
    .await
    .expect("Failed to get site details");
}