#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn part1(input_file: String) -> usize {
    let mut answer: usize = 0;
    for line in input_file.lines() {
        let left = line.split(',').collect::<Vec<&str>>()[0];
        let left_split = left
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let right = line.split(',').collect::<Vec<&str>>()[1];
        let right_split = right
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if (left_split[0] <= right_split[0] && left_split[1] >= right_split[1])
            || (right_split[0] <= left_split[0] && right_split[1] >= left_split[1])
        {
            answer += 1;
        }
    }
    answer
}

fn part2(input_file: String) -> usize {
    let mut answer: usize = 0;
    for line in input_file.lines() {
        let left = line.split(',').collect::<Vec<&str>>()[0];
        let left_split = left
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let right = line.split(',').collect::<Vec<&str>>()[1];
        let right_split = right
            .split('-')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let left_range = left_split[0]..=left_split[1];
        let right_range = right_split[0]..=right_split[1];
        if (right_range.contains(&left_split[0]) || right_range.contains(&left_split[1]))
            || (left_range.contains(&right_split[0]) || left_range.contains(&right_split[1]))
        {
            answer += 1;
        }
    }
    answer
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
