// #![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;
use regex::Regex;

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    for line in input_file.lines() {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        for captures in re.captures_iter(line) {
            let num1: usize = captures.get(1).map_or("", |m| m.as_str()).parse().unwrap();
            let num2: usize = captures.get(2).map_or("", |m| m.as_str()).parse().unwrap();
            answer += num1 * num2;
        }
    }
    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let mut enabled = true;
    for line in input_file.lines() {
        let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

        let mut pos = 0;
        while pos < line.len() {
            let cur = &line[pos..];
            if cur.starts_with("do()") {
                enabled = true;
                pos += "do()".len();
            } else if cur.starts_with("don't()") {
                enabled = false;
                pos += "don't()".len();
            } else if cur.starts_with("mul(") {
                let cap_mul = re_mul.captures(cur).unwrap();
                let start = pos + cap_mul.get(0).unwrap().start();
                if enabled && start - pos == 0 {
                    let num1: usize = cap_mul[1].parse().unwrap();
                    let num2: usize = cap_mul[2].parse().unwrap();
                    answer += num1 * num2;
                }
                pos += "mul()".len();
            } else {
                pos += 1;
            }
        }
    }
    answer
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
