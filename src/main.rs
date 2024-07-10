use pulldown_cmark::{Options, Parser};

use std::fs::{self, read_dir, read_to_string, write};

const MARKDOWN_FOLDER: &'static str = "./posts";
const TEMPLATE_FOLDER: &'static str = "./templates";
const OUTPUT_FOLDER: &'static str = "./public";

fn main() {
    let md_files = read_dir(MARKDOWN_FOLDER);
    for md_file in md_files.unwrap() {
        let md_file = md_file.unwrap();
        let file_name = md_file.file_name();
        let file_name = file_name.to_str().unwrap();
        let file_name = file_name.split(".").take(1).next().unwrap();
        let md = read_to_string(md_file.path()).unwrap();
        let meta_data_options = Options::ENABLE_YAML_STYLE_METADATA_BLOCKS;
        let parser = Parser::new_ext(&md, meta_data_options);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        let content = format!("{}", html);
        write(&format!("{}/{}.html", OUTPUT_FOLDER, file_name), content);
    }
}
