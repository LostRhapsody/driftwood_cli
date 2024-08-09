use std::{
    fs,
    collections::HashMap,
};
use reqwest::{
    header,
    ClientBuilder,
    get,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Value {
    String(String),
    bool(bool),
    usize(usize),
    missing(serde::de::IgnoredAny),
    null,
}

pub struct Post {
    pub title: String,
    date: String,
    content: String,
    pub filename: String,
}

impl Post {
    pub fn new(title: String, date: String, content: String, filename: String) 
    -> Post {
        Post {
            title: title,
            date: date,
            content: content,
            filename: filename,
        }
    }

    pub fn read_and_parse(md_filename: &str, html_filename: &str) -> bool {
        let md_input = fs::read_to_string(md_filename)
            .expect("Couldn't find markdown file");
        let parser = pulldown_cmark::Parser::new(&md_input);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        // output the new file to disk
        fs::write(html_filename, &html_output)
            .expect("Couldn't write to output file");
        true
    }

}

/// TODO - Retrieve all the user's sites and display their info in a list
/// That means the enum will need to be stored in a vector we can iterate over
/// TODO - Create a new site request
/// TODO - Update a site request
/// TODO - Shutdown a site request (just stop it, don't delete the whole thing)
/// TODO - Delete a site request
/// TODO: Update the Netlify lib so it uses OAuth2 instead of a token
pub struct netlify{

}

// Netlify API token:
// nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c

impl netlify {    
    #[tokio::main]
    pub async fn get_site_details(id: &str) -> Result<(), Box<dyn std::error::Error>>  {

        println!("Connecting to Netlify API...");
        println!("Getting site info for site id: {}", id);
        
        // define the user agent
        static APP_USER_AGENT: &str = concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        );

        // define the token
        static TOKEN: &str = "nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c";

        // define the base URL
        let base_url = String::from("https://api.netlify.com/api/v1/sites/") + id;

        // create the builder and client
        let builder = ClientBuilder::new();                
        let client = builder
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        // make the request passing in the token
        let resp = client.get(base_url)
            .bearer_auth(TOKEN)
            .send()
            .await?
            // .text() // <- works but it returns stringified JSON, not a useful data object
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
}