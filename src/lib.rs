use anyhow::{Context, Result};
/// TODO - Add a customizable favicon for the site
/// TODO - Add an 'about' page
/// TODO - Finish the Repository integration
use git2::{Repository, Signature};
use regex::Regex;
use serde::Deserialize;
use serde::Serialize;
use std::{
    env,
    error::Error,
    fs,
    io::Write,
    path::{Path, PathBuf},
};
use tinytemplate::{format_unescaped, TinyTemplate};

pub struct Post {
    pub title: String,
    pub date: String,
    pub content: String,
    pub filename: String,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
struct BlogCardContext {
    filename: String,
    title: String,
    date: String,
    excerpt: String,
    image: String,
    sitename: String,
    tags: String,
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

/// SiteDetails struct
/// Contains the details of a site
/// name: The name of the site
/// url: The URL of the site
/// screenshot_url: The URL of the site's screenshot
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SiteDetails {
    pub name: Option<String>,
    pub id: Option<String>,
    pub ssl: Option<bool>,
    pub url: Option<String>,
    pub screenshot_url: Option<String>,
    pub required: Option<Vec<String>>,
}

static POST_CARD_TEMPLATE: &'static str = include_str!("templates/default/blog-card-template.html");
static POST_PAGE_TEMPLATE: &'static str = include_str!("templates/default/post-template.html");
static INDEX_TEMPLATE: &'static str = include_str!("templates/default/index-template.html");

impl Post {
    pub fn new(title: String) -> Post {
        println!("Creating new post: {}", title);
        let date = chrono::Local::now();
        let date = date.format("%Y/%m/%d %I:%M %p").to_string();
        let filename = String::new();
        let content = String::new();
        let tags = Vec::new();
        Post {
            title,
            date,
            content,
            filename,
            tags,
        }
    }

    pub fn clean_filename(&mut self) -> Result<()> {
        println!("Cleaning filename: {}", self.title);
        let re = Regex::new(r"[^a-zA-Z0-9\s]")?;
        let mut new_filename = self.title.clone();
        // remove all special chars, replace with whitespace
        new_filename = re.replace_all(&new_filename, " ").to_string();
        // remove all extra whitespace (leave one space between words)
        new_filename = new_filename
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        // replace all whitespace with a dash
        new_filename = new_filename.replace(" ", "-");
        self.filename = new_filename;
        println!("Filename cleaned: {}", self.filename);
        Ok(())
    }

    pub fn build_post_name(&mut self) -> Result<()> {
        self.title = self.filename.replace("-", " ");
        println!("Post name built: {}", self.title);
        Ok(())
    }

    pub fn check_post_dir(site: &SiteDetails) -> Result<()> {
        let post_path: PathBuf = SiteDetails::build_site_path(&site)?.join("md_posts");
        println!("Checking post directory: {}", post_path.to_str().unwrap());
        if !post_path.exists() {
            fs::create_dir(post_path)
                .context("Failed to create this site's 'md_posts' directory")?;
        }
        Ok(())
    }

    pub fn build_post_path(&self, site: &SiteDetails) -> Result<PathBuf> {
        let post_path = SiteDetails::build_site_path(&site)?
            .join("md_posts")
            .join(format!("{}.md", self.filename));
        println!("Post path built: {}", post_path.to_str().unwrap());
        Ok(post_path)
    }

    pub fn clean_and_set_tags(&mut self, new_tags: String) -> Result<()> {
        println!("Cleaning and setting tags: {}", new_tags);
        let tags = new_tags
            .trim()
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let re = Regex::new(r"[^a-zA-Z0-9\s]")?;
        // iterate through each tag and remove all special chars, replace with whitespace
        for tag in &tags {
            let new_tag = re.replace_all(&tag, " ").to_string();
            self.tags.push(new_tag);
        }
        Ok(())
    }

    pub fn write_post_to_disk(&self, site: &SiteDetails) -> Result<()> {
        println!("Writing post to disk: {}", self.filename);
        let new_posts_path = SiteDetails::build_site_path(&site)?
            .join("md_posts")
            .join(format!("{}.md", self.filename));
        fs::write(new_posts_path.clone(), &self.content).context("Failed to write to file.")?;

        // open the post file written to disk and write the title and timestamp to the file
        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(new_posts_path)
            .context("Failed to open file.")?;

        let post_content = format!(
            "date:{}\nexcerpt:{}\nimage:{}\ntags:{}\n# {}", 
            self.date,
            "Write cool excerpt here",
            "https://images.unsplash.com/photo-1615147342761-9238e15d8b96?ixid=MXwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHw%3D&ixlib=rb-1.2.1&auto=format&fit=crop&w=1001&q=80",
            self.tags.join(","),
            self.title,
        );

        file.write_all(post_content.as_bytes())
            .context("Failed to write to file.")?;

        println!("Post `{}` was created successfully. Edit your new file in: {}", self.title, self.filename);

        Ok(())
    }

    pub fn commit_post_to_repo(site: &SiteDetails, message: &str) -> Result<()> {
        println!("Committing post to repo: {}", message);
        let site_path = SiteDetails::build_site_path(&site)?;
        let repo = Repository::open(site_path)?;
        let signature = Signature::now("Driftwood", "driftwood@example.com")?;
        let mut index = repo.index()?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;
        repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?;
        Ok(())
    }
}

impl SiteDetails {
    pub fn build_site_path(&self) -> Result<PathBuf> {
        let site_path = PathBuf::from(format!(
            "sites/{}_{}",
            self.name.clone().unwrap(),
            self.id.clone().unwrap()
        ));
        Ok(site_path)
    }

    pub fn create_site_repo(&self) -> Result<()> {
        let repo_path = SiteDetails::build_site_path(self)?;
        let repo = Repository::open(repo_path)?;
        Ok(())
    }

    pub fn check_for_site_repo(&self) -> Result<bool> {
        let repo_path = SiteDetails::build_site_path(self)?;
        let repo = Repository::open(repo_path);
        Ok(repo.is_ok())
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

    // Create a vector to store post data
    let mut post_data: Vec<(String, String, String, Vec<String>)> = Vec::new();

    // First pass: extract date and other information
    for post in &posts {
        let post_file_path = Path::new(post);
        let post_file = fs::read_to_string(post_file_path)?;
        let mut date = String::new();
        let mut tags = Vec::new();

        for line in post_file.lines() {
            println!(">> Line >> : {}", line.trim());
            if line.trim().starts_with("<p>date:") {
                date = line.replace("<p>date:", "").trim().to_string();
            } else if line.trim().starts_with("tags:") {
                tags = line
                    .replace("tags:", "")
                    .trim()
                    .split(",")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                println!("Tags: {:?}", tags);
            }
        }

        post_data.push((post.clone(), date, post_file, tags));
    }

    // Sort posts by date in descending order (newest first)
    post_data.sort_by(|a, b| b.1.cmp(&a.1));

    // iterate through all the Posts
    for (post, date, post_file, tags) in post_data {
        println!("Tags: {:?}", tags);
        println!(">> Post: {}", post);
        let post_file_path = Path::new(&post);
        let mut excerpt = String::new();
        let mut image = String::new();
        let mut new_post_file = String::new();

        // extract the date, excerpt, and image from the post_file
        for line in post_file.lines() {
            if line.trim().starts_with("<p>date:") {
                continue;
            } else if line.trim().starts_with("excerpt:") {
                excerpt.push_str(line.replace("excerpt:", "").trim());
                continue;
            } else if line.trim().starts_with("image:") {
                image.push_str(line.replace("image:", "").trim());
                continue;
            } else if line.trim().starts_with("tags:") {
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
            filename: format!("posts/{}", post_file_name),
            title: post_title.clone(),
            date: date.clone(),
            excerpt: excerpt,
            image: image,
            sitename: site_name.clone(),
            tags: tags.join(", "),
        };
        println!("Blog card context: {}", blog_card_context.tags);

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
    }

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

pub struct Git {}
impl Git {
    pub fn init_git_repo(site_path: &str) -> Result<Repository, git2::Error> {
        let path = Path::new(site_path);
        Repository::init(path)
    }

    pub fn commit_changes(repo: &Repository, message: &str) -> Result<(), git2::Error> {
        let signature = Signature::now("Driftwood", "driftwood@example.com")?;
        let mut index = repo.index()?;
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;

        let parents = match repo.head() {
            Ok(head) => vec![head.peel_to_commit()?],
            Err(_) => vec![],
        };

        let parent_refs: Vec<&git2::Commit> = parents.iter().collect();

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            parent_refs.as_slice(),
        )?;

        Ok(())
    }

    pub fn view_commit_history(repo_path: &str, limit: usize) -> Result<(), git2::Error> {
        let repo = Repository::open(repo_path)?;
        let mut revwalk = repo.revwalk()?;
        revwalk.push_head()?;
        revwalk.set_sorting(git2::Sort::TIME)?;

        println!("Commit History (Last {} commits):", limit);
        println!("--------------------------------");

        for (i, oid) in revwalk.take(limit).enumerate() {
            let oid = oid?;
            let commit = repo.find_commit(oid)?;

            let time = commit.time();
            let dt: chrono::DateTime<chrono::Utc> =
                chrono::TimeZone::timestamp_opt(&chrono::Utc, time.seconds(), 0).unwrap();

            println!("Commit:     {}", oid);
            println!("Author:     {}", commit.author());
            println!("Date:       {}", dt.format("%Y-%m-%d %H:%M:%S"));
            println!(
                "Message:    {}",
                commit.message().unwrap_or("No commit message")
            );
            println!("--------------------------------");
        }

        Ok(())
    }
}

pub struct OAuth2 {}
impl OAuth2 {
    pub fn get_env_var(name: &str) -> Result<String, env::VarError> {
        let var = env::var(name)?;
        Ok(var)
    }
}
