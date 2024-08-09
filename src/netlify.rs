/// TODO - Retrieve all the user's sites and display their info in a list
/// That means the enum will need to be stored in a vector we can iterate over
/// TODO - Create a new site request
/// TODO - Update a site request
/// TODO - Shutdown a site request (just stop it, don't delete the whole thing)
/// TODO - Delete a site request
/// TODO: Update the Netlify lib so it uses OAuth2 instead of a token

use std::collections::HashMap;
use reqwest::ClientBuilder;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Value {
    String(String),
    Bool(bool),
    Usize(usize),
    Missing(serde::de::IgnoredAny),
    Null,
}

pub struct Netlify {
    user_agent: String,
    token: String,
    url: String,
}

pub struct SiteDetails {
    name: String,
    url: String,
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

        // define the token
        let token: &str = "nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c";

        // define the base URL
        let base_url: String = String::from("https://api.netlify.com/api/v1/");

        Netlify {
            user_agent: user_agent.to_string(),
            token: token.to_string(),
            url: base_url,
        }    

    }

    pub async fn get_site_details(netlify: Netlify, id: &str) -> Result<(), Box<dyn std::error::Error>>  {

        println!("Getting site info for site id: {}", id);
    
        let request_url = netlify.url + "sites/" + id;
    
        // create the builder and client
        let builder = ClientBuilder::new();                
        let client = builder
            .user_agent(netlify.user_agent)
            .build()
            .unwrap();
    
        // make the request passing in the token
        let resp = client.get(request_url)
            .bearer_auth(netlify.token)
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?;
        
        // extract the field name from the json
        let field_name = resp.get("name").unwrap();
        let field_url = resp.get("url").unwrap();
        println!("Site name: {:?}", field_name);
        println!("Site URL: {:?}", field_url);
        println!("Connection closed.");
        Ok(())
    }
    
    pub async fn get_sites() -> Result<(), Box<dyn std::error::Error>>  {    
    
        println!("Connecting to Netlify API...");
        let request_url = BASE_URL.to_owned() + "sites";
    
        // create the builder and client
        let builder = ClientBuilder::new();                
        let client = builder
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();
    
        // make the request passing in the token
        let resp = client.get(request_url)
            .bearer_auth(TOKEN)
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?;
    
        Ok(())
    }

}