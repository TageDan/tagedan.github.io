#!/bin/sh

#infinite
while [ "$1" = "t" ]
do

# Compile tailwind
npx tailwindcss -i ./public/css/input.css -o ./public/css/output.css

# Compile and run rust
cargo run

done

# Compile tailwind
npx tailwindcss -i ./public/css/input.css -o ./public/css/output.css

# Compile and run rust
cargo run
