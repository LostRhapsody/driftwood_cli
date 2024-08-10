/// TODO - Retrieve all the user's sites and display their info in a list
/// That means the enum will need to be stored in a vector we can iterate over
/// TODO - Create a new site request
/// TODO - Update a site request
/// TODO - Shutdown a site request (just stop it, don't delete the whole thing)
/// TODO - Delete a site request
/// TODO: Update the Netlify lib so it uses OAuth2 instead of a token
use reqwest::ClientBuilder;

use serde::Deserialize;

pub struct Netlify {
    user_agent: String,
    token: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct SiteDetails {
    pub name: Option<String>,
    pub url: Option<String>,
    pub screenshot_url: Option<String>,
}

impl Netlify {

    /// Create a struct to store Netlify API connection details
    pub fn new(token: &str) -> Netlify {
        println!("Connecting to Netlify API...");
        // define the user agent
        let user_agent: &str = concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        );

        // define the base URL
        let base_url: String = String::from("https://api.netlify.com/api/v1/");

        Netlify {
            user_agent: user_agent.to_string(),
            token: token.to_string(),
            url: base_url,
        }    

    }

    pub async fn get_site_details(&self, id: &str) -> Result<Vec<SiteDetails>, Box<dyn std::error::Error>>  {

        println!("Getting site info for site id: {}", id);
    
        let request_url = self.url.clone() + "sites/" + id;
    
        // create the builder and client
        let builder = ClientBuilder::new();                
        let client = builder
            .user_agent(&self.user_agent)
            .build()
            .unwrap();

        println!("Client built.");

        let response = client
            .get(request_url)
            .bearer_auth(&self.token)
            .send()
            .await;

        match response {
            Ok(resp) => {
                // println!("Response: {:?}", resp);
                if resp.status().is_success() {
                    let sites: Vec<SiteDetails> = resp
                        .json()
                        .await?;
                    // println!("Sites deserialized: {:?}", sites);
                    return Ok(sites);
                } else {
                    println!("Failed to get sites: {}", resp.status());
                    return Err(format!("Failed to get sites: {}", resp.status()).into());
                }
            },
            Err(e) => {
                println!("Failed to get sites: {:?}", e);
                return Err(format!("Failed to get sites: {:?}", e).into());
            }
        }
    }
    
    pub async fn get_sites(&self) -> Result<Vec<SiteDetails>, Box<dyn std::error::Error>>  {    
    
        println!("Connecting to Netlify API...");
        let request_url = self.url.clone() + "sites";
        println!("Request URL: {:?}", request_url);
    
        // create the builder and client
        let builder = ClientBuilder::new();                
        let client = builder
            .user_agent(&self.user_agent)
            .build()
            .unwrap();

        println!("Client built.");

        let response = client
            .get(request_url)
            .bearer_auth(&self.token)
            .send()
            .await;

        match response {
            Ok(resp) => {
                // println!("Response: {:?}", resp);
                if resp.status().is_success() {
                    let sites: Vec<SiteDetails> = resp
                        .json()
                        .await?;
                    // println!("Sites deserialized: {:?}", sites);
                    return Ok(sites);
                } else {
                    println!("Failed to get sites: {}", resp.status());
                    return Err(format!("Failed to get sites: {}", resp.status()).into());
                }
            },
            Err(e) => {
                println!("Failed to get sites: {:?}", e);
                return Err(format!("Failed to get sites: {:?}", e).into());
            }
        }
    }

}