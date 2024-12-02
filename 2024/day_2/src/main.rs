#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn is_safe(report: &[isize]) -> bool {
    let mut s: Vec<char> = Vec::new();
    for i in report.windows(2) {
        let dif = i[1] - i[0];
        s.push(if dif > 0 { '+' } else { '-' });
        if !(-3..=3).contains(&dif) || dif == 0 {
            return false;
        }
    }
    if !s.iter().all(|&x| x == s[0]) {
        return false;
    }
    true
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    for line in input_file.lines() {
        let report = line
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        if is_safe(&report) {
            answer += 1;
        }
    }
    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    for line in input_file.lines() {
        let levels_len = line.split_whitespace().count();
        for i in 0..levels_len {
            let mut report = line
                .split_whitespace()
                .collect::<Vec<&str>>()
                .iter()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>();
            report.remove(i);
            if is_safe(&report) {
                answer += 1;
                break;
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
