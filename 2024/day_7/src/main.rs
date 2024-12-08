#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;
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

fn append_number(a: usize, b: usize) -> usize {
    if b == 0 {
        return a * 10;
    }

    let mut digits_in_b = 0;
    let mut temp_b = b;

    while temp_b > 0 {
        temp_b /= 10;
        digits_in_b += 1;
    }

    a * 10_usize.pow(digits_in_b as u32) + b
}

fn check_possibilities(equation: &Equation, symbols: &[char]) -> bool {
    // println!("[checking] {equation:?}");
    let combinations_len = equation.numbers.len() - 1;

    let mut combination = vec![symbols[0]; combinations_len];
    let mut indices = vec![0; combinations_len];

    loop {
        // println!("combination: {combination:?}");
        let mut sum = equation.numbers[0];
        let mut valid = true;

        for (i, nums) in equation.numbers.windows(2).enumerate() {
            let symbol = combination[i];
            match symbol {
                '+' => sum += nums[1],
                '*' => sum *= nums[1],
                '|' => sum = append_number(sum, nums[1]),
                _ => (),
            }
            if sum > equation.target {
                valid = false;
                break;
            }
        }

        if valid && sum == equation.target {
            // println!("[result] true\t{equation:?}\t{combination:?}\n");
            return true;
        }

        let mut carry = true;
        for i in (0..combinations_len).rev() {
            if carry {
                indices[i] += 1;
                if indices[i] < symbols.len() {
                    combination[i] = symbols[indices[i]];
                    carry = false;
                } else {
                    indices[i] = 0;
                    combination[i] = symbols[0];
                }
            }
        }

        if carry {
            break;
        }
    }

    // println!("[result] false\t{equation:?}\n");
    false
}

fn part1(input_file: &str) -> usize {
    let equations = parse(input_file);

    equations
        .par_iter()
        .filter(|equation| check_possibilities(&equation, &['*', '+']))
        .map(|equation| equation.target)
        .sum()
}

fn part2(input_file: &str) -> usize {
    let equations = parse(input_file);

    equations
        .par_iter()
        .filter(|equation| check_possibilities(&equation, &['*', '+', '|']))
        .map(|equation| equation.target)
        .sum()
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
