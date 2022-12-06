#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn find_header(input: &str, num_of_unique: usize) -> usize {
    let chrs: Vec<_> = input.trim().chars().collect();
    for (i, chunk) in chrs.windows(num_of_unique).enumerate() {
        let mut deduped = chunk.to_vec();
        deduped.sort();
        deduped.dedup();
        if deduped.len() == num_of_unique {
            return i + num_of_unique;
        }
    }
    0
}

fn part1(input_file: String) -> usize {
    find_header(&input_file, 4)
}

fn part2(input_file: String) -> usize {
    find_header(&input_file, 14)
}

fn main() {
    let part = 2;

    let input_file = load_file("input.txt");

    if part == 1 {
        print_answer(part1(input_file));
    } else if part == 2 {
        print_answer(part2(input_file));
    }
}
