use std::fs::read_to_string;

fn part1(input_file: String) {
    let mut answer: usize = 0;
    let mut numbers_vec_str: Vec<&str> = input_file.split("\n").collect();
    if numbers_vec_str.last().unwrap().to_owned() == "" { numbers_vec_str.pop(); } // remove the last element if it's empty
    let mut numbers_vec: Vec<usize> = vec![];
    for number in numbers_vec_str.iter() {
        numbers_vec.push(number.parse::<usize>().unwrap());
    }

    let mut current_number: usize = numbers_vec.first().unwrap().to_owned();

    for number_index in 1..numbers_vec.len() {
        if numbers_vec[number_index] > current_number {
            answer += 1;
        }
        current_number = numbers_vec[number_index];
    }

    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let mut answer: usize = 0;
    let mut numbers_vec_str: Vec<&str> = input_file.split("\n").collect();
    if numbers_vec_str.last().unwrap().to_owned() == "" { numbers_vec_str.pop(); } // remove the last element if it's empty
    let mut numbers_vec: Vec<usize> = vec![];
    for number in numbers_vec_str.iter() {
        numbers_vec.push(number.parse::<usize>().unwrap());
    }

    let mut current_result: usize;
    let mut results_vec: Vec<usize> = vec![];

    for number in 0..numbers_vec.len()-2 {
        current_result = numbers_vec[number] + numbers_vec[number + 1] + numbers_vec[number + 2];
        results_vec.push(current_result);
    }

    let mut current_number: usize = results_vec.first().unwrap().to_owned();
    for result_index in 1..results_vec.len() {
        if results_vec[result_index] > current_number {
            answer += 1;
        }
        current_number = results_vec[result_index];
    }

    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2(input_file); }
}
