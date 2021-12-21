#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn check_board(board: Vec<usize>, numbers: Vec<usize>) -> bool {
    let mut result: bool = false;
    let mut hit_vec: Vec<&str> = vec![];

    for number_index in 0..board.len() {
        if numbers.contains(&board[number_index]) {
            hit_vec.push("hit");
        } else {
            hit_vec.push("miss");
        }
    }

    if (hit_vec[0] == "hit"
        && hit_vec[1] == "hit"
        && hit_vec[2] == "hit"
        && hit_vec[3] == "hit"
        && hit_vec[4] == "hit")
        || (hit_vec[5] == "hit"
            && hit_vec[6] == "hit"
            && hit_vec[7] == "hit"
            && hit_vec[8] == "hit"
            && hit_vec[9] == "hit")
        || (hit_vec[10] == "hit"
            && hit_vec[11] == "hit"
            && hit_vec[12] == "hit"
            && hit_vec[13] == "hit"
            && hit_vec[14] == "hit")
        || (hit_vec[15] == "hit"
            && hit_vec[16] == "hit"
            && hit_vec[17] == "hit"
            && hit_vec[18] == "hit"
            && hit_vec[19] == "hit")
        || (hit_vec[20] == "hit"
            && hit_vec[21] == "hit"
            && hit_vec[22] == "hit"
            && hit_vec[23] == "hit"
            && hit_vec[24] == "hit")
        || (hit_vec[0] == "hit"
            && hit_vec[5] == "hit"
            && hit_vec[10] == "hit"
            && hit_vec[15] == "hit"
            && hit_vec[20] == "hit")
        || (hit_vec[1] == "hit"
            && hit_vec[6] == "hit"
            && hit_vec[11] == "hit"
            && hit_vec[16] == "hit"
            && hit_vec[21] == "hit")
        || (hit_vec[2] == "hit"
            && hit_vec[7] == "hit"
            && hit_vec[12] == "hit"
            && hit_vec[17] == "hit"
            && hit_vec[22] == "hit")
        || (hit_vec[3] == "hit"
            && hit_vec[8] == "hit"
            && hit_vec[13] == "hit"
            && hit_vec[18] == "hit"
            && hit_vec[23] == "hit")
        || (hit_vec[4] == "hit"
            && hit_vec[9] == "hit"
            && hit_vec[14] == "hit"
            && hit_vec[19] == "hit"
            && hit_vec[24] == "hit")
    {
        result = true;
    }

    return result;
}

fn part1(input_file: String) {
    // function which checks if the provided board (vec) won with the provided numbers (vec)
    // vec of vec<size> (boards)
    // vec of usize (numbers)
    let answer: usize;
    let input: Vec<&str> = input_file.split("\n\n").collect();
    let numbers: Vec<usize> = input[0]
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut boards: Vec<Vec<usize>> = vec![];
    let mut tmp_board: Vec<usize>;
    for board_index in 1..input.len() {
        tmp_board = input[board_index]
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        boards.push(tmp_board);
    }
    let mut run: bool = true;
    let mut last_number_index: usize = 0;
    let mut winning_board: Vec<usize> = vec![];
    for number_index in 0..numbers.len() {
        for board in &boards {
            if check_board(board.to_owned(), numbers[0..number_index].to_owned()) {
                winning_board = board.to_owned();
                run = false;
                break;
            }
        }
        if run == false {
            last_number_index = number_index - 1;
            break;
        }
    }
    let mut winning_numbers: Vec<usize> = vec![];
    for number in 0..last_number_index + 1 {
        winning_numbers.push(numbers[number]);
    }
    let mut sum_of_unmarked: usize = 0;
    for number in &winning_board {
        if !winning_numbers.contains(&number) {
            sum_of_unmarked += number.to_owned();
        }
    }

    answer = sum_of_unmarked * numbers[last_number_index];

    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let answer: usize;
    let input: Vec<&str> = input_file.split("\n\n").collect();
    let numbers: Vec<usize> = input[0]
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut boards: Vec<Vec<usize>> = vec![];
    let mut tmp_board: Vec<usize>;
    for board_index in 1..input.len() {
        tmp_board = input[board_index]
            .split_whitespace()
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        boards.push(tmp_board);
    }
    let mut winning_boards: Vec<Vec<usize>> = vec![];
    let mut run: bool = true;
    let mut last_number_index: usize = 0;
    for number_index in 0..numbers.len() {
        for board in &boards {
            if check_board(board.to_owned(), numbers[0..number_index].to_owned()) {
                if !winning_boards.contains(&board) {
                    winning_boards.push(board.to_owned());
                }
                if winning_boards.len() == boards.len() {
                    last_number_index = number_index;
                    run = false;
                    break;
                }
            }
        }
        if run == false {
            break;
        }
    }
    let mut winning_numbers: Vec<usize> = vec![];
    for number in 0..last_number_index {
        winning_numbers.push(numbers[number]);
    }
    let mut sum_of_unmarked: usize = 0;
    for number in winning_boards.last().unwrap() {
        if !winning_numbers.contains(&number) {
            sum_of_unmarked += number.to_owned();
        }
    }

    answer = sum_of_unmarked * numbers[last_number_index - 1];

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
