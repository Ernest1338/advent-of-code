#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn part1(input_file: String) {
    let crabs: Vec<usize> = input_file
        .trim()
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut fuel_costs: Vec<usize> = vec![];
    let positions_to_test: usize = 500;
    let mut cur_fuel_cost: usize = 0;
    for position in 1..=positions_to_test {
        for crab in &crabs {
            cur_fuel_cost += (crab.to_owned() as i32 - position as i32).abs() as usize;
        }
        fuel_costs.push(cur_fuel_cost);
        cur_fuel_cost = 0;
    }
    let answer: usize = fuel_costs.iter().min().unwrap().to_owned();
    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let crabs: Vec<usize> = input_file
        .trim()
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut fuel_costs: Vec<usize> = vec![];
    let positions_to_test: usize = 500;
    let mut cur_fuel_cost: usize = 0;
    let mut tmp_fuel: usize;
    for position in 1..=positions_to_test {
        for crab in &crabs {
            tmp_fuel = (crab.to_owned() as i32 - position as i32).abs() as usize;
            for i in 1..=tmp_fuel {
                cur_fuel_cost += i;
            }
        }
        fuel_costs.push(cur_fuel_cost);
        cur_fuel_cost = 0;
    }
    let answer: usize = fuel_costs.iter().min().unwrap().to_owned();
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
