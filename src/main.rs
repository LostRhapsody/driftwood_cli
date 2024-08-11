use http_body_util::Empty;
use http_body_util::BodyExt;
use hyper::body;
use hyper::body::Body;
use hyper::Request;
use hyper::body::Bytes;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt as _, self};

pub mod cli;
pub mod netlify;

use netlify::{
    Netlify,
    SiteDetails,
    Payload,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let url = "https://api.netlify.com/api/v1/sites".parse::<hyper::Uri>()?;
    let url = "http://httpbin.org/ip".parse::<hyper::Uri>()?;
    
    let host = url.host().expect("uri has no host");

    let port = url.port_u16().unwrap_or(80);

    let address = format!("{}:{}", host, port);

    let stream = TcpStream::connect(address).await?;

    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let payload = Payload {
        name: String::from("testSite6"),
    };

    // Serialize the payload to JSON
    let json_payload = serde_json::to_string(&payload)?;
    
    // Convert the JSON string to bytes
    let body = Body::from(json_payload);
    
    let req = Request::builder()
        .uri(url)
        .header(hyper::header::HOST, authority.as_str())
        .body(body)?;

    let mut res = sender.send_request(req).await?;

    println!("Response status: {}", res.status());
    
    while let Some(next) = res.frame().await {
        let frame = next?;
        if let Some(chunk) = frame.data_ref() {
            io::stdout().write_all(chunk).await?;
        }
    }
    
    // cli::draw_menu();
    // let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");

    // Get all the sites
    // let _ = get_sites(netlify);

    // Create a new site
    // let _ = create_site(netlify,String::from("testSite6"));

    Ok(())
}

/// Get all the sites for the user
/// netlify: A Netlify instance
/// Returns a vector of SiteDetails
fn get_sites(netlify: Netlify) -> Vec<SiteDetails> {
    match netlify.get_sites() {
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

/// Add a new site
/// netlify: A Netlify instance
/// site_name: The name of the site to create
/// Returns a vector of SiteDetails
fn create_site(netlify: Netlify, site_name: String) -> SiteDetails {
    match netlify.create_site(site_name) {
        Ok(sites) => {
            println!("Done");
            println!("\nSite Details:");
            println!("{:?}", sites);
            sites
        }
        Err(e) => {
            println!("Error: {:?}", e);
            SiteDetails {
                name: Some(String::from("")),
                url: Some(String::from("")),
                screenshot_url: Some(String::from(""))
            }
        }
    }
}