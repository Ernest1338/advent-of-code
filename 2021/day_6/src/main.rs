#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn vec_usize_count(vec: Vec<usize>) -> usize {
    let mut count: usize = 0;
    for value in vec {
        count += value;
    }
    return count;
}

fn vec_cycle(vec: Vec<usize>) -> Vec<usize> {
    let mut result_vec: Vec<usize> = vec.to_owned();
    for i in 0..vec.len() {
        if i + 1 == vec.len() {
            result_vec[i] = vec[0];
        } else {
            result_vec[i] = vec[i + 1];
        }
    }
    return result_vec;
}

fn part1(input_file: String) {
    let answer: usize;
    let initial_state_str: &str = &input_file.trim();
    let mut lanternfishes: Vec<u8> = initial_state_str
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    let mut fishes_to_add: Vec<u8> = vec![];
    let number_of_days: u8 = 80;
    for _day in 0..number_of_days {
        for fish_index in 0..lanternfishes.len() {
            if lanternfishes[fish_index] > 0 {
                lanternfishes[fish_index] -= 1;
            } else {
                lanternfishes[fish_index] = 6;
                fishes_to_add.push(8);
            }
        }
        for fish in &fishes_to_add {
            lanternfishes.push(fish.to_owned());
        }
        fishes_to_add.clear();
    }
    answer = lanternfishes.len();
    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let answer: usize;
    let mut fish_timer_lookup: Vec<usize> = vec![0; 9];
    let initial_state: Vec<u8> = input_file
        .trim()
        .split(",")
        .map(|n| n.parse::<u8>().unwrap())
        .collect();
    for fish_timer in initial_state {
        fish_timer_lookup[fish_timer as usize] += 1;
    }
    let number_of_days: u16 = 256;
    let mut tmp_fishes: usize;
    for _day in 1..=number_of_days {
        tmp_fishes = fish_timer_lookup[0];
        fish_timer_lookup = vec_cycle(fish_timer_lookup);
        fish_timer_lookup[6] += tmp_fishes;
        //println!("End of day: {}: {}", _day, vec_usize_count(fish_timer_lookup.to_owned()));
    }
    answer = vec_usize_count(fish_timer_lookup);
    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 {
        part1(input_file);
    } else if part == 2 {
        part2(input_file);
    }
}
