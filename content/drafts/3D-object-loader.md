---
title: "Create a 3D-Terminal Renderer (part 2)"
author: "Tage Danielsson"
description: "Add camera rotation, .obj file loading and parallelisation using rayon."
date: ""
tags:
  - "rust"
  - "3D"
  - "obj"
  - "parsing"
path: "/posts/terminal-rendering-2.html"
---

# Introduction
This is the second post about terminal rendering. Start by reading
[the first part](https://tagedan.github.io/posts/terminal_rendering.html)
and then come back to this when you are done with that.
In this part we will create a 3D-file loader, add some camera 
controls to view our meshes from different angles and speed up the rendering.

# Setup
We will add some new files for this part. 
This will be our updated file-structure:
```
terminal-renderer
+-- Cargo.toml
+-- src
|   +-- lib.rs
|   +-- main.rs
|   +-- loader
|   |   +-- mod.rs
|   +-- math
|   |   +-- mod.rs
|   +-- renderer
|   |   +-- mod.rs
```
We will also change up our dependencies a little.
We'll remove [term_size](https://crates.io/crates/term_size)
and add [rayon](https://crates.io/crates/rayon)
and [crossterm](https://crates.io/crates/crossterm). 
The updated `Cargo.toml` file looks like:
```toml
[dependencies]
crossterm = "0.28.1" # Terminal Size, RawMode and input
rayon = "1.10.0" # Parallelisation
vec3-rs = "0.1.6" #Vector operations
```

That's it for the setup. Now we can start loading a .obj file.

# Structure of a .obj file
In this post we will keep it simple by only considering 
.obj files without textures or vertex normals.

Let's look at a really simple example.

```wavefront
# tri.obj

v -1.0 -1.0 0.0
v 1.0 -1.0 0.0
v 0.0 1.0 0.0

f 1 2 3
```
This file represent a single triangle face. Each line starts with a word 
telling us what the line represents. In our case these words are 'v' (vertex) and 'f' (face).
For each vertex we then find 3 values representing it's xyz coordinates and for each face
we find 3 values indicating the index of it's first, second and third vertex. (note that this index is one based)

Now that we know the structure of a obj file, let's try to load it as a mesh.

# Parsing the lines
 
Start by creating a loading function and an error enum.
```rust
// loader/mod.rs

#[derive(Clone, Copy)]
pub enum MeshError {}


pub fn load_obj<P: AsRef<Path>>(path: P) -> Result<Mesh, MeshError> {
}
```
This function takes a file path to load and returns the a result 
containing the mesh or an error describing what went wrong.

We will initialize a empty vector of vertices and triangles.
Then we will read the content of the file. (note that this can fail 
and that we have added two errors `FileNotFoundError` and `UTF8Error` to our enum)
```rust
// loader/mod.rs

#[derive(Clone, Copy)]
pub enum MeshError {
    FileNotFoundError,
    UTF8Error,
}

pub fn load_obj<P: AsRef<Path>>(path: P) -> Result<Mesh, MeshError> {
    let mut verts = Vec::new();
    let mut tris = Vec::new();

    let file = std::fs::read(path).map_err(|err| MeshError::FileNotFoundError)?;
    let content = String::from_utf8(file).map_err(|err| MeshError::UTF8Error)?;  
```

The `?` operator here will give us the value contained by the `Ok` variant of a result just like an `unwrap` would. 
The difference is that `?` propagates the error by returning it from the function instead of panicing like an `unwrap` would.
If used correctly this can make for some really clean and simple code
(This is the first time I'm using it in a real project so I'm not saying my way is the right way here). 

We will then start looping through the lines of the file and split it on spaces.
We can then match the first part of the string to determine if it's a vertex or a face.

```rust
for line in content.lines() {
    let mut parts = line.split(' ');
    match parts.next() {
        Some(t) => match t {
            "v" => {
                // TODO: add_vertex(&mut verts, parts)?;
            }
            "f" => {
                // TODO: add_face(&mut tris, &verts, parts)?;
            }
            _ => (),
        },
        None => (),
    };
}
```

When we have filled our tris vector with vertices we can convert it into a mesh and return it from the function.

```rust
Ok(Mesh::new(tris))
```

We now just need to parse the rest of our lines. (the values)

## Parsing vertices
To parse a vertex we will create a function called `add_vertex`.

```rust
fn add_vertex(
    verts: &mut Vec<Vector3<f64>>,
    mut parts: std::str::Split<char>,
) -> Result<(), MeshError> {
```

This function will take a mutable refrence to our list of vertices and the 
remaining parts of the line as it's parameters (the "v" was consumed by the call to `next()` when we matched)
and it will return a result containing a `MeshError` in the case of an error and a unit '()' in the case of a success.

We will expect three coordinates so we can create three variables `c1`, `c2` and `c3` convert them to numbers and add a new vertex out of them. 
If there is too few or too many vertices, or if we fail to parse the coordinate strings, we will return an `InvalidMeshError`. 
```rust
fn add_vertex(
    verts: &mut Vec<Vector3<f64>>,
    mut parts: std::str::Split<char>,
) -> Result<(), MeshError> {
    // Coordinate 1
    let c1 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let c1: f64 = c1.parse().map_err(|_| MeshError::InvalidMeshError)?;

    // Coordinate 2
    let c2 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let c2: f64 = -c2.parse().map_err(|_| MeshError::InvalidMeshError)?;

    // Coordinate 3
    let c3 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let c3: f64 = c3.parse().map_err(|_| MeshError::InvalidMeshError)?;

    // Too many coordinates
    if parts.next() != None {
        return Err(MeshError::InvalidMeshError);
    }

    verts.push(Vector3::new(c1, c2, c3));

    Ok(())
}   
```

The `.ok_or` method is used to convert a `Option` into a `Result` 
type by converting the `Some(T)` case into `Ok(t)` and the 
`None` case into `Err(E)` where `E` is `InvalidMeshError` in our case.

The `.map_err` method is used to convert one error type to another. 
We use this to convert the error of the parse function into an `InvalidMeshError`.

The last part checks if there are still more parts. If there is we got too many 
coordinates and we return an error, if there's not we push a new vertex to our vertices and return an `Ok(())`

Now we can parse vertices, let's do faces!


## Parsing faces
Just like vertices, we're starting by defining the function `add_face`.

This function will accept a mutable refrence to a list of triangles (our faces), 
a list of vertices, and the iterator for the remaining parts of the line. 
It will have the same return type as the `add_vertex` function.

This function will have almost the same logic as the `add_vertex` function. The difference 
is that we can have more than three indices for the vertices in a face. 
```rust
fn add_face(
    tris: &mut Vec<Tri>,
    verts: &[Vec3],
    mut parts: std::str::Split<char>,
) -> Result<(), MeshError> {
    let v1 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let v1: usize = v1
        .parse::<usize>()
        .map_err(|_| MeshError::InvalidMeshError)?
        - 1;
    let v2 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let v2: usize = v2
        .parse::<usize>()
        .map_err(|_| MeshError::InvalidMeshError)?
        - 1;
    let v3 = parts.next().ok_or(MeshError::InvalidMeshError)?;
    let v3: usize = v3
        .parse::<usize>()
        .map_err(|_| MeshError::InvalidMeshError)?
        - 1;

    tris.push(Tri::new(
        *verts.get(v1).ok_or(MeshError::InvalidMeshError)?,
        *verts.get(v2).ok_or(MeshError::InvalidMeshError)?,
        *verts.get(v3).ok_or(MeshError::InvalidMeshError)?,
        Vec3::new(255., 255., 255.),
    ));

    Ok(())
}
```


