// used for parsing metadata
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use yaml_front_matter::YamlFrontMatter;

// used for parsing markdown
use pulldown_cmark::{Options, Parser};

// used for templating
use handlebars::{to_json, Handlebars};
use serde_json::{json, Value as Json};

use std::{
    ffi::OsStr,
    fs::{self, read_to_string},
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
    path: String,
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

    fn add_file_path(self, file_name: &str, render_tree: impl Render) -> Self {
        let html = render_tree.render(&self.templates);
        let file = file_name.to_owned() + ".html";
        let path = self.output.clone().join(file);
        fs::write(path, html).unwrap();
        self
    }

    fn add_content_folder_path<T: Serialize + DeserializeOwned>(
        self,
        folder_name: &str,
        content_render_tree: impl Render,
    ) -> Self {
        let output_folder = self.output.clone().join(folder_name);
        fs::create_dir_all(&output_folder).unwrap();
        for file in fs::read_dir(self.content.clone().join(folder_name)).unwrap() {
            let f = file.unwrap();
            let file_path = f.path();
            let file_stem = file_path.file_stem().unwrap();
            let file_name = file_stem.to_str().unwrap();
            let file_name = file_name;
            let html = content_render_tree.render_for_file::<T>(
                &self.templates,
                &self.content,
                &(folder_name.to_owned() + file_name),
            );
            let file = file_name.to_owned() + ".html";
            let path = output_folder.join(file);
            fs::write(path, html).unwrap();
        }
        self
    }
}

trait Render {
    fn handlebars() -> Handlebars<'static> {
        Handlebars::new()
    }
    fn render(&self, template_folder: &Path) -> String;
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize,
        T: DeserializeOwned;
}

impl<C> Render for (&str, C)
where
    C: Render,
{
    fn render(&self, template_folder: &Path) -> String {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        Self::handlebars()
            .render_template(
                &template,
                &json!({"content": self.1.render(template_folder)}),
            )
            .unwrap()
    }
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize,
        T: DeserializeOwned,
    {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        Self::handlebars()
            .render_template(
                &template,
                &json!({"content": self.1.render_for_file::<T>(template_folder, content_folder, file_name)}),
            )
            .unwrap()
    }
}

impl Render for &str {
    fn render(&self, template_folder: &Path) -> String {
        let file = self.to_owned().to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        Self::handlebars()
            .render_template(&template, &json!({}))
            .unwrap()
    }
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize + DeserializeOwned,
    {
        let file = self.to_owned().to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        let file = file_name.to_owned() + ".md";
        let (html, md) = parse_markdown(&content_folder.join(file));
        let mut json = to_json(get_metadata::<T>(md));
        merge(&mut json, &json!({"content": html}));
        Self::handlebars()
            .render_template(&template, &json)
            .unwrap()
    }
}

impl Render for (&str, Json) {
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize,
        T: DeserializeOwned,
    {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        let file = file_name.to_owned() + ".md";
        let (html, md) = parse_markdown(&content_folder.join(file));
        let mut json = to_json(get_metadata::<T>(md));
        merge(&mut json, &self.1);
        merge(&mut json, &json!({"content": html}));
        Self::handlebars()
            .render_template(&template, &json)
            .unwrap()
    }
    fn render(&self, template_folder: &Path) -> String {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(&file)).expect(&format!(
            "Can't find file {:?}",
            template_folder.join(&file)
        ));
        Self::handlebars()
            .render_template(&template, &self.1)
            .unwrap()
    }
}

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

impl<C> Render for (&str, Json, C)
where
    C: Render,
{
    fn render(&self, template_folder: &Path) -> String {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        let mut json = self.1.clone();
        merge(
            &mut json,
            &json!({"content": self.2.render(template_folder)}),
        );
        Self::handlebars()
            .render_template(&template, &json)
            .unwrap()
    }
    fn render_for_file<T>(
        &self,
        template_folder: &Path,
        content_folder: &Path,
        file_name: &str,
    ) -> String
    where
        T: Serialize,
        T: DeserializeOwned,
    {
        let file = self.0.to_owned() + ".html";
        let template = fs::read_to_string(template_folder.join(file)).unwrap();
        let mut json = self.1.clone();
        merge(
            &mut json,
            &json!({"content": self.2.render_for_file::<T>(template_folder, content_folder, file_name)}),
        );
        Self::handlebars()
            .render_template(&template, &json)
            .unwrap()
    }
}

fn parse_markdown(md_file: &Path) -> (String, String) {
    let md = read_to_string(md_file).expect(&format!("Can't find file {:?}", md_file));
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

fn get_all_metadata<T: DeserializeOwned>(md_folder: &'static str) -> Vec<T> {
    let mut res = Vec::new();
    for file in fs::read_dir(md_folder).unwrap() {
        let file = file.unwrap().path();
        let content = fs::read_to_string(file).unwrap();
        let meta = get_metadata::<T>(content);
        res.push(meta)
    }
    res
}

fn main() {
    MarkdownSiteGenerator::default()
        .add_file_path(
            "index",
            (
                "base",
                json!({"title": "Blog"}),
                (
                    "index",
                    (
                        "post_list",
                        json!({"posts": get_all_metadata::<Metadata>("./content/posts")}),
                    ),
                ),
            ),
        )
        .add_content_folder_path::<Metadata>("posts/", ("base", "post"));
}
