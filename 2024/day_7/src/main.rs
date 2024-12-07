#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug)]
struct Equation {
    target: usize,
    numbers: Vec<usize>,
}

fn parse(input: &str) -> Vec<Equation> {
    let mut out = Vec::new();
    for l in input.lines() {
        let mut numbers = Vec::new();
        let split: Vec<&str> = l.split(": ").collect();
        let target: usize = split[0].parse().unwrap();
        for n in split[1].split_whitespace() {
            numbers.push(n.parse().unwrap());
        }
        out.push(Equation { target, numbers });
    }
    out
}

fn check_possibilities(equation: &Equation, symbols: &[char]) -> bool {
    println!("[checking] {equation:?}");
    let combinations_len = equation.numbers.len() - 1;
    let combinations = symbols
        .iter()
        .copied()
        .combinations_with_replacement(combinations_len)
        .flat_map(|combo| combo.into_iter().permutations(combinations_len))
        .unique();
    for combination in combinations {
        let mut sum = equation.numbers[0];
        for (i, nums) in equation.numbers.windows(2).enumerate() {
            let symbol = combination[i];
            match symbol {
                '+' => {
                    sum += nums[1];
                }
                '*' => {
                    sum *= nums[1];
                }
                '|' => {
                    sum = sum * 10 + nums[1];
                }
                _ => (),
            }
            if sum > equation.target {
                break;
            }
        }
        // println!("sum: {sum}\tcombination: {combination:?}");
        if sum == equation.target {
            println!("[result] true\t{equation:?}");
            return true;
        }
    }
    println!("[result] false\t{equation:?}");
    false
}

fn part1(input_file: &str) -> usize {
    let equations = parse(input_file);
    let mut answer = 0;

    equations
        .par_iter()
        .filter(|equation| check_possibilities(&equation, &['+', '*']))
        .map(|equation| equation.target)
        .sum()
}

fn part2(input_file: &str) -> usize {
    let equations = parse(input_file);
    let mut answer = 0;

    equations
        .par_iter()
        .filter(|equation| check_possibilities(&equation, &['+', '*', '|']))
        .map(|equation| equation.target)
        .sum()
}

fn main() {
    let input_file = load_file("input.txt");

    // println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
