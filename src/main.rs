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
    name: String,
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

#[derive(Deserialize, Serialize)]
struct ProblemMetadata {
    no: usize,
    solution_path: String,
    img_link: String,
    problem: String,
}

fn main() {
    let mut problems = markdown_utils::get_all_metadata::<ProblemMetadata>("./content/problems");
    problems.sort_unstable_by_key(|x| usize::MAX - x.no);

    generator::MarkdownSiteGenerator::default()
        .add_file_path(
            "blog",
            (
                "base",
                json!({"title": "Blog", "path": "/blog"}),
                (
                    "blog",
                    json!({"posts": markdown_utils::get_all_metadata::<Metadata>("./content/blog")}),
                ),
            ),
        )
        .add_content_folder_path::<Metadata>("blog/", ("base", "post"))
        .add_file_path("index", ("base", json!({"title": "TageDan", "path": ""}), ("index")))
        .add_file_path("portfolio", ("base", json!({"title": "Portfolio"}), ("portfolio", json!({"project": markdown_utils::get_all_metadata::<ProjectMetadata>("./content/projects")}))))
        .add_file_path("problems", ("base", json!({"title": "Problems"}), ("problems", json!({"problem": problems}))))
    .add_content_folder_path::<ProblemMetadata>("problems/", ("base", "solution"));
}
// struct MyToiletWrapper<A: render::Render>(A);

// impl<A: render::Render> MyToiletWrapper<A> {
//     fn update_headers(&self, init_s: String) -> String {
//         let mut n_string = String::new();

//         for line in init_s.lines() {
//             if line.trim().starts_with("<h1>") {
//                 let data = line.trim();
//                 let main_part = &data[4..data.len() - 5];
//                 let main_part = decode(main_part.as_bytes()).to_string().unwrap();

//                 println!(r#"figlet "{}""#, main_part);

//                 let out = std::process::Command::new("sh")
//                     .arg("-c")
//                     .arg(&format!(
//                         r#"toilet -f future "{}" -w 100"#,
//                         main_part
//                     ))
//                     .output()
//                     .unwrap();

//                 let res = String::from_utf8(out.stdout).unwrap();
//                 // println!("{}", res);

//                 let len = res.lines().count();

//                 for l in &res.lines().collect::<Vec<_>>()[0..(len)] {
//                     n_string.push_str("<h1>");

//                     let l = htmlentity::entity::encode(l.as_bytes(), &htmlentity::entity::EncodeType::Named, &htmlentity::entity::CharacterSet::Html).to_string().unwrap();

//                     n_string.push_str(&l);
//                     println!("{l}");

//                     n_string.push_str("</h1>\n");
//                 }
//             } else {
//                 n_string.push_str(line);
//                 n_string.push('\n');
//             }
//         }

//         n_string
//     }
// }

// impl<A: render::Render> render::Render for MyToiletWrapper<A> {
//     fn render(&self, template_folder: &std::path::Path) -> String {
//         let initial_string = self.0.render(template_folder);
//         self.update_headers(initial_string)
//     }

//     fn render_for_file<T>(
//         &self,
//         template_folder: &std::path::Path,
//         content_folder: &std::path::Path,
//         file_name: &str,
//     ) -> String
//     where
//         T: Serialize,
//         T: serde::de::DeserializeOwned,
//     {
//         let initial_string =
//             self.0
//                 .render_for_file::<T>(template_folder, content_folder, file_name);
//         self.update_headers(initial_string)
//     }
// }
