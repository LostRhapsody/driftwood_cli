use std::{fs, io::Write, path::Path};

use driftwood::Post;

use chrono;

use crate::netlify::{Netlify, SiteDetails};

/// Draws the menu and all the options
pub fn draw_menu() {
    loop {
        // clear the terminal
        print!("\x1B[2J\x1B[1;1H");
        println!("Driftwood - Deploy blogs in one click");
        println!("---------------------------------------");
        println!("Options:");
        println!("1. Create a blog post");
        println!("2. Create a site");
        println!("3. List your sites");
        println!("Type 'exit' to quit.");
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => create_post(),
            "2" => create_website(),
            "3" => list_websites(),
            "exit" => break,
            _ => println!("Invalid option. Please try again."),
        }

        // Process the input here
        if input.trim() == "exit" {
            break;
        }
    }
}

fn create_post() {
    // clear the terminal
    print!("\x1B[2J\x1B[1;1H");
    println!("Create a blog post");
    println!("---------------------------------------");
    println!("Enter the name of the blog post.");
    println!("Type 'exit' to quit.");
    print!("> ");
    std::io::stdout().flush().unwrap();

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

    let path = Path::new("posts");

    if !path.exists() {
        fs::create_dir(path).expect("Failed to create 'posts' directory");
    }

    let title = input.trim().to_string() + ".md";
    let filename = "posts/".to_string() + title.clone().as_str();

    // create a new file in the 'posts' directory
    let post = Post::new(
        title,
        chrono::Local::now().to_string(),
        "This is a test post".to_string(),
        filename,
    );

    // write the post to disk
    fs::write(&post.filename, "").expect("Failed to write to file.");
    println!(
        "Post `{}` was created successfully. Edit your new file in: {}",
        post.title, post.filename
    );
    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdin().read_line(&mut input).unwrap();
}

fn create_website() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Create a website");
    println!("---------------------------------------");
    println!("Enter the name of your website.");
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

    print!("\x1B[2J\x1B[1;1H");
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
        update_site(site.unwrap());
    }

    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdin().read_line(&mut input).unwrap();

}

fn update_site(site:&SiteDetails) {

    // grab all the sites
    let netlify: Netlify = Netlify::new("nfp_vc77UcLjcM57aomvo6UsxzJRdRdHNSQie33c");
    print!("\x1B[2J\x1B[1;1H");
    println!("Name: {}", site.name.clone().unwrap());
    println!("Id: {}", site.name.clone().unwrap());
    println!("URL: {}", site.name.clone().unwrap());
    println!("---------------------------------------");
    println!("Type a new name to update the site's name.");
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

    let site_name = input.trim().to_string();

    let new_site = SiteDetails {
        name: Some(site_name),
        id: site.id.clone(),
        url: site.url.clone(),
        screenshot_url: site.screenshot_url.clone(),
    };

    let existing_site = SiteDetails {
        name: site.name.clone(),
        id: site.id.clone(),
        url: site.url.clone(),
        screenshot_url: site.screenshot_url.clone(),
    };

    let _ = update_site_details(netlify, existing_site, new_site);

}

fn check_input_length(input: &str, length: usize) -> bool {
    let mut success = true;
    // needs to be at least length characters long
    if input.len() < length {
        println!("Invalid entry");
        println!("Press enter to return to the main menu.");
        std::io::stdin().read_line(&mut String::new()).unwrap();
        success = false;
    }
    success
}

/// Convert a markdown file to an HTML file via the command line
fn convert_md_to_html_cli() {
    print!("\x1B[2J\x1B[1;1H");
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
    if success {
        println!("Successfully converted markdown to HTML.");
    } else {
        println!("Failed to convert markdown to HTML.");
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
    match netlify.create_site(site_name) {
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
            println!("Done");
            println!("\nSite Details:");
            println!("{:?}", site);
            Ok(site)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e)
        }
    }
}