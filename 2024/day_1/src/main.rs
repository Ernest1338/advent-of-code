#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn parse_lists(input: &str) -> (Vec<isize>, Vec<isize>) {
    let (mut left, mut right) = (Vec::new(), Vec::new());

    for line in input.lines() {
        let mut line_iter = line.split_whitespace();

        left.push(line_iter.next().unwrap().parse::<isize>().unwrap());
        right.push(line_iter.next().unwrap().parse::<isize>().unwrap());
    }

    (left, right)
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let (mut left, mut right) = parse_lists(input_file);

    left.sort();
    right.sort();

    for (l, r) in left.iter().zip(right.iter()) {
        answer += (l - r).unsigned_abs();
    }

    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let (mut left, mut right) = parse_lists(input_file);

    for n in left {
        answer += n as usize * right.iter().filter(|&&x| x == n).count();
    }

    answer
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
