---
title: "Build a VM in Rust"
author: "Tage Danielsson"
description: "How to build a LC-3 virtual machine using rust"
date: ""
tags:
  - "rust"
  - "virtual machine"
  - "LC-3"
path: "/posts/build_a_vm_in_rust.html"
---

In this post were building a virtual machine that can run LC-3 machine code.
LC-3 stands for Little Computer 3 and is used primarily for educational purposes. 
This post is based on [this article](https://www.jmeiners.com/lc3-vm/) and my goal is to 
rewrite his C implementation in rust. Checkout the other post if you want a more in depth explanation.

Before we jump into the code I would like to highlight some of the benefits with virtual machines.
Virtual machines can be used to emulate hardware devices, 
in this case we are acctually trying to create a virtual computer that 
would possibly handle all sorts of input/output. Another use case is to act as a 
common interface with the computer, by writing multiple small vm's supporting the same machine code 
you are able to target multiple computer architectures without compiling 
to another instruction for each one. Instead of compiling for both ARM and x86/64 
you could simply compile to LC-3 and run it with their respective virtual machines.
We can also utilise that the virtual machine can see all the memory of the programs it's running
to implement garbage collection.

Now we know why someone would like to build a virtual machine. Let's do it!
The virtual machine will have to store 3 things:
* memory
* registers
* condition flags

We will also define the instruction set for the machine.
Let's put it into code:
```rust

```
