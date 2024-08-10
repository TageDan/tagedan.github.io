---
title: "Static Site Generation using Rust"
author: "Tage Danielsson"
description: "How I made my own framework to generate the static pages on this site using rust"
date: "2024-07-12"
tags:
  - "rust"
  - "html"
  - "handlebars"
  - "static site generation"
  - "markdown"
path: "/posts/static_site_generation_rust.html"
---

I have for a long time wanted to build a website where I could have my own blog and project portfolio.
To acheive this the first thing that came to mind was to spin up a fast server in python (since I have some 
experience using flask) and than host it with something like python anywhere. 
But then I realised that the site I wanted to create (this site) barely needed any interactivity
and it would be unneccesary too write a server for it. Instead I looked into static site generation tools like
[HUGO](https://gohugo.io/) since I knew i wanted to easely be able to convert markdown content into static pages
wich I could then host using a static-site-server (something like [Github Pages](https://pages.github.com/)).
I decided that this would be a little bit boring since it seemed to easy. Instead I wanted to create my own 
static site generation framework called stat-site-framework using my very favorite language rust.

That was all the backstory to how this started. Next up, let's talk dependencies! 
I am a strong believer that in most cases it's best to choose dependencies that are 
specialized on the one task that you want it to do for you. This way you know why you're 
using the dependencies you're using and you know when to reach for them. You also reduce 
your applications overhead and compile times by choosing smaller specialized dependencies.
That said the dependencies I chose were:
```
[dependencies]
  pulldown-cmark = "0.11.0" # used to parse markdown files into html
  handlebars = "5.1.2" # templating engine for generating html
  yaml-front-matter = "0.1.0" # Used for extracting metadata from markdown files
  serde = "1.0.204" # Used for Serializing and Deserializing metadata and json data
  serde_json = "1.0.120" # used to handle json data (required by handlebars)
```

Now let's look at how you might use the framework to get a better Idea of 
what structures and functions this framework provides and what the api looks like.
This is the code (rust code) for this website:
``` 
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
        .add_file_path("portfolio", ("base", json!({"title": "TageDan"}), ("index")));
}
```
And the file structure looks something like this:

* blog/
    * src/
        * main.rs
    * content/
        * posts/
            * (markdown content files)
    * public/
        * (outputed static files)

