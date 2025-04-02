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
struct ProjectMetadata {
    title: String,
    description: String,
    tags: Vec<String>,
    path: String,
    img: String,
}

// Problem Metadata
#[derive(Deserialize, Serialize)]
struct ProblemMetadata {
    no: usize,
    title: String,
    path: String,
}

fn main() {
    let mut problems = markdown_utils::get_all_metadata::<ProblemMetadata>("./content/problem");
    problems.sort_unstable_by_key(|x| usize::MAX - x.no);

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
        .add_file_path("portfolio", ("base", json!({"title": "Portfolio"}), ("portfolio", json!({"project": markdown_utils::get_all_metadata::<ProjectMetadata>("./content/projects")}))))
        .add_file_path("problems", ("base", json!({"title": "Problems"}), ("problems", json!({"problem": problems }))))
        .add_content_folder_path::<ProblemMetadata>("problem/", ("base", "problem"));
}
