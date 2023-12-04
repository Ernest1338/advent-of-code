#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn parse_line(line: &str) -> (Vec<usize>, Vec<usize>) {
    let winning_nums = line
        .split('|')
        .next()
        .unwrap()
        .split_whitespace()
        .skip(2)
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let nums = line
        .split('|')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    (winning_nums, nums)
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    for line in input_file.lines() {
        let (winning_nums, nums) = parse_line(line);
        let mut worth = 0;
        for n in &winning_nums {
            if nums.contains(n) {
                if worth == 0 {
                    worth = 1;
                } else {
                    worth *= 2;
                }
            }
        }
        answer += worth;
    }
    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let mut cards = vec![1; input_file.lines().collect::<Vec<&str>>().len() + 1];
    for (i, line) in input_file.lines().enumerate() {
        let (winning_nums, nums) = parse_line(line);
        let mut how_many_match = 0;
        for n in winning_nums {
            if nums.contains(&n) {
                how_many_match += 1;
            }
        }
        for x in 0..cards[i] {
            for c in cards.iter_mut().take(i + how_many_match + 1).skip(i + 1) {
                *c += 1;
            }
        }
    }
    cards.iter().rev().skip(1).sum()
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
