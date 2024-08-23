/// TODO - Arrange the blog cards in a grid
/// TODO - Add a customizable favicon for the site
/// TODO - Add an 'about' page
/// TODO - Center the title on blog post pages
use serde::Serialize;
use std::{env, error::Error, fs, path::Path};
use tinytemplate::{format_unescaped, TinyTemplate};

pub struct Post {
    pub title: String,
    pub date: String,
    pub content: String,
    pub filename: String,
}

#[derive(Serialize)]
struct BlogCardContext {
    filename: String,
    title: String,
    date: String,
    excerpt: String,
    image: String,
    sitename: String,
}

#[derive(Serialize)]
struct IndexContext {
    sitename: String,
    blog_cards: String,
}

#[derive(Serialize)]
struct PostContext {
    title: String,
    content: String,
    date: String,
    sitename: String,
}
// htt_blog_cardps://images.unsplash.com/photo-1615147342761-9238e15d8b96?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=1001&q=80

static POST_CARD_TEMPLATE: &'static str = include_str!("templates/default/blog-card-template.html");
static POST_PAGE_TEMPLATE: &'static str = include_str!("templates/default/post-template.html");
static INDEX_TEMPLATE: &'static str = include_str!("templates/default/index-template.html");

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

    pub fn template_html(
        posts: Vec<String>,
        site_path: String,
        site_name: String,
    ) -> Result<bool, Box<dyn Error>> {
        println!(">> Templating HTML");

        // create the templates
        println!(">> Creating templates");
        let mut tt_blog_card = TinyTemplate::new();
        tt_blog_card.set_default_formatter(&format_unescaped);
        let mut tt_post_page = TinyTemplate::new();
        tt_post_page.set_default_formatter(&format_unescaped);
        let mut tt_index = TinyTemplate::new();
        tt_index.set_default_formatter(&format_unescaped);
        println!(">> Adding templates");
        tt_blog_card.add_template("card", POST_CARD_TEMPLATE)?;
        tt_post_page.add_template("post", POST_PAGE_TEMPLATE)?;
        tt_index.add_template("index", INDEX_TEMPLATE)?;
        println!(">> Templates created");

        let site_path = Path::new(&site_path);
        let mut rendered_index = String::new();
        let mut rendered_blog_cards = String::new();

        println!(">> Iterating through posts");

        // iterate through all the Posts
        posts.iter().for_each(|post| {
            println!(">> Post: {}", post);

            let post_file_path = Path::new(&post);
            let post_file = fs::read_to_string(post_file_path).unwrap();
            let mut excerpt = String::new();
            let mut image = String::new();
            let mut new_post_file = String::new();
            let mut date = String::new();

            // extract the date, excerpt, and image from the post_file
            for line in post_file.lines() {
                if line.trim().starts_with("<p>date:") {
                    date.push_str(line.replace("<p>date:", "").trim());
                    continue;
                } else if line.trim().starts_with("excerpt:") {
                    excerpt.push_str(line.replace("excerpt:", "").trim());
                    continue;
                } else if line.trim().starts_with("image:") {
                    image.push_str(line.replace("image:", "").trim());
                    continue;
                } else {
                    // put all the lines, except the above three, into new_post_file
                    new_post_file.push_str(line);
                    new_post_file.push_str("\n\r");
                }
            }

            let _ = fs::write(post_file_path, &new_post_file);
            // read in the updated file
            let post_file = fs::read_to_string(post_file_path).unwrap();
            println!(">> Post file written to disk");

            let post_file_name = post_file_path
                .file_name()
                .unwrap()
                .to_str()
                .expect("Failed to convert post file path to string");

            // also create a post title out of that
            let post_title = post_file_name
                .replace(".html", "")
                .replace(".md", "")
                .replace("-", " ");

            // create the context/data for the template
            println!(">> Creating blog card context");
            let blog_card_context = BlogCardContext {
                filename: format!("/posts/{}", post_file_name),
                title: post_title.clone(),
                date: date.clone(),
                excerpt: excerpt,
                image: image,
                sitename: site_name.clone(),
            };

            rendered_blog_cards.push_str(
                &tt_blog_card
                    .render("card", &blog_card_context)
                    .expect("Failed templating the blog link context"),
            );
            println!(">> Blog card context templated");

            let post_context = PostContext {
                title: post_title,
                content: post_file,
                date: date.clone(),
                sitename: site_name.clone(),
            };
            println!(">> Templating post: {}", post_file_path.to_str().unwrap());
            let rendered_post = tt_post_page
                .render("post", &post_context)
                .expect("Failed templating the post context");
            println!(">> Templated post: {}", post_file_path.to_str().unwrap());
            fs::write(post_file_path, &rendered_post).expect("Failed to write post to disk");

        });

        println!(">> Templating index");
        let index_context = IndexContext {
            sitename: site_name.clone(),
            blog_cards: rendered_blog_cards,
        };

        rendered_index.push_str(
            &tt_index
                .render("index", &index_context)
                .expect("Failed templating the index context"),
        );
        println!(">> Templated index");
        println!(">> Writing index to disk");
        let index_filename = site_path.join("index.html");
        fs::write(index_filename, rendered_index)?;

        Ok(true)
    }
}

pub struct OAuth2 {}
impl OAuth2 {
    pub fn get_env_var(name: &str) -> Result<String, env::VarError> {
        let var = env::var(name)?;
        Ok(var)
    }
}
