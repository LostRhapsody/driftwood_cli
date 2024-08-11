/// Netlify Module
/// Used to interact with the Netlify API

/// TODO - Add site request works but name does not get set for new site, probably bad JSON
/// TODO - Update a site request
/// TODO - Shutdown a site request (just stop it, don't delete the whole thing)
/// TODO - Delete a site request
/// TODO: Update the Netlify lib so it uses OAuth2 instead of a token
use serde::Deserialize;

/// Netlify struct
/// Contains the user agent, token, and base URL for the Netlify API
pub struct Netlify {
    user_agent: String,
    token: String,
    url: String,
}

/// SiteDetails struct
/// Contains the details of a site
/// name: The name of the site
/// url: The URL of the site
/// screenshot_url: The URL of the site's screenshot
#[derive(Debug, Deserialize)]
pub struct SiteDetails {
    pub name: Option<String>,
    pub url: Option<String>,
    pub screenshot_url: Option<String>,
}

impl Netlify {
    /// Create a struct to store Netlify API connection details
    /// token: The Netlify API token
    /// Returns a Netlify struct
    pub fn new(token: &str) -> Netlify {
        println!("> Creating Netlify API Struct");

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

    /// Get the details of a site
    /// id: The ID of the site
    /// Returns a Result containing a vector of SiteDetails or an error
    pub async fn get_site_details(
        &self,
        id: &str,
    ) -> Result<Vec<SiteDetails>, Box<dyn std::error::Error>> {

        println!("> Getting site details for: {}", id);
        
        // create the url
        let request_url = self.url.clone() + "sites/" + id;
        // build and send the request
        let client = self.build_client();
        let response = self.send_get_request(client, request_url).await;
        // return the response
        self.read_array_response(response).await

    }

    /// Get all the sites for the user
    /// Returns a Result containing a vector of SiteDetails or an error
    pub async fn get_sites(
        &self
    ) -> Result<Vec<SiteDetails>, Box<dyn std::error::Error>> {

        println!("> Getting all site details");

        // create the url
        let request_url = self.url.clone() + "sites";
        // build and send the request
        let client = self.build_client();
        let response = self.send_get_request(client, request_url).await;
        // return the response
        self.read_array_response(response).await

    }

    /// Add a new site
    /// Returns a Result containing a vector of SiteDetails or an error
    pub async fn create_site(
        &self, 
        site_name: String
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {

        println!("> Creating site: {}", site_name);

        // create the url
        let request_url = self.url.clone() + "sites";
        // create the request body
        let json = serde_json::json!(
        {
            "name": site_name,
            "ssl": true,
            "processing_settings": {
                "html": {
                    "pretty_urls": true
                },
            },
        });
        println!("JSON: {}", json.to_string());
        // build and send the request
        let client = self.build_client();
        let response = self.send_post_request(
            client, 
            request_url, 
            json.to_string()
        ).await;
        // return the response
        self.read_object_response(response).await
    }

    /// Create a reqwest::Client
    /// Returns a reqwest::Client
    fn build_client(&self) -> reqwest::Client {
        println!("> Building Client...");
        let builder = reqwest::ClientBuilder::new();
        let client = builder.user_agent(&self.user_agent).build().unwrap();
        client
    }

    /// Send a request to the Netlify API
    /// client: The reqwest::Client to use
    /// request_url: The URL to send the request to
    /// Returns a Result containing a reqwest::Response or an error
    async fn send_get_request(
        &self,
        client: reqwest::Client,
        request_url: String,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        println!("> Sending GET request to: {}", request_url);
        let response = client
            .get(request_url)
            .bearer_auth(&self.token)
            .send()
            .await?;
        Ok(response)
    }

    async fn send_post_request(
        &self,
        client: reqwest::Client,
        request_url: String,
        json: String,
    ) -> Result<reqwest::Response, Box<dyn std::error::Error>> {
        println!("> Sending POST request to: {}", request_url);        
        let response = client
            .post(request_url)
            .bearer_auth(&self.token)
            .header("Content-Type", "application/json;UTF-8")
            .body(json)
            .send()
            .await?;

        Ok(response)
    }    

    /// Read the response from the Netlify API (array)
    /// response: The response from the Netlify API
    /// Returns a Result containing a vector of SiteDetails or an error
    async fn read_array_response(
        &self,
        response: Result<reqwest::Response, Box<dyn std::error::Error>>,
    ) -> Result<Vec<SiteDetails>, Box<dyn std::error::Error>> {
        println!("> Reading Response (array)...");

        match response {
            Ok(resp) => {

                // println!("Response: {:?}", resp);
                
                if resp.status().is_success() {

                    let json: serde_json::Value = resp.json().await?;
                    let sites: Vec<SiteDetails> = serde_json::from_value(json)?;
                    Ok(sites)

                } else {
                    println!("> Request failed: {}", resp.status());
                    return Err(
                        format!(
                            "> Request failed: {}", 
                            resp.status()
                        ).into()
                    );
                }
            }
            Err(e) => {
                println!("> Request failed: {:?}", e);
                return Err(format!("> Request failed: {:?}", e).into());
            }
        }
    }

    /// Read the response from the Netlify API (single object)
    /// response: The response from the Netlify API
    /// Returns a Result containing a vector of SiteDetails or an error
    async fn read_object_response(
        &self,
        response: Result<reqwest::Response, Box<dyn std::error::Error>>,
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {
        println!("> Reading Response (object)...");

        match response {
            Ok(resp) => {

                if resp.status().is_success() {

                    let json: serde_json::Value = resp.json().await?;
                    let sites: SiteDetails = serde_json::from_value(json)?;
                    Ok(sites)

                } else {
                    println!("> Request failed: {}", resp.status());
                    return Err(
                        format!(
                            "> Request failed: {}", 
                            resp.status()
                        ).into()
                    );
                }
            }
            Err(e) => {
                println!("> Request failed: {:?}", e);
                return Err(format!("> Request failed: {:?}", e).into());
            }
        }
    }
}