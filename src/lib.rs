use std::{error::Error, fs};

pub struct Post {
    pub title: String,
    pub date: String,
    pub content: String,
    pub filename: String,
}

impl Post {
    pub fn new(title: String, date: String, content: String, filename: String) 
    -> Post {
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

}