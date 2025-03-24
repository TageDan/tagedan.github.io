---
title: "Fantastic Flamegraphs"
author: "Tage Danielsson"
description: "How I used flamegraphs to optimize my chess engine"
date: "2024-12-27"
tags:
  - "Tools"
  - "Programing"
  - "Optimizations"
path: "/blog/cpu_profiling_using_flamegraphs.html"
name: "cpu_profiling_using_flamegraphs"
---

## Intro
Lately I have been building a [project](https://github.com/TageDan/Chlang) for which I needed a chess engine. I decided to make it my
very own by building the board representation from scratch. But after finishing the basic representation to make it possible to play games I soon realised I had a *BIG* performance problem. Some games would even cause my computer to crash. This felt like the perfect time to test out a tool I've been wanting to try: *[Flamegraph](https://github.com/flamegraph-rs/flamegraph)*

## What is a flamegraph
A flamegraph is a chart over the different stackframes encountered in a program. It can help you to visualize how your program runs and highlight the bottlenecks.
Each bar in the chart represents a stack frame (basically a function) in your program and the wider bar under it is it's parent (the function which called this function).
The width of each box represents how much of the total time of the program was spent in that function. The "app" stackframe is therefore 100% of the width becuase it covers the whole program. If 50% of the program is spent inside some function then that box will be half of the charts width.

An example of a flamechart could look like this.
![MySQL Flamechart]()

## How did I use flamegraphs
As I said My board representation was really slow and to find out why I wanted to use flamegraph.
First of I created a benchmark using another great tool, [hyperfine](https://github.com/sharkdp/hyperfine) and this is what i got.
 * Benchmark 1: 456.5 ± 14.1 ms (over 10 runs)

And then I ran `cargo flamegraph` to produce my chart which came out like this:
![My first flamechart]()

