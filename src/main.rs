// used for parsing metadata
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yaml_front_matter::YamlFrontMatter;

// used for parsing markdown
use pulldown_cmark::{Options, Parser};

// used for templating
use handlebars::{to_json, Handlebars};
use serde_json::{json, Value as Json};

use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, read_dir, read_to_string, write},
    path::{Path, PathBuf},
};

// Setting metadata paramaters
#[derive(Deserialize, Serialize)]
struct Metadata {
    title: String,
    author: String,
    description: String,
    tags: Vec<String>,
    date: String,
}

struct MarkdownSiteGenerator {
    content: PathBuf,
    templates: PathBuf,
    output: PathBuf,
}

impl Default for MarkdownSiteGenerator {
    fn default() -> Self {
        Self {
            content: Path::new("./content").to_owned(),
            templates: Path::new("./templates").to_owned(),
            output: Path::new("./public").to_owned(),
        }
    }
}

impl MarkdownSiteGenerator {
    fn with_dirs<S>(content: Option<&S>, templates: Option<&S>, output: Option<&S>) -> Self
    where
        S: AsRef<OsStr>,
    {
        let (content, templates, output) = (
            {
                if let Some(x) = content {
                    Path::new(x).to_owned()
                } else {
                    Self::default().content
                }
            },
            {
                if let Some(x) = templates {
                    Path::new(x).to_owned()
                } else {
                    Self::default().templates
                }
            },
            {
                if let Some(x) = output {
                    Path::new(x).to_owned()
                } else {
                    Self::default().output
                }
            },
        );

        Self {
            content,
            templates,
            output,
        }
    }

    fn generate(&mut self) {
        let handlebars = Handlebars::new();
        let mut templates: HashMap<String, String> = HashMap::new();
        self.get_templates(&mut templates);
        let md_files = read_dir(self.content.clone());
        for md_file in md_files.unwrap() {
            let md_file = md_file.unwrap();
            if md_file.path().is_dir() {
                continue;
            }

            // Getting file_name for post

            let file_name = md_file.path();
            let file_name = file_name.file_stem().unwrap();
            // Parsing markdown using pulldown_cmark
            let (html, md) = parse_markdown(md_file);

            // Getting metadata for post

            let Metadata {
                title,
                author,
                description,
                tags,
                date,
            } = get_metadata::<Metadata>(md);

            // Generating new content using handlebars
            let content = handlebars
                .render_template(
                    &templates.get("post").unwrap(),
                    &json!({"title": title, "author": author,  "content": html }),
                )
                .unwrap();

            let content = handlebars
                .render_template(
                    &templates.get("base").unwrap(),
                    &json!({"title": title, "content": content}),
                )
                .unwrap();

            // Writing content to static html_file
            let _ = write(
                self.output
                    .join(self.content.clone())
                    .join(&format!("{}.html", file_name.to_str().unwrap())),
                content,
            )
            .unwrap();
        }
    }

    fn get_templates(&self, templates: &mut HashMap<String, String>) {
        let template_files = read_dir(self.templates.clone()).unwrap();
        for file in template_files {
            let file = file.unwrap();
            if file.path().is_dir() {
                continue;
            }
            let file_name = String::from(file.path().file_stem().unwrap().to_str().unwrap());
            let html_string = read_to_string(file.path()).unwrap();
            templates.insert(file_name, html_string);
        }
    }

    fn add_file_path(&self, arg: &str, content_metadata_list: impl Render) -> Self {
        todo!()
    }

    fn add_markdown_path(&self) -> Self {
        todo!()
    }
}

trait Render {}

impl<C> Render for (&str, C) where C: Render {}

impl Render for &str {}

impl Render for (&str, Json) {}

fn merge(a: &mut Json, b: &Json) {
    match (a, b) {
        (&mut Json::Object(ref mut a), &Json::Object(ref b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Json::Null), v);
            }
        }
        (a, b) => {
            *a = b.clone();
        }
    }
}

impl<C> Render for (&str, Json, C) where C: Render {}

fn parse_markdown(md_file: fs::DirEntry) -> (String, String) {
    let md = read_to_string(md_file.path()).unwrap();
    let meta_data_options = Options::ENABLE_YAML_STYLE_METADATA_BLOCKS;
    let parser = Parser::new_ext(&md, meta_data_options);
    let mut html = String::new();
    pulldown_cmark::html::push_html(&mut html, parser);
    (html, md)
}

fn get_metadata<T>(md: String) -> T
where
    T: DeserializeOwned,
{
    let result = YamlFrontMatter::parse::<T>(&md).unwrap();
    result.metadata
}

fn get_all_metadata<T>() -> Vec<T> {
    todo!()
}

fn main() {
    let mut generator = MarkdownSiteGenerator::default()
        .add_file_path(
            "index",
            (
                "base",
                json!({"title": "Blog"}),
                (
                    "index",
                    ("post_list", json!({"posts":get_all_metadata::<Metadata>()})),
                ),
            ),
        )
        .add_markdown_path();
}
