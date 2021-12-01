#!/bin/bash

dir=$1
arg1=$2
arg2=$3
arg3=$4

if [[ $dir == "" ]] # if the directory is empty, show the usage information
then
    echo "Usage: ./run.sh <day number>" && exit
fi

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

rustc "./$dir/main.rs" -o "./$dir/target/main.x" && ./$dir/target/main.x $arg1 $arg2 $arg3
