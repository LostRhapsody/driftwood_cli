pub mod cli;
pub mod netlify;

use netlify::{
    Netlify,
    SiteDetails
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cli::draw_menu();
    println!("new netlify");
    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    println!("new netlify done");
    let _ = Netlify::get_sites(&netlify);
    // let _sites = get_sites(netlify);

    // let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    // get_site_details(netlify,"56830fd5-ff33-438e-a0fd-2d68868cb2e6");
    Ok(())
}

async fn get_sites(netlify: Netlify) -> Vec<SiteDetails> {
    println!("Getting all sites...");
    let sites: Vec<SiteDetails> = 
        Netlify::get_sites(&netlify)
        .await
        .expect("Failed to get all sites");
    println!("Done");

    for each in &sites {
        println!("{}", each.name);
    }
    
    sites
}

async fn get_site_details(netlify: Netlify, id: &str) {
    Netlify::get_site_details(
    &netlify,
    id
    )
    .await
    .expect("Failed to get site details");
}