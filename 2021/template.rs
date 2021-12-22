#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn part1(input_file: String) {
    let mut answer: usize = 0;
    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let mut answer: usize = 0;
    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 1;

    let input_file: String = read_to_string("sample.txt").unwrap();

    if part == 1 {
        part1(input_file);
    } else if part == 2 {
        part2(input_file);
    }
}
