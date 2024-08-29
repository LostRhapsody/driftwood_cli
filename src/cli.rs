use crate::netlify::{Netlify, Ssl_Cert};
use anyhow::{Context, Result};
use driftwood::{Git, Post, SiteDetails, read_and_parse, template_html};
use std::{fs, io::Write, path::Path, vec};

// TODO - Seperate all the logic that involves building files or interacting with the Netlify API to lib.rs.
// TODO - Implement tui-rs for a better user experience

/// Draws the menu and all the options
pub fn draw_menu() -> Result<()> {
    loop {
        println!("Driftwood - Deploy blogs in one click");
        println!("---------------------------------------");
        println!("Options:");
        println!("1. Create a site");
        println!("2. Select a site");
        println!("Type 'q' to quit.");
        print!("> ");
        std::io::stdout()
            .flush()
            .context("Failed to flush stdout")?;

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read line")?;

        let _ = match input.trim() {
            "1" => create_website(),
            "2" => list_websites(),
            "q" => break,
            _ => print_error_message("Invalid option. Please try again."),
        };
    }
    Ok(())
}

fn create_post(site: &SiteDetails) -> Result<()> {
    println!("Create a blog post");
    println!("---------------------------------------");
    println!("Enter the name of the blog post.");
    println!("Type 'q' to quit.");
    print!("> ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;

    // standard input stuff
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;
    // Process the input here
    if input.trim() == "q" {
        return Ok(());
    }
    // needs to be at least 1 characters long
    if !check_input_length(&input, 2) {
        return Ok(());
    }

    // create a new post
    // date is set automatically
    let mut new_post = Post::new(input.trim().to_string());

    // strip bad chars and set post.filename
    new_post.clean_filename()?;
    // (replace "-"" with spaces basically) and set post.title
    new_post.build_post_name()?;
    // check if the site dir exists, if not create it (sitedir/md_posts)
    Post::check_post_dir(site)?;

    println!("Enter tags for the post, separated by commas.");
    print!("> ");
    let mut input = String::new();
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;

    println!("Input: {}", input);

    // remove special chars and set post.tags
    new_post.clean_and_set_tags(input)?;

    // write the post to disk
    match new_post.write_post_to_disk(site) {
        Ok(_) => println!("Post written to disk."),
        Err(e) => println!("Error: {}", e),
    }

    if !site.check_for_site_repo()? {
        site.create_site_repo().context("Failed to initialize new repository")?
    }

    Post::commit_post_to_repo(site, &format!("Add new post: {}", new_post.title))?;

    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdin()
        .read_line(&mut String::new())
        .context("Failed to read line")?;
    Ok(())
}

fn create_website() -> Result<()> {
    println!("Create a website");
    println!("---------------------------------------");
    println!("Enter the name of your website.");
    println!("Note: Special characters, spaces, and punctuation will be replaced with a dash '-'.");
    println!("Type 'q' to return to the main menu.");
    print!("> ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;

    // Process the input here
    if input.trim() == "q" {
        return Ok(());
    }

    if !check_input_length(&input, 2) {
        return Ok(());
    }

    let website_name = input.trim().to_string();

    let netlify: Netlify = Netlify::new();
    let site_details = create_site(netlify, website_name).expect("Failed to create site");
    make_site_dir(&site_details);

    let repo = Git::init_git_repo(&site_details.name.clone().unwrap());
    Git::commit_changes(&repo.unwrap(), "Initial commit")?;

    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;

    Ok(())
}

fn list_websites() -> Result<()> {
    // grab all the sites
    let netlify: Netlify = Netlify::new();
    let site_details: Vec<SiteDetails> = get_sites(netlify);

    println!("Your Websites");
    println!("---------------------------------------");
    // print out all site names
    site_details.iter().for_each(|site| {
        println!("{}", site.name.clone().unwrap());
    });
    println!("---------------------------------------");
    println!("To edit a website's details, enter the site's name.");
    println!("Type 'q' to return to the main menu.");
    print!("> ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;

    // Process the input here
    if input.trim() == "q" {
        return Ok(());
    }

    if !check_input_length(&input, 2) {
        return Ok(());
    }

    let site_id = input.trim().to_string();

    // if the site id matches one of the site names in site_details
    // then get the site details for that site
    let site = site_details
        .iter()
        .find(|site| site.name.clone().unwrap() == site_id);

    if site.is_some() {
        make_site_dir(site.unwrap());
        update_site(site.unwrap())?;
    }

    Ok(())
}

fn update_site(site: &SiteDetails) -> Result<()> {
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
    println!("Type 'q' to return to the main menu.");
    print!("> ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;

    // Process the input here
    if input.trim() == "q" {
        return Ok(());
    }

    if !check_input_length(&input, 2) {
        return Ok(());
    }

    match input.trim() {
        "1" => create_post(site),
        "2" => deploy_site(site),
        "3" => update_site_name(site),
        "4" => delete_site(site),
        "5" => create_ssl_certificate(site),
        _ => print_error_message("Invalid option. Returning to main menu."),
    }?;

    Ok(())
}

fn print_error_message(msg: &str) -> Result<()> {
    println!("{}", msg);
    Ok(())
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

    println!("Path: {}", path.display());

    if !path.exists() {
        fs::create_dir(path).expect(&format!(
            "Failed to create site directory for site: {}",
            site.name.clone().unwrap()
        ));
    }
}

fn update_site_name(site: &SiteDetails) -> Result<()> {
    let netlify: Netlify = Netlify::new();

    println!("Name: {}", site.name.clone().unwrap());
    println!("Enter the new name of your website.");
    println!("Note: Special characters, spaces, and punctuation will be replaced with a dash '-'.");
    println!("Type 'q' to return to the main menu.");
    print!("> ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;

    // Process the input here
    if input.trim() == "q" {
        return Ok(());
    }

    let site_name = input.trim().to_string();

    let mut new_site = site.clone();
    new_site.name = Some(site_name);
    let existing_site = site.clone();

    let _ = update_site_details(netlify, existing_site, new_site);

    Ok(())
}

fn deploy_site(site: &SiteDetails) -> Result<()> {
    let netlify: Netlify = Netlify::new();

    // first loop through the site's posts and convert them to HTML
    let site_path = SiteDetails::build_site_path(site)?;
    // convert site_path PathBuf to string
    let site_path = site_path.to_string_lossy().to_string();
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

    let mut html_file_names = vec![];

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
            let success = read_and_parse(&md_file_name, &html_file_name);
            match success {
                Ok(_) => {
                    println!("Successfully converted markdown to HTML.");
                    // add this html file name to a vector of strings
                    html_file_names.push(html_file_name);
                }
                Err(e) => {
                    println!("Failed to convert markdown to HTML.");
                    println!("Error: {:?}", e);
                }
            }
        }
    }

    // remove any dashes or underscores from the site name, replace with spaces
    let clean_site_name = site
        .name
        .clone()
        .unwrap()
        .replace("-", " ")
        .replace("_", " ");
    let template_success = template_html(html_file_names, site_path.clone(), clean_site_name);
    match template_success {
        Ok(_) => {
            println!("Successfully templated blog links.");
        }
        Err(e) => {
            println!("Failed to template blog links.");
            println!("Error: {:?}", e);
        }
    }

    // second generate the sha1 hash of all the html files
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
        return Ok(());
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
                sha1_hashmap.files.iter().for_each(|file_hash| {
                    // destructure the tuple (apparently iterating through hashmaps gives you tuples)
                    let (current_file_name, current_file_hash) = file_hash;
                    // if they match, print
                    if file == current_file_hash {
                        println!("> Matching File hash: {:?}", file_hash);
                        let response = netlify.upload_file(
                            site.name.clone().unwrap(),
                            site.id.clone().unwrap(),
                            new_site.id.clone().unwrap(),
                            Path::new(current_file_name),
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

    Ok(())
}

fn delete_site(site: &SiteDetails) -> Result<()> {
    let netlify: Netlify = Netlify::new();

    println!("Deleting: {}", site.name.clone().unwrap());
    println!("This will permanently delete the website.");
    println!("Are you sure you want to continue?");
    println!("Type 'yes' to delete.");
    println!("Type 'q' to return to the main menu.");
    print!("> ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;

    // Process the input here
    if input.trim() == "q" {
        return Ok(());
    } else if input.trim() == "yes" {
        let _ = netlify.delete_site(site.clone());
        println!("Site deleted.");
        println!("Press enter to return to the main menu.");
        print!("> ");
        std::io::stdin()
            .read_line(&mut input)
            .context("Failed to read line")?;
    } else {
        println!("Invalid option. Returning to main menu.");
    }

    Ok(())
}

fn create_ssl_certificate(site: &SiteDetails) -> Result<()> {
    println!("Enter the details for your SSL certificate.");
    println!("Type 'q' to return to the main menu.");
    print!("Certificate > ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;
    if input.trim() == "q" {
        return Ok(());
    }
    let certificate = input.trim().to_string();
    print!("Key > ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;
    if input.trim() == "q" {
        return Ok(());
    }
    let key = input.trim().to_string();
    print!("CA > ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;
    if input.trim() == "q" {
        return Ok(());
    }
    let ca = input.trim().to_string();
    println!(
        "Creating SSL certificate for site: {}",
        site.name.clone().unwrap()
    );
    let netlify: Netlify = Netlify::new();
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
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;

    Ok(())
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
fn convert_md_to_html_cli() -> Result<()> {
    println!("Convert a markdown file to HTML:");
    println!("---------------------------------------");
    println!("Enter the name of the markdown file followed by the name of the output HTML file.");
    println!("Type 'q' to return to the main menu.");
    print!("> ");
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Invalid entry for markdown and html filenames.");

    // needs to be at least 3 characters long
    if !check_input_length(&input, 3) {
        return Ok(());
    }

    if input.trim() == "q" {
        return Ok(());
    }

    let args: Vec<&str> = input.trim().split(" ").collect();

    let success = read_and_parse(&args[0], &args[1]);
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
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read line")?;

    Ok(())
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