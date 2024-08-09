use serde::{Deserialize, Serialize};
use serde_json::json;
use stat_site_framework::*;

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

fn main() {
    generator::MarkdownSiteGenerator::default()
        .add_file_path(
            "blog",
            (
                "base",
                json!({"title": "Blog"}),
                (
                    "index",
                    (
                        "post_list",
                        json!({"posts": markdown_utils::get_all_metadata::<Metadata>("./content/posts")}),
                    ),
                ),
            ),
        )
        .add_content_folder_path::<Metadata>("posts/", ("base", "post"))
        .add_file_path("index", ("base", json!({"title": "TageDan"}), ("index")));
}
