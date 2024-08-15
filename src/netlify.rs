/// Netlify Module
/// Used to interact with the Netlify API

/// TODO - Most Important!!! Place .md files in md_posts instead of /posts
///        When we deploy the site, convert the .md to .html and place in /posts
///        continue on with the process, generating hashes and sending those to Netlify.

/// TODO - Update the Netlify lib so it uses OAuth2 instead of a token
/// TODO - Add publish site/push site content function
/// TODO - Add convert markdown to html and populate template function
/// TODO - Test provisioning an SSL certificate
use serde::{
    Deserialize,
    Serialize,
};

use std::{collections::HashMap, fs, path::Path};

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
#[derive(Debug, Deserialize,Serialize,Clone)]
pub struct SiteDetails {
    pub name: Option<String>,
    pub id: Option<String>,
    pub ssl: Option<bool>,
    pub url: Option<String>,
    pub screenshot_url: Option<String>,
    pub required: Option<Vec<String>>,
}

/// FileHashes struct
/// Contains the path and SHA1 hash of a file
#[derive(Serialize, Deserialize, Debug)]
pub struct FileHashes {
    files: HashMap<String, String>,
}

/// Ssl_Cert struct
/// Contains the details of an SSL certificate
/// Fields match Netlify's API for provisioning an SSL certificate
/// cert: The SSL certificate
/// key: The SSL certificate key
/// ca_cert: The SSL certificate CA
pub struct Ssl_Cert {
    pub cert: Option<String>,
    pub key: Option<String>,
    pub ca_cert: Option<String>,
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
        // test URL, sends an echo to a test endpoint and responds with the request data        
        // let base_url: String = String::from("https://echo.free.beeceptor.com");

        Netlify {
            user_agent: user_agent.to_string(),
            token: token.to_string(),
            url: base_url,
        }
    }

    /// Get the details of a site
    /// id: The ID of the site
    /// Returns a Result containing a vector of SiteDetails or an error
    pub fn get_site_details(
        &self,
        id: &str,
    ) -> Result<Vec<SiteDetails>, Box<dyn std::error::Error>> {

        println!("> Getting site details for: {}", id);
        
        // create the url
        let request_url = self.url.clone() + "sites/" + id;
        // build and send the request
        let client = self.build_client();
        let response = self.send_get_request(client, request_url);
        // return the response
        self.read_array_response(response)

    }

    /// Get all the sites for the user
    /// Returns a Result containing a vector of SiteDetails or an error
    pub fn get_sites(
        &self
    ) -> Result<Vec<SiteDetails>, Box<dyn std::error::Error>> {

        println!("> Getting all site details");

        // create the url
        let request_url = self.url.clone() + "sites";
        // build and send the request
        let client = self.build_client();
        let response = self.send_get_request(client, request_url);
        // return the response
        self.read_array_response(response)

    }

    /// Add a new site
    /// Returns a Result containing a vector of SiteDetails or an error
    pub fn create_site(
        &self, 
        new_site: SiteDetails
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {

        println!("> Creating site: {}", new_site.name.clone().unwrap());

        // create the url
        let request_url = self.url.clone() + "sites";

        // create the request body
        let json = serde_json::to_value(new_site)?;

        // build and send the request
        let client = self.build_client();
        let response = self.send_post_request(
            client, 
            request_url, 
            json
        );

        // return the response
        self.read_object_response(response)
    }

    /// Update an existing site
    /// Returns a Result containing a vector of the new SiteDetails or an error
    pub fn update_site(
        &self,
        existing_site_details: SiteDetails,
        new_site_details: SiteDetails,
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {

        println!(
            "> Updating site: {}", 
            existing_site_details.name.clone().unwrap()
        );

        // create the url
        let request_url = 
            self.url.clone() + 
            "sites/" + 
            existing_site_details.id.clone().unwrap().as_str();

        // serialize the new_site_details into a serde_json::Value
        let json = serde_json::to_value(new_site_details)?;

        // build and send the request
        let client = self.build_client();
        let response = self.send_patch_request(
            client, 
            request_url, 
            json
        );

        // return the response
        self.read_object_response(response)
    }

    /// Update an existing site
    /// Returns a Result containing a vector of the new SiteDetails or an error
    pub fn delete_site(
        &self,
        site_details: SiteDetails,
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {

        println!(
            "> Deleting site: {}", 
            site_details.name.clone().unwrap()
        );

        // create the url
        let request_url = 
            self.url.clone() + 
            "sites/" + 
            site_details.id.clone().unwrap().as_str();

        // build and send the request
        let client = self.build_client();
        let response = self.send_delete_request(
            client, 
            request_url, 
            serde_json::Value::Null
        );

        // return the response
        self.read_object_response(response)
    }

    /// Send a list of files to the Netlify API
    /// site_details: A SiteDetails struct containing the site ID
    /// file_hashes: A FileHashes struct containing the path and SHA1 hash of a file
    /// Returns a Result containing a vector of SiteDetails 
    /// with the checksums for the required files in a 'required' array
    pub fn send_file_checksums(
        &self,
        site_details: SiteDetails,
        file_hashes: FileHashes,
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {

        // create the url
        let request_url = format!(
            "{}sites/{}/deploys", 
            self.url, 
            site_details.id.unwrap()
        );

        // build and send the request
        let client = self.build_client();
        let response = self.send_post_request(
            client, 
            request_url, 
            serde_json::to_value(file_hashes)?
        );

        // return the response
        self.read_object_response(response)
    }

    /// Provision an SSL certificate for a site
    /// # Note - Unstable
    /// This function is untested and may not work as expected
    /// Why would you want to provision a new SSL anyway?
    pub fn provision_ssl(
        &self,
        site_details: SiteDetails,
        ssl_details: Ssl_Cert,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        println!("> Creating SSL certificate for: {}", site_details.name.clone().unwrap());

        let request_url = 
            self.url.clone() + 
            "sites/" + 
            site_details.id.unwrap().as_str() + 
            "/ssl?certificate=" + 
            ssl_details.cert.unwrap().as_str() +
            "&key=" +
            ssl_details.key.unwrap().as_str() +
            "&ca_certificates=" +
            ssl_details.ca_cert.unwrap().as_str();

        let client = self.build_client();

        // despite being a POST request, doesn't need a body.
        let response = self.send_post_request(
            client, 
            request_url, 
            serde_json::Value::Null
        );

        match response {
            Ok(resp) => {

                if resp.status().is_success() {

                    let json: serde_json::Value = resp.json()?;
                    println!("{}", json);
                    return Ok(true)

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

    /// Create a reqwest::Client
    /// Returns a reqwest::Client
    fn build_client(&self) -> reqwest::blocking::Client {

        println!("> Building Builder...");
        let builder = reqwest::blocking::ClientBuilder::new();
        println!("> Building Client...");
        let client = builder.user_agent(&self.user_agent).build().unwrap();
        println!("> Done building client...");
        client
        
    }

    /// Send a request to the Netlify API
    /// client: The reqwest::Client to use
    /// request_url: The URL to send the request to
    /// Returns a Result containing a reqwest::Response or an error
    fn send_get_request(
        &self,
        client: reqwest::blocking::Client,
        request_url: String,
    ) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
        
        println!("> Sending GET request to: {}", request_url);

        let response = client
            .get(request_url)
            .bearer_auth(&self.token)
            .send()?;
        
        Ok(response)
    
    }

    /// Send a POST request to the Netlify API
    /// client: The reqwest::Client to use
    /// request_url: The URL to send the request to
    /// json: The JSON to send in the request
    /// Returns a Result containing a reqwest::Response or an error
    fn send_post_request(
        &self,
        client: reqwest::blocking::Client,
        request_url: String,
        json: serde_json::Value,
    ) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
        
        println!("> Sending POST request to: {}", request_url);     

        let request = client
            .post(request_url)
            .bearer_auth(&self.token)
            .json(&json)
            .headers(Netlify::build_request_headers());

        let response = request.send()?;        

        Ok(response)
    
    }    

    /// Send a PATCH request to the Netlify API
    /// client: The reqwest::Client to use
    /// request_url: The URL to send the request to
    /// json: The JSON to send in the request
    /// Returns a Result containing a reqwest::Response or an error
    fn send_patch_request(
        &self,
        client: reqwest::blocking::Client,
        request_url: String,
        json: serde_json::Value,
    ) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
        
        println!("> Sending PATCH request to: {}", request_url);     

        let request = client
            .patch(request_url)
            .bearer_auth(&self.token)
            .json(&json)
            .headers(Netlify::build_request_headers());

        let response = request.send()?;        

        Ok(response)
    
    }   

    /// Send a DELETE request to the Netlify API
    /// client: The reqwest::Client to use
    /// request_url: The URL to send the request to
    /// json: The JSON to send in the request
    /// Returns a Result containing a reqwest::Response or an error
    fn send_delete_request(
        &self,
        client: reqwest::blocking::Client,
        request_url: String,
        json: serde_json::Value,
    ) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
        
        println!("> Sending DELETE request to: {}", request_url);     

        let request = client
            .delete(request_url)
            .bearer_auth(&self.token)
            .json(&json)
            .headers(Netlify::build_request_headers());

        let response = request.send()?;        

        Ok(response)
    
    } 

    /// Read the response from the Netlify API (array)
    /// response: The response from the Netlify API
    /// Returns a Result containing a vector of SiteDetails or an error
    fn read_array_response(
        &self,
        response: Result<reqwest::blocking::Response, Box<dyn std::error::Error>>,
    ) -> Result<Vec<SiteDetails>, Box<dyn std::error::Error>> {
        println!("> Reading Response (array)...");

        match response {
            Ok(resp) => {

                // println!("Response: {:?}", resp);
                
                if resp.status().is_success() {

                    let json: serde_json::Value = resp.json()?;
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
    fn read_object_response(
        &self,
        response: Result<reqwest::blocking::Response, Box<dyn std::error::Error>>,
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {
        println!("> Reading Response (object)...");        

        match response {
            Ok(resp) => {

                if resp.status() == 422 {
                    println!("> Site name provided is not unique.");
                    println!("> Request failed with a status of 422.");
                    println!(concat!("> !!!Note: 422 means 'unprocessable ",
                    "entity', but it could just be your site name is already ",
                    "being used. Try a different, more unique name.!!!"
                    ));
                }

                if resp.status().is_success() {

                    let json: serde_json::Value = resp.json()?;
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

    /// Build the headers for the POST request, specifically create_site
    fn build_request_headers() -> reqwest::header::HeaderMap {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
        headers
    }

    /// Reads in all files in a site's posts directory and generates SHA1 hashes
    /// Returns a FileHashes struct containing the path and SHA1 hash of a file
    pub fn generate_sha1_for_posts(
        site_path: &Path,
        posts_dir: &Path
    ) -> Result<FileHashes, Box<dyn std::error::Error>> {
        println!("> Generating SHA1 hashes for posts...");
        println!("> Posts directory: {:?}", posts_dir);

        let mut file_hashes = FileHashes {
            files: HashMap::new(),
        };

        let mut sha1 = sha1_smol::Sha1::new();

        // first grab the hash for /site/index.html
        let index_file = fs::read(
            format!("{}/index.html",site_path.display())
        )?;

        sha1.update(&index_file);
        file_hashes.files.insert(
            "index.html".to_string(), 
            sha1.digest().to_string()
        );
        sha1.reset();

        // loop through the files in this dir
        for entry in fs::read_dir(posts_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
                // provide sha1.update with the file
                let file = fs::read(path)?;
                sha1.update(&file);
                file_hashes.files.insert(
                    format!("/posts/{}", file_name), 
                    sha1.digest().to_string()
                );
                sha1.reset();
            }
        }
        
        println!("{:?}",file_hashes);

        Ok(file_hashes)
    }
}