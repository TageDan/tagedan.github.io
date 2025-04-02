#!/bin/sh
find src content templates src -type f | entr -n "./build.sh"
