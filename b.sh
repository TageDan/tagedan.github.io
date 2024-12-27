#!/bin/sh


#infinite

if [ $1 = "-h" ]; then
  echo "usage: ./b.sh [flags]"
  echo "flags:"
  echo "-r	repeat forever"
  echo "-s	host simple webserver in background"
  echo "-rs	do both -r and -s"
  echo "-h	print thus help message"
  fi

exec > /dev/null 2>&1
case "$1" in

  "-r")
	  while true
	do
	cargo run --release
	npx tailwindcss -i ./public/css/input.css -o ./public/css/output.css	
	done
	;;


	
	"-s")
	live-server ./public -q &
	cargo run --release
	npx tailwindcss -i ./public/css/input.css -o ./public/css/output.css	
	;;

	
	"-rs")
	live-server ./public -q & 
	while true
	do
	cargo run --release
	npx tailwindcss -i ./public/css/input.css -o ./public/css/output.css	
	done
;;
	


esac
