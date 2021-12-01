#!/bin/bash

dir=$1
arg1=$2
arg2=$3
arg3=$4

if [[ $dir == "" ]] # if the directory is empty, show the usage information
then
    echo "Usage: ./run.sh <day number>" && exit
fi

if [ ! -d "./day_$dir" ] # directory does not exist, creating the file structure
then
    cargo new --vcs=none --quiet "day_$dir" &&
    touch "./day_$dir/input1.txt" &&
    touch "./day_$dir/input2.txt" &&
    echo "[!] created the file structure"
fi

(cd "./day_$dir" && cargo run --quiet $arg1 $arg2 $arg3)
