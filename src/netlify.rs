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

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Value::String(s) => s.clone(),
            Value::Bool(b) => b.to_string(),
            Value::Usize(u) => u.to_string(),
            Value::Missing(_) => String::new(),
            Value::Null => String::new(),
        }
    }
}

pub struct Netlify {
    user_agent: String,
    token: String,
    url: String,
}

pub struct SiteDetails {
    pub name: String,
    pub url: String,
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

    pub async fn get_site_details(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>  {

        println!("Getting site info for site id: {}", id);
    
        let request_url = self.url.clone() + "sites/" + id;
    
        // create the builder and client
        let builder = ClientBuilder::new();                
        let client = builder
            .user_agent(&self.user_agent)
            .build()
            .unwrap();
    
        // make the request passing in the token
        let resp = client.get(request_url)
            .bearer_auth(&self.token)
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
    
        // make the request passing in the token
        let resp = client.get(request_url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?;

        println!("Response received.");

        // loop through the response and store certain fields in vector that make up the details of a 'site' struct
        let mut sites: Vec<SiteDetails> = Vec::new();
        for (key, value) in resp.iter() {
            println!("Key: {:?}, Value: {:?}", key, value);
            
            let mut name = String::new();
            let mut url = String::new();

            if key.to_string() == "name" {
                name = value.to_string();
            } else if key.to_string() == "url" {
                url = value.to_string();
            }

            if name == "" || url == "" {
                continue;
            }

            let site = SiteDetails {
                name: name,
                url: url,
            };

            println!("Site name: {:?}", site.name);

            sites.push(site);
        }

        Ok(sites)
    }

}