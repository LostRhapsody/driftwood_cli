use std::fs;

pub struct Post {
    pub title: String,
    date: String,
    content: String,
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

    pub fn read_and_parse(md_filename: &str, html_filename: &str) -> bool {
        let md_input = fs::read_to_string(md_filename)
            .expect("Couldn't find markdown file");
        let parser = pulldown_cmark::Parser::new(&md_input);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        // output the new file to disk
        fs::write(html_filename, &html_output)
            .expect("Couldn't write to output file");
        true
    }

}