/// TODO - Find out why the date is still showing up on the blog post page
/// TODO - Add a nicer formatted date to the blog cards and post page
/// TODO - The date doesn't show up on the blog cards
/// TODO - replace {SITE_TITLE} with the actual site title
/// TODO - Update blog card title's for dark mode, can't read them
/// TODO - space out the links in the table of contents
/// TODO - Arrange the blog cards in a grid
/// TODO - Make the hero section on the home page an image
/// TODO - Add a customizable favicon for the site
/// TODO - Add an 'about' page
/// TODO - Center the title on blog post pages
use serde::Serialize;
use std::{env, error::Error, fs, path::Path};
use tinytemplate::TinyTemplate;

pub struct Post {
    pub title: String,
    pub date: String,
    pub content: String,
    pub filename: String,
}

#[derive(Serialize)]
struct Context {
    filename: String,
    title: String,
    date: String,
    excerpt: String,
    image: String,
}
static LINK_TEMPLATE: &'static str = r#"
    <div class="card">
        <img src="{image}" class="card__image" alt="{title}" />
        <div class="card__content">
            <time datetime="{date}" class="card__date">{date}</time>
            <h2 class="card__title"><a href="{filename}">{title}</a></h2>
            <p>{excerpt}</p>
        </div>
    </div>
"#;

impl Post {
    pub fn new(title: String, date: String, content: String, filename: String) -> Post {
        Post {
            title: title,
            date: date,
            content: content,
            filename: filename,
        }
    }

    pub fn read_and_parse(md_filename: &str, html_filename: &str) -> Result<bool, Box<dyn Error>> {
        println!(">> Reading file: {}", md_filename);
        let md_input = fs::read_to_string(md_filename)?;
        println!(">> Building parser");
        let parser = pulldown_cmark::Parser::new(&md_input);
        println!(">> Creating HTML output string");
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        println!(">> Pushed HTML to cmark parser");
        // output the new file to disk
        fs::write(html_filename, &html_output)?;
        println!(">> Wrote new file to disk");
        Ok(true)
    }

    pub fn template_html(posts: Vec<String>, site_path: String) -> Result<bool, Box<dyn Error>> {
        println!(">> Templating HTML");

        // create the <a> tag template
        let mut tt = TinyTemplate::new();
        tt.add_template("link", LINK_TEMPLATE)?;

        let site_path = Path::new(&site_path);        

        /***********************
        * Build the Home page  *
        ***********************/

        // read in the index-template.html file
        let template_filename = site_path.join("index-template.html");
        let template_file = fs::read_to_string(template_filename.clone())?;

        let mut new_html_output = String::new();

        // read in each line of the template_file until you hit the <!-- blog links --> line
        for line in template_file.lines() {
            // once we find the line, loop through each post and insert it as a link
            if line.trim() == "<!-- blog links -->" {

                // iterate through all the Posts
                posts.iter().for_each(|post| {
                    println!(">> Post: {}", post);

                    // each post contains its FULL path on disk,
                    // but we just need the path starting from the /posts/ directory
                    let post_file_path = Path::new(&post);
                    let post_file = fs::read_to_string(post_file_path).unwrap();
                    let mut date = String::new();
                    let mut excerpt = String::new();
                    let mut image = String::new();
                    let mut new_post_file = String::new();

                    // extract the date, excerpt, and image from the post_file
                    for line in post_file.lines() {
                        if line.trim().starts_with("<p>date:") {
                            date.push_str(line.replace("<p>date:", "").trim());
                            continue;
                        }
                        else if line.trim().starts_with("excerpt:") {
                            excerpt.push_str(line.replace("excerpt:", "").trim());
                            continue;
                        }
                        else if line.trim().starts_with("image:") {
                            image.push_str(line.replace("image:", "").trim());
                            continue;
                        } else {
                            // put all the lines, except the above three, into new_post_file
                            new_post_file.push_str(line);
                            new_post_file.push_str("\n\r");
                        }
                    }

                    let _ = fs::write(post_file_path, &new_post_file);

                    let post_file_path = post_file_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .expect("Failed to convert post file path to string");


                    // also create a post title out of that
                    let post_title = post_file_path
                        .replace(".html", "")
                        .replace(".md", "")
                        .replace("-", " ");

                    // create the context/data for the template
                    let context = Context {
                        filename: format!("/posts/{}", post_file_path),
                        title: post_title,
                        date: date,
                        excerpt: excerpt,
                        image: image,
                    };

                    let rendered = tt
                        .render("link", &context)
                        .expect("Failed templating the blog link context");

                    new_html_output.push_str(&rendered);
                    new_html_output.push_str("\n\r");
                    
                });
            }
            // take every line we read in and add it to the new_html_output
            new_html_output.push_str(line);
            new_html_output.push_str("\n\r");
        }

        let index_filename = site_path.join("index.html");
        fs::write(index_filename, &new_html_output)?;

        /***********************
        *Build the Posts pages *
        ***********************/
        let post_template_filename = site_path.join("post-template.html");
        let post_template_file = fs::read_to_string(post_template_filename.clone())?;


        let posts_path = site_path.join("posts");
        let posts_files = fs::read_dir(posts_path)?;
        for post in posts_files {
            
            let post_path = post.unwrap().path();

            println!(">> Post: {}", post_path.to_str().unwrap());

            let post_file = fs::read_to_string(post_path.clone())?;

            let mut new_html_output = String::new();

            for line in post_template_file.lines() {
                println!("line: {}", line);
                if line.trim() == "{POST_TITLE}" {

                    let post_file_path = post_path
                        .file_name()
                        .unwrap()
                        .to_str()
                        .expect("Failed to convert post file path to string");

                    // create a post title out of that
                    let post_title = post_file_path
                        .replace(".html", "")
                        .replace(".md", "")
                        .replace("-", " ");

                    new_html_output.push_str(&post_title);
                    new_html_output.push_str("\n\r");
                    // skip to the next loop
                    continue;
                }
                if line.trim() == "{POST_CONTENT}" {
                    new_html_output.push_str(&post_file);                
                    new_html_output.push_str("\n\r");
                    // skip to the next loop
                    continue;
                }
                new_html_output.push_str(line);
            new_html_output.push_str("\n\r");
            }

            fs::write(post_path, &new_html_output)?;

        }


        Ok(true)
    }

}

pub struct OAuth2 {}
impl OAuth2 {
    pub fn get_env_var(name:&str) -> Result<String, env::VarError> {
        let var = env::var(name)?;
        Ok(var)
    }
}