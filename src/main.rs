use serde::{Deserialize, Serialize};
use serde_json::json;
use stat_site_framework::*;

// Post metadata
#[derive(Deserialize, Serialize)]
struct Metadata {
    title: String,
    author: String,
    description: String,
    tags: Vec<String>,
    date: String,
    path: String,
}

// Project Metadata
#[derive(Deserialize, Serialize)]
struct ProjectMetaData {
    title: String,
    description: String,
    tags: Vec<String>,
    path: String,
    img: String,
}

fn main() {
    generator::MarkdownSiteGenerator::default()
        .add_file_path(
            "blog",
            (
                "base",
                json!({"title": "Blog"}),
                (
                    "blog",
                    (
                        "post_list",
                        json!({"posts": markdown_utils::get_all_metadata::<Metadata>("./content/posts")}),
                    ),
                ),
            ),
        )
        .add_content_folder_path::<Metadata>("posts/", ("base", "post"))
        .add_file_path("index", ("base", json!({"title": "TageDan"}), ("index")))
        .add_file_path("portfolio", ("base", json!({"title": "Portfolio"}), ("portfolio", json!({"project": markdown_utils::get_all_metadata::<ProjectMetaData>("./content/projects")}))));
}
