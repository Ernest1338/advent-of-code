#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn into_stacks(stacks_str: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for line in stacks_str.lines() {
        let line_chars: Vec<char> = line.chars().collect();
        for i in (1..line.len()).step_by(4) {
            if line_chars[i] == '1' {
                break;
            }
            if stacks.len() < &i / 4 + 1 {
                stacks.push(vec![]);
            }
            if line_chars[i] != ' ' {
                stacks[&i / 4].push(line_chars[i]);
            }
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }
    stacks
}

struct Instruction {
    how_many: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Instruction {
        let instruction_split = s.split(' ').collect::<Vec<&str>>();
        let how_many: usize = instruction_split[1].parse().unwrap();
        let from: usize = instruction_split[3].parse().unwrap();
        let to: usize = instruction_split[5].parse().unwrap();
        Instruction { how_many, from, to }
    }
}

fn print_answer(stacks: &[Vec<char>]) {
    let answer: String = stacks.iter().map(|s| s.last().unwrap()).collect();
    println!("Answer: {answer}");
}

fn part1(input_file: String) {
    let stacks_str = input_file.split("\n\n").collect::<Vec<&str>>()[0];
    let instructions = input_file.split("\n\n").collect::<Vec<&str>>()[1];
    let mut stacks = into_stacks(stacks_str);
    for instruction in instructions.lines() {
        let instr: Instruction = instruction.into();
        for _ in 0..instr.how_many {
            let tmp = stacks[instr.from - 1].pop().unwrap();
            stacks[instr.to - 1].push(tmp);
        }
    }
    print_answer(&stacks);
}

fn part2(input_file: String) {
    let stacks_str = input_file.split("\n\n").collect::<Vec<&str>>()[0];
    let instructions = input_file.split("\n\n").collect::<Vec<&str>>()[1];
    let mut stacks = into_stacks(stacks_str);
    for instruction in instructions.lines() {
        let instr: Instruction = instruction.into();

        let from_stack_len = stacks[instr.from - 1].len();
        let mut tmp: Vec<char> =
            stacks[instr.from - 1][from_stack_len - instr.how_many..from_stack_len].to_vec();
        stacks[instr.to - 1].append(&mut tmp);
        stacks[instr.from - 1].truncate(from_stack_len - instr.how_many);
    }
    print_answer(&stacks);
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
