use std::{
    io::Write,
    fs,
};

use driftwood::{
    Post,
    netlify,
};
use chrono;

/// Draws the menu and all the options
pub fn draw_menu(){    
    loop {
        // clear the terminal
        print!("\x1B[2J\x1B[1;1H");
        println!("Driftwood - Deploy blogs in one click");
        println!("---------------------------------------");
        println!("Options:");
        println!("1. Create a blog post");
        println!("2. Publish the website");
        println!("Type 'exit' to quit.");
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => create_post(),
            "2" => publish_website(),
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
    if !check_input_length(&input, 2) {return;}

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
    fs::write(&post.filename,"").expect("Failed to write to file.");
    println!("Post `{}` was created successfully. Edit your new file in: {}", post.title, post.filename);
    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdin().read_line(&mut input).unwrap();

}

fn publish_website(){
    print!("\x1B[2J\x1B[1;1H");
    println!("Publish the website");
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

    if !check_input_length(&input, 2) {return;}

    let website_name = input.trim().to_string();
    // println!("Pretending to publish site `{}` to netlify lol", website_name);
    // let _ = netlify::connect_to_api();
    println!("Press enter to return to the main menu.");
    print!("> ");
    std::io::stdin().read_line(&mut input).unwrap();
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
fn convert_md_to_html_cli(){
    print!("\x1B[2J\x1B[1;1H");
    println!("Convert a markdown file to HTML:");
    println!("---------------------------------------");
    println!("Enter the name of the markdown file followed by the name of the output HTML file.");
    println!("Type 'exit' to return to the main menu.");
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input).expect(
        "Invalid entry for markdown and html filenames."
    );

    // needs to be at least 3 characters long
    if !check_input_length(&input, 3) {return;}

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