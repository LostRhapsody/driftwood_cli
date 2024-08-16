use pulldown_cmark::html;
use serde::Serialize;
use std::{error::Error, fs};
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
}

static LINK_TEMPLATE: &'static str = "
    <a class='card' href='{filename}'>
        <img src='https://images.unsplash.com/photo-1615147342761-9238e15d8b96?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=1001&q=80' class='card__image' alt='brown couch' />
        <div class='card__content'>
            <time datetime='2021-03-30' class='card__date'>30 März 2021</time>
            <span class='card__title'>{title}<span>
        </div>
    </a>
    <br />
";

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
        println!(">> Template HTML");

        // create the <a> tag template
        let mut tt = TinyTemplate::new();
        tt.add_template("link", LINK_TEMPLATE)?;

        // read in the index-template.html file
        let template_filename = format!("{}/index-template.html", site_path);
        let template_file = fs::read_to_string(template_filename.clone())?;
        let index_filename = format!("{}/index.html", site_path);
        println!(">> Template Filename: {}", template_filename);
        println!(">> Template File: {}", template_file);

        let mut new_html_output = String::new();
        // read in each line of the template_file until you hit the <!-- blog links --> line
        for line in template_file.lines() {
            println!(">> Line: {}", line);

            // once we find the line, loop through each post and insert it as a link
            if line.trim() == "<!-- blog links -->" {
                // iterate through all the Posts
                posts.iter().for_each(|post| {
                    println!(">> Post: {}", post);

                    // each post contains its FULL path on disk,
                    // but we just need the path starting from the /posts/ directory
                    let post_file_path = std::path::Path::new(&post);
                    let post_file_path = post_file_path.file_name().unwrap().to_str().unwrap();

                    println!(">> Post File Path: {}", post_file_path);

                    // also create a post title out of that
                    let post_title = post_file_path.replace(".html", "");
                    let post_title = post_title.replace(".md", "");
                    let post_title = post_title.replace("-", " ");

                    println!(">> Post Title: {}", post_title);

                    let context = Context {
                        filename: format!("/posts/{}", post_file_path),
                        title: post_title,
                    };

                    let rendered = tt
                        .render("link", &context)
                        .expect("Tried templating the link context, failed haha");

                    println!(">> Context: {:?}", context.filename);
                    new_html_output.push_str(&rendered);
                    new_html_output.push_str("\n\r");
                });
            }
            // take every line we read in and add it to the new_html_output
            new_html_output.push_str(line);
            new_html_output.push_str("\n\r");
        }

        fs::write(index_filename, &new_html_output)?;

        Ok(true)
    }

}
