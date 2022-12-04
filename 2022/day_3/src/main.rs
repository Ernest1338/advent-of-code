#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn value_of(item: char) -> usize {
    let lower = ('a'..='z').into_iter().collect::<Vec<char>>();
    let upper = ('A'..='Z').into_iter().collect::<Vec<char>>();
    if lower.contains(&item) {
        lower.iter().position(|&e| e == item).unwrap() + 1
    } else if upper.contains(&item) {
        upper.iter().position(|&e| e == item).unwrap() + 27
    } else {
        0
    }
}

fn part1(input_file: String) {
    let mut answer: usize = 0;
    let input_file = input_file.trim();
    for line in input_file.split('\n').collect::<Vec<&str>>() {
        let line_len_half = line.len() / 2;
        let first_half = &line[..line_len_half];
        let second_half = &line[line_len_half..];
        for item in first_half.chars() {
            if second_half.contains(item) {
                // println!("adding: {} of value {}", item, value_of(item));
                answer += value_of(item);
                break;
            }
        }
    }
    print_answer(answer);
}

fn part2(input_file: String) {
    let mut answer: usize = 0;
    let input_file = input_file.trim();
    let lines = input_file.split('\n').collect::<Vec<&str>>();
    for i in (0..lines.len()).step_by(3) {
        let first = lines[i];
        let second = lines[i + 1];
        let third = lines[i + 2];
        for item in first.chars() {
            if second.contains(item) && third.contains(item) {
                answer += value_of(item);
                break;
            }
        }
    }
    print_answer(answer);
}

fn main() {
    let part = 2;

    let input_file = load_file("input.txt");

    if part == 1 {
        part1(input_file);
    } else if part == 2 {
        part2(input_file);
    }
}
