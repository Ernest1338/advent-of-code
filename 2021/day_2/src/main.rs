use std::fs::read_to_string;

fn part1(input_file: String) {
    let answer: usize;
    let mut steps_vec_str: Vec<&str> = input_file.split("\n").collect();
    if steps_vec_str.last().unwrap().to_owned() == "" { steps_vec_str.pop(); } // remove the last element if it's empty
    let steps_vec: Vec<Vec<&str>> = steps_vec_str.iter().map(|step| step.split(" ").collect()).collect();

    let mut position: usize = 0;
    let mut depth: usize = 0;

    for step in steps_vec.iter() {
        if step[0] == "forward" {
            position += step[1].parse::<usize>().unwrap();
        }
        else if step[0] == "down" {
            depth += step[1].parse::<usize>().unwrap();
        }
        else if step[0] == "up" {
            depth -= step[1].parse::<usize>().unwrap();
        }
    }

    answer = position * depth;

    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let answer: usize;
    let mut steps_vec_str: Vec<&str> = input_file.split("\n").collect();
    if steps_vec_str.last().unwrap().to_owned() == "" { steps_vec_str.pop(); } // remove the last element if it's empty
    let steps_vec: Vec<Vec<&str>> = steps_vec_str.iter().map(|step| step.split(" ").collect()).collect();

    let mut position: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;

    let mut value: usize;
    for step in steps_vec.iter() {
        value = step[1].parse::<usize>().unwrap();
        if step[0] == "forward" {
            position += value;
            depth += aim * value;
        }
        else if step[0] == "down" {
            aim += value;
        }
        else if step[0] == "up" {
            aim -= value;
        }
    }

    answer = position * depth;

    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2(input_file); }
}
