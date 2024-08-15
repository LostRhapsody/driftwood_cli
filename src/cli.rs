/// TODO - make a standard function for writing out menus to reduce repeat code
/// TODO - make a standard function for reading input to reduce repeat code
/// TODO - Rebuild the navigation so users can go back and forth between menus easier
use std::{fs, io::Write, path::Path};

use driftwood::Post;

use chrono;

use crate::netlify::{Netlify, SiteDetails, Ssl_Cert};

/// Draws the menu and all the options
pub fn draw_menu() {
    loop {
        // clear the terminal
        // print!("\x1B[2J\x1B[1;1H");
        println!("Driftwood - Deploy blogs in one click");
        println!("---------------------------------------");
        println!("Options:");
        println!("1. Create a site");
        println!("2. Select a site");
        println!("Type 'exit' to quit.");
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => create_website(),
            "2" => list_websites(),
            "exit" => break,
            _ => println!("Invalid option. Please try again."),
        }

        // Process the input here
        if input.trim() == "exit" {
            break;
        }
    }
}

fn create_post(site: &SiteDetails) {
    // clear the terminal
    // print!("\x1B[2J\x1B[1;1H");
    println!("Create a blog post");
    println!("---------------------------------------");
    println!("Enter the name of the blog post.");
    println!("Type 'exit' to quit.");
    print!("> ");
    std::io::stdout().flush().unwrap();

    // standard input stuff
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    // Process the input here
    if input.trim() == "exit" {
        return;
    }
    // needs to be at least 1 characters long
    if !check_input_length(&input, 2) {
        return;
    }

    // build the post path
    let post_path = &format!("{}/md_posts", build_site_path(site));
    let post_path = Path::new(post_path);

    if !post_path.exists() {
        fs::create_dir(post_path).expect("Failed to create this site's 'md_posts' directory");
    }

    // build the title of the file and filename
    let title = input.trim().to_string() + ".md";
    let filename = format!("{}/{}", post_path.display(), title);

    let date = chrono::Local::now();
    let date = date.date_naive().to_string();

    // create a new file in the 'posts' directory
    let post = Post::new(title, date, "This is a test post".to_string(), filename);

    // write the post to disk
    fs::write(&post.filename, "").expect("Failed to write to file.");

    // open the post file written to disk and write the title and timestamp to the file
    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&post.filename)
        .expect("Failed to open file.");

    let post_content = format!("# {}\nDate: {}", post.title, post.date);

    file.write_all(post_content.as_bytes())
        .expect("Failed to write to file.");

    println!(
        "Post `{}` was created successfully. Edit your new file in: {}",
        post.title, post.filename
    );

    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdin().read_line(&mut input).unwrap();
}

fn create_website() {
    // print!("\x1B[2J\x1B[1;1H");
    println!("Create a website");
    println!("---------------------------------------");
    println!("Enter the name of your website.");
    println!("Note: Special characters, spaces, and punctuation will be replaced with a dash '-'.");
    println!("Type 'exit' to return to the main menu.");
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Process the input here
    if input.trim() == "exit" {
        return;
    }

    if !check_input_length(&input, 2) {
        return;
    }

    let website_name = input.trim().to_string();

    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    let _ = create_site(netlify, website_name);

    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdin().read_line(&mut input).unwrap();
}

fn list_websites() {
    // grab all the sites
    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    let site_details: Vec<SiteDetails> = get_sites(netlify);

    // print!("\x1B[2J\x1B[1;1H");
    println!("Your Websites");
    println!("---------------------------------------");
    // print out all site names
    site_details.iter().for_each(|site| {
        println!("{}", site.name.clone().unwrap());
    });
    println!("---------------------------------------");
    println!("To edit a website's details, enter the site's name.");
    println!("Type 'exit' to return to the main menu.");
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Process the input here
    if input.trim() == "exit" {
        return;
    }

    if !check_input_length(&input, 2) {
        return;
    }

    let site_id = input.trim().to_string();

    // if the site id matches one of the site names in site_details
    // then get the site details for that site
    let site = site_details
        .iter()
        .find(|site| site.name.clone().unwrap() == site_id);

    if site.is_some() {
        make_site_dir(site.unwrap());
        update_site(site.unwrap());
    }
}

fn update_site(site: &SiteDetails) {
    // print!("\x1B[2J\x1B[1;1H");
    println!("Name: {}", site.name.clone().unwrap());
    println!("Id: {}", site.name.clone().unwrap());
    println!("URL: {}", site.name.clone().unwrap());
    println!("SSL: {}", site.ssl.clone().unwrap());
    println!("---------------------------------------");
    println!("Options:");
    println!("1. Create a blog post");
    println!("2. Deploy the site");
    println!("3. Update the site's name");
    println!("4. Delete the site");
    println!("5. Provision an SSL certificate");
    println!("Type 'exit' to return to the main menu.");
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Process the input here
    if input.trim() == "exit" {
        return;
    }

    if !check_input_length(&input, 2) {
        return;
    }

    match input.trim() {
        "1" => create_post(site),
        "2" => deploy_site(site),
        "3" => update_site_name(site),
        "4" => delete_site(site),
        "5" => create_ssl_certificate(site),
        _ => println!("Invalid option. Returning to main menu."),
    }
}

/// Creates a directory for the site on disk using site.name + site.id
/// Does not consume the site struct or return any values
fn make_site_dir(site: &SiteDetails) {
    let site_dir = "sites";
    let path = Path::new(site_dir);

    if !path.exists() {
        fs::create_dir(path).expect("Failed to create 'sites' directory");
    }

    // create a directory for the site
    let site_dir = format!(
        "sites/{}_{}",
        site.name.clone().unwrap(),
        site.id.clone().unwrap()
    );
    let path = Path::new(&site_dir);

    if !path.exists() {
        fs::create_dir(path).expect(&format!(
            "Failed to create site directory for site: {}",
            site.name.clone().unwrap()
        ));
    }
}

fn update_site_name(site: &SiteDetails) {
    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    // print!("\x1B[2J\x1B[1;1H");
    println!("Name: {}", site.name.clone().unwrap());
    println!("Enter the new name of your website.");
    println!("Note: Special characters, spaces, and punctuation will be replaced with a dash '-'.");
    println!("Type 'exit' to return to the main menu.");
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Process the input here
    if input.trim() == "exit" {
        return;
    }

    let site_name = input.trim().to_string();

    let mut new_site = site.clone();
    new_site.name = Some(site_name);
    let existing_site = site.clone();

    let _ = update_site_details(netlify, existing_site, new_site);
}

fn deploy_site(site: &SiteDetails) {
    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");

    // first loop through the site's posts and convert them to HTML
    let site_path = build_site_path(site);
    let post_path = format!("{}/md_posts", site_path);
    let html_post_path = format!("{}/posts", site_path);

    // lol bad names... it's the PATH types of these string paths
    let post_path_path = Path::new(&post_path);
    let html_post_path_path = Path::new(&html_post_path);
    if !post_path_path.exists() {
        fs::create_dir(post_path_path).expect("Failed to create this site's 'md_posts' directory");
    }
    if !html_post_path_path.exists() {
        fs::create_dir(html_post_path_path)
            .expect("Failed to create this site's 'md_posts' directory");
    }

    // loop through md posts
    for entry in fs::read_dir(post_path).unwrap() {
        // md filename
        let entry = entry.unwrap();
        println!("> {:?}", entry.file_name().to_string_lossy());
        // full path to md file
        let md_file_name = entry.path();
        println!("> {:?}", md_file_name);
        // full path to html file
        let html_file_name = format!(
            "{}/posts/{}.html",
            site_path,
            entry.file_name().to_string_lossy()
        );
        println!("> {:?}", html_file_name);

        if md_file_name.is_file() {
            let md_file_name = md_file_name.to_string_lossy();
            // let html_file_name = html_file_name.file_name().unwrap().to_string_lossy().into_owned();
            // convert to html
            let success = Post::read_and_parse(&md_file_name, &html_file_name);
            match success {
                Ok(_) => {
                    println!("Successfully converted markdown to HTML.");
                }
                Err(e) => {
                    println!("Failed to convert markdown to HTML.");
                    println!("Error: {:?}", e);
                }
            }
        }
    }

    // second generate the sha1 hash of all the html files
    let site_path = build_site_path(site);
    let post_path = format!("{}/posts", site_path);

    let site_path = Path::new(&site_path);
    let post_path = Path::new(&post_path);

    let sha1_result = Netlify::generate_sha1_for_posts(site_path, post_path);

    if sha1_result.is_ok() {
        println!("> SHA1 hash generated successfully.");
    } else {
        println!("> Error: {}", sha1_result.err().unwrap());
        println!("Press enter to return to the main menu.");
        print!("> ");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        return;
    }    

    // unwrap the result to get the FileHashes struct
    let sha1_hashmap = sha1_result.unwrap();

    // post the file hashes to netlify
    let new_site = netlify.send_file_checksums(site.clone(), &sha1_hashmap);
    
    // make sure you don't overlap "site" and "new site"
    // site is the og site details, new site is the deploy details + site details
    // the id will overlap
    match new_site {
        Ok(new_site) => {
            println!(">Site Details:");
            println!("{:?}", new_site);

            // loop over the site's required vector (unwrap to get outside the option)
            new_site.required.unwrap().iter().for_each(|file| {
                println!("> Required file: {:?}", file);

                // loop through our hashmap of file hashes
                sha1_hashmap.files.iter().for_each(|file_hash|{

                    // destructure the tuple (apparently iterating through hashmaps gives you tuples)
                    let (current_file_name,current_file_hash) = file_hash;
                    // if they match, print
                    if file == current_file_hash {
                        println!("> Matching File hash: {:?}", file_hash);
                        let response = netlify.upload_file(
                            site.name.clone().unwrap(),
                            site.id.clone().unwrap(),
                            Path::new(current_file_name)
                        );
                        match response {
                            Ok(_) => {
                                println!("> File uploaded successfully.");
                            }
                            Err(e) => {
                                println!("> Error: {:?}", e);
                            }
                        }
                    }
                });
            });
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}

fn delete_site(site: &SiteDetails) {
    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    // print!("\x1B[2J\x1B[1;1H");
    println!("Deleting: {}", site.name.clone().unwrap());
    println!("This will permanently delete the website.");
    println!("Are you sure you want to continue?");
    println!("Type 'yes' to delete.");
    println!("Type 'exit' to return to the main menu.");
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Process the input here
    if input.trim() == "exit" {
        return;
    } else if input.trim() == "yes" {
        let _ = netlify.delete_site(site.clone());
        println!("Site deleted.");
        println!("Press enter to return to the main menu.");
        print!("> ");
        std::io::stdin().read_line(&mut input).unwrap();
    } else {
        println!("Invalid option. Returning to main menu.");
    }
}

fn create_ssl_certificate(site: &SiteDetails) {
    // print!("\x1B[2J\x1B[1;1H");
    println!("Enter the details for your SSL certificate.");
    println!("Type 'exit' to return to the main menu.");
    print!("Certificate > ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "exit" {
        return;
    }
    let certificate = input.trim().to_string();
    print!("Key > ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "exit" {
        return;
    }
    let key = input.trim().to_string();
    print!("CA > ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "exit" {
        return;
    }
    let ca = input.trim().to_string();
    println!(
        "Creating SSL certificate for site: {}",
        site.name.clone().unwrap()
    );
    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    let mut current_site = site.clone();
    current_site.ssl = Some(true);

    let new_ssl_details = Ssl_Cert {
        cert: Some(certificate),
        key: Some(key),
        ca_cert: Some(ca),
    };

    let _ = provision_ssl(netlify, current_site, new_ssl_details);
    let mut input = String::new();
    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut input).unwrap();
}

fn check_input_length(input: &str, length: usize) -> bool {
    let mut success = true;
    // needs to be at least length characters long
    if input.len() < length {
        println!("Invalid entry");
        println!("Press enter to return to the main menu.");
        print!("> ");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        success = false;
    }
    success
}

/// Convert a markdown file to an HTML file via the command line
fn convert_md_to_html_cli() {
    // print!("\x1B[2J\x1B[1;1H");
    println!("Convert a markdown file to HTML:");
    println!("---------------------------------------");
    println!("Enter the name of the markdown file followed by the name of the output HTML file.");
    println!("Type 'exit' to return to the main menu.");
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Invalid entry for markdown and html filenames.");

    // needs to be at least 3 characters long
    if !check_input_length(&input, 3) {
        return;
    }

    if input.trim() == "exit" {
        return;
    }

    let args: Vec<&str> = input.trim().split(" ").collect();

    let success = Post::read_and_parse(&args[0], &args[1]);
    match success {
        Ok(_) => {
            println!("Successfully converted markdown to HTML.");
        }
        Err(e) => {
            println!("Failed to convert markdown to HTML.");
            println!("Error: {:?}", e);
        }
    }
    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdin().read_line(&mut input).unwrap();
}

/// Add a new site
/// netlify: A Netlify instance
/// site_name: The name of the site to create
/// Returns a vector of SiteDetails
fn create_site(
    netlify: Netlify,
    site_name: String,
) -> Result<SiteDetails, Box<dyn std::error::Error>> {
    match netlify.create_site(SiteDetails {
        name: Some(site_name),
        id: None,
        url: None,
        ssl: None,
        screenshot_url: None,
        required: None,
    }) {
        Ok(sites) => {
            println!("> Site Details:");
            println!("> {:?}", sites);
            Ok(sites)
        }
        Err(e) => {
            println!("> Error: {:?}", e);
            Err(e)
        }
    }
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

/// Update the site details
/// netlify: A Netlify instance
/// Returns a vector of the new SiteDetails
fn update_site_details(
    netlify: Netlify,
    existing_site_details: SiteDetails,
    new_site_details: SiteDetails,
) -> Result<SiteDetails, Box<dyn std::error::Error>> {
    match netlify.update_site(existing_site_details, new_site_details) {
        Ok(site) => {
            println!(">Site Details:");
            println!("{:?}", site);
            Ok(site)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}

fn provision_ssl(
    netlify: Netlify,
    site: SiteDetails,
    ssl_details: Ssl_Cert,
) -> Result<bool, Box<dyn std::error::Error>> {
    match netlify.provision_ssl(site, ssl_details) {
        Ok(ssl_enabled) => {
            println!("> SSL Status:");
            println!("{:?}", ssl_enabled);
            Ok(ssl_enabled)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}

fn build_site_path(site: &SiteDetails) -> String {
    format!(
        "sites/{}_{}",
        site.name.clone().unwrap(),
        site.id.clone().unwrap()
    )
}
