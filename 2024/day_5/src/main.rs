#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

enum RuleOutcome {
    Passed,
    Failed,
    None,
}

fn check_rule(rule: &[usize], update: &[usize]) -> RuleOutcome {
    // println!("{update:?}\t{rule:?}");

    let left_index = match update.iter().position(|&x| x == rule[0]) {
        Some(v) => v,
        None => return RuleOutcome::None,
    };

    let right_index = match update.iter().position(|&x| x == rule[1]) {
        Some(v) => v,
        None => return RuleOutcome::None,
    };

    // println!("L: {left_index:?} R: {right_index:?}");

    if left_index > right_index {
        return RuleOutcome::Failed;
    }

    RuleOutcome::Passed
}

fn all_rules_apply(rules: &[Vec<usize>], update: &[usize]) -> bool {
    for rule in rules {
        let res = check_rule(rule, update);
        if let RuleOutcome::Failed = res {
            return false;
        }
    }
    true
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let input_split = input_file.split("\n\n").collect::<Vec<&str>>();
    let rules: Vec<Vec<usize>> = input_split[0]
        .lines()
        .map(|l| l.split("|").map(|e| e.parse().unwrap()).collect())
        .collect();
    let updates: Vec<Vec<usize>> = input_split[1]
        .lines()
        .map(|l| l.split(",").map(|e| e.parse().unwrap()).collect())
        .collect();

    for update in &updates {
        // println!("checking {update:?}");
        if !all_rules_apply(&rules, update) {
            continue;
        }
        // println!("passed");

        let middle = update[(update.len() - 1) / 2];
        answer += middle;
    }

    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let input_split = input_file.split("\n\n").collect::<Vec<&str>>();
    let rules: Vec<Vec<usize>> = input_split[0]
        .lines()
        .map(|l| l.split("|").map(|e| e.parse().unwrap()).collect())
        .collect();
    let updates: Vec<Vec<usize>> = input_split[1]
        .lines()
        .map(|l| l.split(",").map(|e| e.parse().unwrap()).collect())
        .collect();

    for update in &updates {
        let mut update = update.clone();
        let mut fixed = false;
        // println!("handling update: {update:?}");
        while !all_rules_apply(&rules, &update) {
            for rule in &rules {
                if let RuleOutcome::Failed = check_rule(rule, &update) {
                    // println!("failed rule: {rule:?}");

                    // switch places

                    let left_index = match update.iter().position(|&x| x == rule[0]) {
                        Some(v) => v,
                        None => continue,
                    };

                    let right_index = match update.iter().position(|&x| x == rule[1]) {
                        Some(v) => v,
                        None => continue,
                    };

                    update.swap(left_index, right_index);
                    fixed = true;
                }
            }
        }
        // println!("fixed update: {update:?}");
        let middle = update[(update.len() - 1) / 2];
        // println!("middle: {middle:?}");
        if fixed {
            answer += middle;
        }
    }

    answer
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
