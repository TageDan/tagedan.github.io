#!/bin/sh

#infinite
while [ "$1" = "t" ]
do


# Compile and run rust
cargo run


# Compile tailwind
npx tailwindcss -i ./public/css/input.css -o ./public/css/output.css

done

# Compile and run rust
cargo run

# Compile tailwind
npx tailwindcss -i ./public/css/input.css -o ./public/css/output.css

