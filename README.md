Hi, this is the source code from my blog.
It uses a static site generetion framework that i made in rust, the main purpose of the framework is to parse markdown files, generate html files at the right paths, and create rendering tree's so that i can reuse html templates easier.
The actual setup with the framework is really short (in `src/main.rs`), the actual files hosted on the website is found in the `public/` directory and all the markdown content can be found at the `content/` directory. There are also some simple bashscripts in here wich I use for my workflow when creating new content.
