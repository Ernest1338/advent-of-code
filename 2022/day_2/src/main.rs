#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn part1(input_file: String) -> usize {
    let mut score: usize = 0;
    let input_file = input_file.trim();
    for line in input_file.split('\n').collect::<Vec<&str>>() {
        let oponnent = line.chars().next().unwrap();
        let response = line.chars().nth(2).unwrap();
        // dbg!(&oponnent, &response);
        match oponnent {
            'A' => {
                // rock
                match response {
                    'X' => {
                        // rock
                        score += 1;
                        // draw
                        score += 3;
                    }
                    'Y' => {
                        // paper
                        score += 2;
                        // win
                        score += 6;
                    }
                    'Z' => {
                        // scissors
                        score += 3;
                        // loose
                    }
                    _ => {}
                }
            }
            'B' => {
                // paper
                match response {
                    'X' => {
                        // rock
                        score += 1;
                        // loose
                    }
                    'Y' => {
                        // paper
                        score += 2;
                        // draw
                        score += 3;
                    }
                    'Z' => {
                        // scissors
                        score += 3;
                        // win
                        score += 6;
                    }
                    _ => {}
                }
            }
            'C' => {
                // scissors
                match response {
                    'X' => {
                        // rock
                        score += 1;
                        // win
                        score += 6;
                    }
                    'Y' => {
                        // paper
                        score += 2;
                        // loose
                    }
                    'Z' => {
                        // scissors
                        score += 3;
                        // draw
                        score += 3;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    score
}

fn part2(input_file: String) -> usize {
    let mut score: usize = 0;
    let input_file = input_file.trim();
    for line in input_file.split('\n').collect::<Vec<&str>>() {
        let oponnent = line.chars().next().unwrap();
        let response = line.chars().nth(2).unwrap();
        // dbg!(&oponnent, &response);
        match oponnent {
            'A' => {
                // rock
                match response {
                    'X' => {
                        // need to loose
                        // use: scissors
                        score += 3;
                        // loose
                    }
                    'Y' => {
                        // need to draw
                        // use: rock
                        score += 1;
                        // draw
                        score += 3;
                    }
                    'Z' => {
                        // need to win
                        // use: paper
                        score += 2;
                        // win
                        score += 6;
                    }
                    _ => {}
                }
            }
            'B' => {
                // paper
                match response {
                    'X' => {
                        // need to loose
                        // use: rock
                        score += 1;
                        // loose
                    }
                    'Y' => {
                        // need to draw
                        // use: paper
                        score += 2;
                        // draw
                        score += 3;
                    }
                    'Z' => {
                        // need to win
                        // use: scissors
                        score += 3;
                        // win
                        score += 6;
                    }
                    _ => {}
                }
            }
            'C' => {
                // scissors
                match response {
                    'X' => {
                        // need to loose
                        // use: paper
                        score += 2;
                        // loose
                    }
                    'Y' => {
                        // need to draw
                        // use: scissors
                        score += 3;
                        // draw
                        score += 3;
                    }
                    'Z' => {
                        // need to win
                        // use: rock
                        score += 1;
                        // win
                        score += 6;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    score
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
