use std::{
    fs,
    collections::HashMap,
};
use reqwest::{
    header,
    ClientBuilder,
    get,
};

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

/// TODO - make it so the request's response is a useful data object
pub struct netlify{

}

// Netlify API token:
// nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c

impl netlify {    
    #[tokio::main]
    pub async fn connect_to_api() -> Result<(), Box<dyn std::error::Error>>  {

        println!("Connecting to Netlify API...");
        
        // define the user agent
        static APP_USER_AGENT: &str = concat!(
            // env!("CARGO_PKG_NAME"),
            // "/",
            // env!("CARGO_PKG_VERSION"),
            "MyApp evan.robertson77@gmail.com"
        );

        // define the token
        static TOKEN: &str = "nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c";

        // define the base URL
        let base_url = String::from("https://api.netlify.com/api/v1/sites/56830fd5-ff33-438e-a0fd-2d68868cb2e6");

        println!("User agent: {}", APP_USER_AGENT);

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
        .text() // <- works but it returns stringified JSON, not a useful data object
        // .json::<HashMap<String, String>>()
        .await?;
        
        println!("{resp:#?}");
        println!("Connection closed.");
        Ok(())
    }
}