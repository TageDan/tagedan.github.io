#!/bin/sh

  cargo run --release
  npx tailwindcss -i ./public/css/input.css -o ./public/css/output.css  
