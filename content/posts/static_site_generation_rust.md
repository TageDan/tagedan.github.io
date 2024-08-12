---
title: "Static Site Generation using Rust"
author: "Tage Danielsson"
description: "How I made my own framework to generate the static pages on this site using rust"
date: "2024-08-12"
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
experience using [flask](https://flask.palletsprojects.com/en/3.0.x/)) and then host it using something like [python anywhere](https://www.pythonanywhere.com). 
I realised that the site I wanted to create (this site) barely needed any interactivity
and that it would be unneccesary too create a server for it. Instead I looked into static site generation tools like
[HUGO](https://gohugo.io/) since I wanted to easily convert markdown content into static pages
which I could then host using a static-site-server (something like [Github Pages](https://pages.github.com/)).
I decided that this would be a little bit too boring since it seemed so easy. Instead I decided to create my own 
static site generation framework called stat-site-framework using my very favorite language rust.

That is the backstory to why I created the framework. Next up, let's talk dependencies! 
I believe that in most cases it's best to choose dependencies that are 
specialized on the one task that you want it to do for you. This way you know why you're 
using the dependencies you're using and when to reach for them. You can also reduce 
your applications overhead and compile times by choosing smaller and more specialized dependencies.
That said the dependencies I chose were:
```toml
[dependencies]
  pulldown-cmark = "0.11.0" # used to parse markdown files into html
  handlebars = "5.1.2" # templating engine for generating html
  yaml-front-matter = "0.1.0" # Used for extracting metadata from markdown files
  serde = "1.0.204" # Used for Serializing and Deserializing metadata and json data
  serde_json = "1.0.120" # used to handle json data (required by handlebars)
```

Now let's look at how you might use the framework to get a better Idea of 
what structures and functions this framework provides and what the api looks like.
This is the rust code (possibly old code) for this website:
```rust 
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

// Project metadata
#[derive(Deserialize, Serialize)]
struct ProjectMetadata {
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
        .add_file_path("portfolio", ("base", json!({"title": "Portfolio"}), ("portfolio", json!({"project": markdown_utils::get_all_metadata::<ProjectMetadata>("./content/projects")}))));
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

On the first three lines we import our dependencies. Once again were using serde/serde_json to handle our data.
We also import * (which means everything basically) from stat_site_framework. Then we define two datatypes,
these are the datatypes that the markdown metadata will be passed into, hence the '#[derive(Deserialize, Serialize)]'.
Now we enter the main function. Here we construct a MarkdownSiteGenerator struct with default parameters to 
generate our pages. Let's take a closer look at that struct now.
```rust
pub struct MarkdownSiteGenerator {
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
```
As we can see it's a really small struct containing only the output directory and the directories 
for the templates and markdown content we want to use. We also implement the Default trait for ease of 
use (There is also a with_dirs method that I wont get into).

Now that we've seen the basic structure of the generator we can finally start generating a page.
To start of simple we will look at how we can do simple templating to generate a page without a 
markdown file. Take a look at the first method we call on the generator:
```rust
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
```
This method call might look really weird. It starts by taking in a string which will 
be the subpath to the outputted html in the output directory (in this case base.html in ./public).
Then comes the weird part, this is where the magic happens, by nesting these tuples we can define a 
rendering hierarchy. The result of a nested tuple will be rendered in place of '{{{ content }}}' 
in the template with the same name as the leading string in the tuple. For example:

These files located in the template directory:
```html
<!-- (base.html) -->

<h1>Image gallery</h1>
<div>
{{{ content }}}
</div>
```
```html
<!-- (index.html) -->

<p>This is a image gallery :)</p>
<div>{{{ content }}}</div>
```
```html
<!-- (images.html) -->

<h3>Images by <a href="https://github.com/TageDan">Tage Danielsson</a></h3>
<img src="..." />
<img src="..." />
<img src="..." />
<img src="..." />
```
And this simple tree
```rust
("base", ("index", "images"))
```
Would output the file:
```html
<h1>Image gallery</h1>
<div>
    <p>This is a image gallery :)</p>
    <div>
        <h3>Images by <a href="https://github.com/TageDan">Tage Danielsson</a></h3>
        <img src="..." />
        <img src="..." />
        <img src="..." />
        <img src="..." />
    </div>
</div>
```

Tell me that's not cool, I mean it's simple but still a really nice way to define your 
rendering in my opinion. But how does it work? 
Well, it really doesn't have to do with anything unique about tuples. The only reason 
that this type of tuple structure works is becuase I've implemented the rendering method for it. That is the real 
magic of this framework, it's really extendible (at least in theory, haven't done that much yet).
The second method argument accepts anything that implements the render trait which means that you can
define any structure and then define any implementation for rendering it and you will be 
able to pass it to the function directly or in the middle of the nested tuples. That's the power of rust's traits.

Let's look at how I decided to implement the render trait for the tuple structure.
```rust
pub trait Render {
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
        handlebars()
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
        handlebars()
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
        handlebars()
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
        handlebars()
            .render_template(&template, &json)
            .unwrap()
    }
}
```
We see that the render trait defines two functions: 
render and render_for_file which is used for markdown folders. They both return strings but the return type might 
change in the future since I'm thinking of "bubbling up" the metadata through the tree.
Anyways, I've also included the implementations for two types. There are a couple of more implementations but I think this 
will be enough to get a good idea of the concept I had in mind for the trait.

Starting of with the implementation for `(&str, C) where C: Render` we can see that we simply pass along the template folder 
(which was passed by the MarkdownSiteGenerator struct) to render C 
and then we render the template refered to by the string setting "content" to the output of C's render. 
We do a similar thing for the render_for_file method but this 
time we pass down the content folder and the name of the markdown file we'll get our data/content from.

At the bottom of the render tree we usually end up with a single string. In this case we want to 
render the template directly and as we see the implementation does just that. It loads the template,
renders it passing an empty json object and then returns the result. In the case where we use the render_for_file function 
we also use the extra parameters that we passed down to load and parse the markdown file, note that we also have a type parameter 'T' here 
which is essential for parsing the markdown metadata. Pass the data we got 
from the markdown file to handlebars when rendering and return the result. That's it! It's by making these kind of 
trait implementations that we are able to get the recursive/nesting nature of the tuples.

Now that we know how the render trait works the hardest part is done. 
The last thing we'll look at is the defenition of the `add_file_path` and `add_content_folder_path` methods.
```rust
pub fn add_file_path(self, file_name: &str, render_tree: impl Render) -> Self {
    let html = render_tree.render(&self.templates);
    let file = file_name.to_owned() + ".html";
    let path = self.output.clone().join(file);
    fs::write(path, html).unwrap();
    self
}

pub fn add_content_folder_path<T: Serialize + DeserializeOwned>(
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
```
These functions are much simpler than you might think. That is becuase most of the work 
is done by our implementation of the Render trait. All we have to do is to call the render/render_for_file 
method on the root of the render tree, pass it the right parameters from the generator struct and 
then write the result to the appropriate file location. In the case of the 
content folder we just need to do this for every file in the folder we take as an argument.

That's all I'm going to go through in this article. The source code for the entire project is available on [github](https://github.com/TageDan/stat-site-framework)
if you want to go through it yourself. As a last point I would like to say that I'm thinking of 
expanding the project a little by also providing functions for setting up the static file server 
in the same file. This way you could easily expand the behaivor to be more dynamic by adding a 
frontend library like [htmx](https://htmx.org/) and use the same Render trait to create responses on the server. 
If you think that would be cool to see, noticed an error in the article our 
have any feedback at all, then please [email me](mailto:danielsson.dev@gmail.com).
