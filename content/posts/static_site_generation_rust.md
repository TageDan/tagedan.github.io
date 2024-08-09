---
title: "Static Site Generation using Rust"
author: "Tage Danielsson"
description: "Using rust to generate static html"
date: "2024-07-12"
tags:
  - "rust"
  - "html"
  - "handlebars"
  - "static site generation"
  - "markdown"
name: "Hej, test"
---

I've wanted to have my own personal website/blog for a while. 
I enjoy making projects in my spare time but since I put so much time into making them I thought it would help me to share the process.
I also wanted a space where i could write about my other intrests like running, fishing, maths and traveling. 
Last but not least I wanted to have a portfolio for my projects.

Getting started building the site I knew that I wanted to use rust. It has been my prefered language for about a year and I really enjoy learning it.
In the begining I looked at using axum to build a server that would serve my html and handle backend-logic but i soon realised that i didn't want 
any features that required a backend (no comments, likes, etc) and instead i turned my head towards projects like [HUGO](https://gohugo.io/) that generate 
that only focus on generating the static content that can simply be hosted using static website hosting like [Github Pages](https://pages.github.com/) which is what I use for this site.
Inspired by HUGO I set out to create my own static site generation framework in rust. The framework is still very much in progress but it has come to the point where I was able to create this blog using it, 
and so I decided that I'm going to document the development starting today.

So far the project includes a couple of dependencies and a single rust binary crate. 
In the future I intend to split this file up to seperate the concerns of the framework but for now I'm still experimenting and 
I am not concerned of writing "good code" since I will refoactor everything later anyways.

Let's talk about the dependencies. To me it's important that I know what purpose each of my direct dependencies have and that 
I don't use big dependencies with a lot off features that I'm not going to use. The dependencies I've added so far are as follows

```
[dependencies]
  pulldown-cmark = "0.11.0"
  handlebars = "5.1.2"
  yaml-front-matter = "0.1.0"
  serde = "1.0.204"
  serde_json = "1.0.120"
```

Each of these dependencies have their own responsibility in the project. In this case handlebars allows me to use 
html templating, pulldown-cmark helps me parse the markdown and yaml-front-matter is used to retrieve the 
metadata of my markdown that I can parse using serde and serde_json.


