#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn parse_line(lines: &mut std::str::Lines) -> Vec<usize> {
    lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap())
        .collect()
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 1;
    let mut lines = input_file.lines();
    let times = parse_line(&mut lines);
    let distances = parse_line(&mut lines);
    for (time, record) in times.iter().zip(&distances) {
        let mut winning = 0;
        for speed in 0..*time {
            if (time - speed) * speed > *record {
                winning += 1;
            }
        }
        answer *= winning;
    }
    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 1;
    let mut lines = input_file.lines();
    let times = parse_line(&mut lines);
    let mut time_str = String::new();
    for t in &times {
        time_str.push_str(&t.to_string());
    }
    let time = time_str.parse::<usize>().unwrap();
    let distances = parse_line(&mut lines);
    let mut record_str = String::new();
    for d in distances {
        record_str.push_str(&d.to_string());
    }
    let record = record_str.parse::<usize>().unwrap();
    let mut winning = 0;
    for speed in 0..time {
        if (time - speed) * speed > record {
            winning += 1;
        }
    }
    answer *= winning;
    answer
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
