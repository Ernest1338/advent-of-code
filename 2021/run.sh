#!/bin/bash

dir=$1

if [ ! -d "./$dir" ] # directory does not exist, creating the file structure
then
    mkdir "./$dir" &&
    touch "./$dir/main.rs" &&
    echo "fn main() {}" > "./$dir/main.rs" &&
    echo "created the file structure"
fi

if [ ! -d "./$dir/target" ] # target directory does not exist
then
    mkdir "./$dir/target" &&
    echo "target directory created"
fi

rustc "./$dir/main.rs" -o "./$dir/target/main.x" && ./$dir/target/main.x
