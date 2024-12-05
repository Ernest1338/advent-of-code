#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

enum RuleOutcome {
    Passed,
    Failed,
    None,
}

fn find_indices(rule: &[usize], update: &[usize]) -> Option<(usize, usize)> {
    let left_index = update.iter().position(|&x| x == rule[0])?;
    let right_index = update.iter().position(|&x| x == rule[1])?;

    Some((left_index, right_index))
}

fn check_rule(rule: &[usize], update: &[usize]) -> RuleOutcome {
    if let Some((left_index, right_index)) = find_indices(rule, update) {
        if left_index > right_index {
            RuleOutcome::Failed
        } else {
            RuleOutcome::Passed
        }
    } else {
        RuleOutcome::None
    }
}

fn all_rules_apply(rules: &[Vec<usize>], update: &[usize]) -> bool {
    rules
        .iter()
        .all(|rule| !matches!(check_rule(rule, update), RuleOutcome::Failed))
}

fn parse_input(input_file: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let input_split = input_file.split("\n\n").collect::<Vec<&str>>();
    let rules: Vec<Vec<usize>> = input_split[0]
        .lines()
        .map(|l| l.split("|").map(|e| e.parse().unwrap()).collect())
        .collect();
    let updates: Vec<Vec<usize>> = input_split[1]
        .lines()
        .map(|l| l.split(",").map(|e| e.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let (rules, updates) = parse_input(input_file);

    for update in &updates {
        if !all_rules_apply(&rules, update) {
            continue;
        }
        let middle = update[(update.len() - 1) / 2];
        answer += middle;
    }

    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let (rules, mut updates) = parse_input(input_file);

    for update in &mut updates {
        let mut fixed = false;

        while !all_rules_apply(&rules, &update) {
            for rule in &rules {
                if let RuleOutcome::Failed = check_rule(rule, &update) {
                    // switch places
                    if let Some((left_index, right_index)) = find_indices(rule, &update) {
                        update.swap(left_index, right_index);
                        fixed = true;
                    }
                }
            }
        }

        if fixed {
            let middle = update[(update.len() - 1) / 2];
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
