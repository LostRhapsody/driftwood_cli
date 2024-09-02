use crate::crypto;
/// TODO - Create a new server host to run the authentication logic through
/// TODO - refresh token
///
use driftwood::OAuth2;
use driftwood::SiteDetails;
use rsa::RsaPrivateKey;

use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, TokenResponse, TokenUrl,
};
use reqwest::Url;
/// Netlify Module
/// Used to interact with the Netlify API
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    path::Path,
};
use webbrowser;

/// Netlify struct
/// Contains the user agent, token, and base URL for the Netlify API
pub struct Netlify {
    user_agent: String,
    token: String,
    url: String,
}

/// FileHashes struct
/// Contains the path and SHA1 hash of a file
#[derive(Serialize, Deserialize, Debug)]
pub struct FileHashes {
    pub files: HashMap<String, String>,
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
    /// checks for token file, if none exists, gets a new token from oauth2 flow
    /// Returns a Netlify struct
    pub fn new() -> Netlify {
        println!("> Creating Netlify API Struct");

        // define the user agent
        let user_agent: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

        // define the base URL
        let base_url: String = String::from(
            OAuth2::get_env_var("NETLIFY_BASE_URL")
                .expect("Failed to get NETLIFY_BASE_URL from .env file"),
        );

        // first check if there is a token on disk
        // if not, get a new token
        let token_file = Path::new("netlify_token.json");
        if token_file.exists() {
            println!("> Token file exists");
            let token = fs::read_to_string(token_file).unwrap();
            Netlify {
                user_agent: user_agent.to_string(),
                token: token,
                url: base_url,
            }
        } else {
            println!("> Token file does not exist");
            let (code, state, private_key) = Self::login().expect("Failed to trigger login");
            let token = Self::exchange_code_for_token(code, state, private_key)
                .expect("Failed to exchange code for token");
            fs::write(token_file, token.as_bytes()).unwrap();
            Netlify {
                user_agent: user_agent.to_string(),
                token: token,
                url: base_url,
            }
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
    pub fn get_sites(&self) -> Result<Vec<SiteDetails>, Box<dyn std::error::Error>> {
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
        new_site: SiteDetails,
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {
        println!("> Creating site: {}", new_site.name.clone().unwrap());

        // create the url
        let request_url = self.url.clone() + "sites";

        // create the request body
        let json = serde_json::to_value(new_site)?;

        // build and send the request
        let client = self.build_client();
        let response = self.send_post_request(client, request_url, json);

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
            self.url.clone() + "sites/" + existing_site_details.id.clone().unwrap().as_str();

        // serialize the new_site_details into a serde_json::Value
        let json = serde_json::to_value(new_site_details)?;

        // build and send the request
        let client = self.build_client();
        let response = self.send_patch_request(client, request_url, json);

        // return the response
        self.read_object_response(response)
    }

    /// Update an existing site
    /// Returns a Result containing a vector of the new SiteDetails or an error
    pub fn delete_site(
        &self,
        site_details: SiteDetails,
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {
        println!("> Deleting site: {}", site_details.name.clone().unwrap());

        // create the url
        let request_url = self.url.clone() + "sites/" + site_details.id.clone().unwrap().as_str();

        // build and send the request
        let client = self.build_client();
        let response = self.send_delete_request(client, request_url, serde_json::Value::Null);

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
        file_hashes: &FileHashes,
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {
        // create the url
        let request_url = format!("{}sites/{}/deploys", self.url, site_details.id.unwrap());

        // build and send the request
        let client = self.build_client();
        let response =
            self.send_post_request(client, request_url, serde_json::to_value(file_hashes)?);

        // return the response
        self.read_object_response(response)
    }

    pub fn upload_file(
        &self,
        // site_details: SiteDetails,
        site_name: String,
        site_id: String,
        deploy_id: String,
        file_path: &Path,
    ) -> Result<SiteDetails, Box<dyn std::error::Error>> {
        // create the url
        let request_url = format!(
            "{}deploys/{}/files{}",
            self.url,
            deploy_id,
            file_path.display()
        );

        println!("> request URL: {}", request_url);
        println!("> File path: {}", file_path.display());

        let full_path_str: String;
        let mut full_path = Path::new("");

        // if the file is /index.html, full path will be sitename_siteid/index.html
        if file_path.to_string_lossy() == "/index.html" {
            println!("> File is index.html");
            full_path_str = format!("sites/{}_{}/index.html", site_name, site_id);
            full_path = Path::new(&full_path_str);
        } else {
            println!("> File is not index.html");
            full_path_str = format!("sites/{}_{}{}", site_name, site_id, file_path.display());
            full_path = Path::new(&full_path_str);
        }

        println!("> Full path: {}", full_path.display());

        // confirm full_path exists
        if !full_path.exists() {
            return Err(format!("> {} not found", full_path.display()).into());
        }

        let file = File::open(full_path)?;

        // build and send the request
        let client = self.build_client();
        let response = self.send_put_request(client, request_url, file);

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
        println!(
            "> Creating SSL certificate for: {}",
            site_details.name.clone().unwrap()
        );

        let request_url = self.url.clone()
            + "sites/"
            + site_details.id.unwrap().as_str()
            + "/ssl?certificate="
            + ssl_details.cert.unwrap().as_str()
            + "&key="
            + ssl_details.key.unwrap().as_str()
            + "&ca_certificates="
            + ssl_details.ca_cert.unwrap().as_str();

        let client = self.build_client();

        // despite being a POST request, doesn't need a body.
        let response = self.send_post_request(client, request_url, serde_json::Value::Null);

        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let json: serde_json::Value = resp.json()?;
                    println!("{}", json);
                    return Ok(true);
                } else {
                    println!("> Request failed: {}", resp.status());
                    return Err(format!("> Request failed: {}", resp.status()).into());
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

        let response = client.get(request_url).bearer_auth(&self.token).send()?;

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

    /// Send a PUT request to the Netlify API
    /// client: The reqwest::Client to use
    /// request_url: The URL to send the request to
    /// json: The JSON to send in the request
    /// Returns a Result containing a reqwest::Response or an error
    fn send_put_request(
        &self,
        client: reqwest::blocking::Client,
        request_url: String,
        file: File,
    ) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
        println!("> Sending PUT request to: {}", request_url);

        let request = client
            .put(request_url)
            .bearer_auth(&self.token)
            .body(file)
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
                    return Err(format!("> Request failed: {}", resp.status()).into());
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
                    println!(concat!(
                        "> !!!Note: 422 means 'unprocessable ",
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
                    return Err(format!("> Request failed: {}", resp.status()).into());
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
        posts_dir: &Path,
    ) -> Result<FileHashes, Box<dyn std::error::Error>> {
        println!("> Generating SHA1 hashes for posts...");
        println!("> Posts directory: {:?}", posts_dir);

        let mut file_hashes = FileHashes {
            files: HashMap::new(),
        };

        let mut sha1 = sha1_smol::Sha1::new();

        // ensure the index.html file exists
        if !Path::new(&format!("{}/index.html", site_path.display())).exists() {
            return Err(format!("> index.html not found in {}", site_path.display()).into());
        }

        // first grab the hash for /site/index.html
        let index_file = fs::read(format!("{}/index.html", site_path.display()))?;

        sha1.update(&index_file);
        file_hashes
            .files
            .insert("/index.html".to_string(), sha1.digest().to_string());
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
                file_hashes
                    .files
                    .insert(format!("/posts/{}", file_name), sha1.digest().to_string());
                sha1.reset();
            }
        }

        println!("{:?}", file_hashes);

        Ok(file_hashes)
    }

    pub fn login() -> Result<(String, String, rsa::RsaPrivateKey), Box<dyn std::error::Error>> {
        println!("> Logging in...");

        let (private_key, public_key) = crypto::generate_key_pair();
        let public_key_pem = crypto::get_public_key_pem(&public_key);
        let encoded_public_key = urlencoding::encode(&public_key_pem);

        let auth_url = format!(
            "https://auth.driftwoodapp.com/login?public_key_pem={}",
            encoded_public_key
        );

        let listener = TcpListener::bind("127.0.0.1:8000")?;

        // Open the authorization URL in the user's browser
        webbrowser::open(&auth_url)?;

        for stream in listener.incoming() {
            let mut stream = stream?;
            let mut buffer = [0; 1024];
            stream.read(&mut buffer)?;

            let request = String::from_utf8_lossy(&buffer);
            let url = request
                .lines()
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap();
            let url = Url::parse(&format!("http://localhost{}", url))?;

            println!("> URL: {}", url);

            let mut code = None;
            let mut state = None;

            for (key, value) in url.query_pairs() {
                match key.as_ref() {
                    "code" => code = Some(value.into()),
                    "state" => state = Some(value.into_owned()),
                    _ => {}
                }
            }

            if let (Some(code), Some(state)) = (code, state) {
                println!("> Authorization code: {}", code);
                println!("> Authorization state: {}", state);

                // Send "You can close this screen now" message
                let file_contents = fs::read_to_string("src/templates/auth/code_received.html")
                    .expect("Should have been able to read the file");

                // Create the HTTP response
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: {}\r\n\r\n{}",
                    file_contents.len(),
                    file_contents
                );

                stream.write_all(response.as_bytes())?;
                stream.flush()?;

                return Ok((code, state, private_key));
            }
        }

        Err("No authorization code received".into())
    }

    pub fn exchange_code_for_token(
        code: String,
        state: String,
        private_key: RsaPrivateKey,
    ) -> Result<String, Box<dyn std::error::Error>> {
        println!("> Exchanging code for token...");
        println!("> Code: {}", code);

        let client = reqwest::blocking::Client::new();

        let response = client
            .get(format!(
                "https://auth.driftwoodapp.com/callback?code={}&state={}",
                code, state
            ))
            .send()?;

        println!("> {:?}", response);

        let token_response: serde_json::Value = response.json()?;

        println!("> Token response: {:?}", token_response);

        let token: String = token_response["token"].as_str().unwrap().to_string();

        println!("> Encrypted token: {}", token);

        let token = crypto::decrypt_token(&token, &private_key);
        println!("> Token: {}", token);
        Ok(token)
    }

    // pub fn trigger_login() -> Result<(), Box<dyn std::error::Error>> {
    //     println!("> Triggering login...");

    //     let (private_key, public_key) = crypto::generate_key_pair();
    //     let public_key_pem = crypto::get_public_key_pem(&public_key);
    //     let encoded_public_key = urlencoding::encode(&public_key_pem);

    //     let auth_url = format!("https://auth.driftwoodapp.com/login?public_key_pem={}", encoded_public_key);

    //     println!("> sending request to: {}", auth_url);
    //     webbrowser::open(&auth_url)?;

    //     Ok(())
    // }

    // fn wait_for_response() -> Result<String, Box<dyn std::error::Error>> {
    //     let listener = TcpListener::bind("localhost:8080")?;

    //     for stream in listener.incoming() {
    //         let mut stream = stream?;
    //         let mut reader = BufReader::new(&stream);
    //         let mut request_line = String::new();
    //         reader.read_line(&mut request_line)?;

    //         if request_line.starts_with("GET") {
    //             let url = url::Url::parse(&format!("http://localhost{}", request_line.split_whitespace().nth(1).unwrap()))?;
    //             let code = url.query_pairs()
    //                 .find(|(key, _)| key == "code")
    //                 .map(|(_, value)| value.into_owned())
    //                 .ok_or("No code found in URL")?;

    //             let auth_code = AuthorizationCode::new(code);

    //             // Create the OAuth2 client (assuming it's already configured elsewhere)
    //             let client = BasicClient::new(
    //                 ClientId::new(OAuth2::get_env_var("NETLIFY_CLIENT_ID").unwrap()),
    //                 Some(ClientSecret::new(OAuth2::get_env_var("NETLIFY_CLIENT_SECRET").unwrap())),
    //                 AuthUrl::new("https://app.netlify.com/authorize".to_string()).unwrap(),
    //                 Some(TokenUrl::new("https://api.netlify.com/oauth/token".to_string()).unwrap())
    //             );

    //             // Exchange the code for a token
    //             let token_result = client
    //                 .exchange_code(auth_code)
    //                 .request(oauth2::reqwest::http_client)?;

    //             // Return the access token as a string
    //             return Ok(token_result.access_token().secret().to_string());
    //         }
    //     }

    //     Err("Failed to receive response".into())
    // }

    // pub fn oauth2() -> Result<StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>, Box<dyn Error>> {
    pub fn netlify_oauth2_code_grant() -> Result<
        oauth2::StandardTokenResponse<oauth2::EmptyExtraTokenFields, oauth2::basic::BasicTokenType>,
        (),
    > {
        println!("> Starting Netlify OAuth2 flow");
        let client = BasicClient::new(
            ClientId::new(OAuth2::get_env_var("NETLIFY_CLIENT_ID").unwrap()),
            // trying with no secret
            // None,
            Some(ClientSecret::new(
                OAuth2::get_env_var("NETLIFY_CLIENT_SECRET").unwrap(),
            )),
            AuthUrl::new("https://app.netlify.com/authorize".to_string()).unwrap(),
            Some(TokenUrl::new("https://api.netlify.com/oauth/token".to_string()).unwrap()),
        )
        .set_redirect_uri(
            RedirectUrl::new(OAuth2::get_env_var("NETLIFY_REDIRECT_URI").unwrap()).unwrap(),
        );

        println!("> Generating PKCE challenge...");

        // Generate the authorization URL
        let (auth_url, csrf_token) = client.authorize_url(CsrfToken::new_random).url();

        println!("Open this URL in your browser:\n{auth_url}\n");

        let (code, state) = {
            // A very naive implementation of the redirect server.
            let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

            // The server will terminate itself after collecting the first code.
            let Some(mut stream) = listener.incoming().flatten().next() else {
                panic!("listener terminated without accepting a connection");
            };

            let mut reader = BufReader::new(&stream);

            let mut request_line = String::new();
            reader.read_line(&mut request_line).unwrap();

            println!("Request line: {:?}", request_line);

            let redirect_url = request_line.split_whitespace().nth(1).unwrap();

            println!("Redirect URL: {:?}", redirect_url);

            let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

            println!("URL: {:?}", url.to_string().clone());

            let code = url
                .query_pairs()
                .find(|(key, _)| key == "code")
                .map(|(_, code)| AuthorizationCode::new(code.into_owned()))
                .unwrap();

            let state = url
                .query_pairs()
                .find(|(key, _)| key == "state")
                .map(|(_, state)| CsrfToken::new(state.into_owned()))
                .unwrap();

            let message = "Driftwood authorization complete! You can close this window now.";
            let response = format!(
                "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                message.len(),
                message
            );
            stream.write_all(response.as_bytes()).unwrap();

            (code, state)
        };

        println!("Netlify code:\n{}\n", code.secret());
        println!(
            "Netlify state:\n{} (expected `{}`)\n",
            state.secret(),
            csrf_token.secret()
        );

        // Exchange the authorization code for an access token
        let token_result = client.exchange_code(code).request(http_client);

        match token_result {
            Ok(token_result) => {
                println!(
                    "Netlify Access token: {}",
                    token_result.access_token().secret()
                );
                Ok(token_result)
            }
            Err(err) => {
                println!("Failed to get token: {}", err);
                if let oauth2::RequestTokenError::ServerResponse(body) = err {
                    println!("Raw server response: {}", body);
                }
                Err(())
            }
        }
    }
}
