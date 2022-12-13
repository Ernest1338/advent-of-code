#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn iterate_round(
    monkeys: &[Vec<&str>],
    monkey_items: &mut [Vec<usize>],
    monkey_item_specs: &mut Vec<usize>,
    part_1: bool,
) {
    for (monkey_n, monkey) in monkeys.iter().enumerate() {
        let operation: Vec<&str> = monkey[2].split("= old ").collect::<Vec<&str>>()[1]
            .split(' ')
            .collect();
        let test: usize = monkey[3].split("by ").collect::<Vec<&str>>()[1]
            .parse()
            .unwrap();
        let throw_table = vec![
            monkey[4]
                .chars()
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap(),
            monkey[5]
                .chars()
                .last()
                .unwrap()
                .to_string()
                .parse::<usize>()
                .unwrap(),
        ];
        let monkey_items_copy = monkey_items.to_owned();
        for item in &monkey_items_copy[monkey_n] {
            monkey_item_specs[monkey_n] += 1;
            let mut item_after_operation: usize = 0;
            let operation_value: usize;
            if operation[1] == "old" {
                operation_value = *item;
            } else {
                operation_value = operation[1].parse().unwrap();
            }
            match operation[0] {
                "+" => {
                    if operation[1] == "old" {
                        if part_1 {
                            item_after_operation = (item + item) / 3;
                        } else {
                            item_after_operation = item + item;
                        }
                    } else {
                        if part_1 {
                            item_after_operation = (item + operation_value) / 3;
                        } else {
                            item_after_operation = item + operation_value;
                        }
                    }
                }
                "*" => {
                    if operation[1] == "old" {
                        if part_1 {
                            item_after_operation = (item * item) / 3;
                        } else {
                            item_after_operation = item * item;
                        }
                    } else {
                        if part_1 {
                            item_after_operation = (item * operation_value) / 3;
                        } else {
                            item_after_operation = item * operation_value;
                        }
                    }
                }
                _ => {}
            };
            monkey_items[monkey_n].pop();
            if &item_after_operation % &test == 0 {
                monkey_items[throw_table[0]].push(item_after_operation);
            } else {
                monkey_items[throw_table[1]].push(item_after_operation);
            }
        }
    }
}

fn populate_monkey_items(monkeys: &Vec<Vec<&str>>, monkey_items: &mut Vec<Vec<usize>>) {
    for monkey in monkeys {
        monkey_items.push(
            monkey[1].split(": ").collect::<Vec<&str>>()[1]
                .split(", ")
                .collect::<Vec<&str>>()
                .iter()
                .map(|item| item.parse::<usize>().unwrap())
                .collect(),
        );
    }
}

fn part1(input_file: String) -> usize {
    let mut answer: usize;
    let input_file = input_file.trim();
    let monkeys: Vec<Vec<&str>> = input_file
        .split("\n\n")
        .map(|monkey| {
            monkey
                .split('\n')
                .map(|each| each.trim())
                .collect::<Vec<&str>>()
        })
        .collect();
    let num_of_monkeys = monkeys.len();
    let mut monkey_item_specs: Vec<usize> = vec![0; num_of_monkeys];
    let mut monkey_items: Vec<Vec<usize>> = Vec::new();
    populate_monkey_items(&monkeys, &mut monkey_items);
    for _ in 0..20 {
        iterate_round(&monkeys, &mut monkey_items, &mut monkey_item_specs, true);
    }
    monkey_item_specs.sort();
    monkey_item_specs.reverse();
    answer = monkey_item_specs[0] * monkey_item_specs[1];
    answer
}

fn part2(input_file: String) -> usize {
    // TODO: find a way to simplify numbers (to avoid multiply with overflow)
    let mut answer: usize;
    let input_file = input_file.trim();
    let monkeys: Vec<Vec<&str>> = input_file
        .split("\n\n")
        .map(|monkey| {
            monkey
                .split('\n')
                .map(|each| each.trim())
                .collect::<Vec<&str>>()
        })
        .collect();
    let num_of_monkeys = monkeys.len();
    let mut monkey_item_specs: Vec<usize> = vec![0; num_of_monkeys];
    let mut monkey_items: Vec<Vec<usize>> = Vec::new();
    populate_monkey_items(&monkeys, &mut monkey_items);
    for i in 0..10000 {
        iterate_round(&monkeys, &mut monkey_items, &mut monkey_item_specs, false);
    }
    monkey_item_specs.sort();
    monkey_item_specs.reverse();
    answer = monkey_item_specs[0] * monkey_item_specs[1];
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
