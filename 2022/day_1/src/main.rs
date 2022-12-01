#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn part1(input_file: String) {
    let mut answer: usize;
    let mut sum = 0;
    let mut highest = 0;
    for line in input_file.split('\n') {
        if line != "" {
            sum += line.parse::<usize>().unwrap_or(0);
        } else {
            if sum > highest {
                highest = sum;
            }
            sum = 0;
        }
    }
    answer = highest;
    print_answer(answer);
}

fn part2(input_file: String) {
    let mut answer: usize;
    let mut sum = 0;
    let mut highest_list = vec![];
    for line in input_file.split('\n') {
        if line != "" {
            sum += line.parse::<usize>().unwrap_or(0);
        } else {
            highest_list.push(sum);
            sum = 0;
        }
    }
    highest_list.sort();
    highest_list.reverse();
    answer = highest_list[0] + highest_list[1] + highest_list[2];
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
