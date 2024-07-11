// used for parsing metadata
use serde::Deserialize;
use yaml_front_matter::YamlFrontMatter;

// used for parsing markdown
use pulldown_cmark::{Options, Parser};

// used for templating
use handlebars::Handlebars;
use serde_json::json;

use std::fs::{self, read_dir, read_to_string, write};

const MARKDOWN_FOLDER: &'static str = "./posts";
const TEMPLATE_FOLDER: &'static str = "./templates";
const OUTPUT_FOLDER: &'static str = "./public";

// Setting metadata paramaters
#[derive(Deserialize)]
struct Metadata {
    title: String,
    author: String,
    description: String,
    tags: Vec<String>,
    date: String,
}

fn main() {
    let handlebars = Handlebars::new();

    // Getting all md-posts
    let md_files = read_dir(MARKDOWN_FOLDER);

    // Getting templates for rendering
    let index_template = read_to_string(&format!("{}/index.html", TEMPLATE_FOLDER)).unwrap();
    let blog_post_card_template =
        read_to_string(&format!("{}/preview.html", TEMPLATE_FOLDER)).unwrap();
    let blog_post_template = read_to_string(&format!("{}/post.html", TEMPLATE_FOLDER)).unwrap();

    // Iterating over md_files to create static html files
    for md_file in md_files.unwrap() {
        let md_file = md_file.unwrap();

        // Getting file_name for post
        let file_name = md_file.file_name();
        let file_name = file_name.to_str().unwrap();
        let file_name = file_name.split(".").take(1).next().unwrap();

        // Parsing markdown using pulldown_cmark
        let md = read_to_string(md_file.path()).unwrap();
        let meta_data_options = Options::ENABLE_YAML_STYLE_METADATA_BLOCKS;
        let parser = Parser::new_ext(&md, meta_data_options);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        // Getting metadata for post
        let result = YamlFrontMatter::parse::<Metadata>(&md).unwrap();

        let Metadata {
            title,
            description,
            author,
            tags,
            date,
        } = result.metadata;

        // Generating new content using handlebars
        let content = handlebars
            .render_template(
                &blog_post_template,
                &json!({"title": title, "author": author, "date": date, "content": html }),
            )
            .unwrap();

        // Writing content to static html_file
        let _ = write(
            &format!("{}/posts/{}.html", OUTPUT_FOLDER, file_name),
            content,
        )
        .unwrap();
    }
}
