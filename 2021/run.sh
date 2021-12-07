#!/bin/bash

day=$1
arg1=$2
arg2=$3
arg3=$4

if [[ $day == "" ]] # if the directory is empty, show the usage information
then
    echo "Usage: ./run.sh <day number>" && exit
fi

if [ ! -d "./day_$day" ] # directory does not exist, creating the file structure
then
    cargo new --vcs=none --quiet "day_$day" &&
    touch "./day_$day/input.txt" &&
    touch "./day_$day/sample.txt" &&
    cat "./template.rs" > "day_$day/src/main.rs" &&
    echo "[!] created the file structure"
fi

(cd "./day_$day" && cargo run --quiet $arg1 $arg2 $arg3)
