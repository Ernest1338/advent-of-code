#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn part1(input_file: String) {
    let mut answer: usize = 0;
    print_answer(answer);
}

fn part2(input_file: String) {
    let mut answer: usize = 0;
    print_answer(answer);
}

fn main() {
    let part = 1;

    let input_file = load_file("sample.txt");

    if part == 1 {
        part1(input_file);
    } else if part == 2 {
        part2(input_file);
    }
}
