#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

const SHAPE_SCORE: [(&str, isize); 3] = [("X", 1), ("Y", 2), ("Z", 3)];
const OPPONENT_SCORE: [(&str, isize); 3] = [("A", 1), ("B", 2), ("C", 3)];
const OUTCOME_SCORE: [(&str, isize); 3] = [("X", 0), ("Y", 3), ("Z", 6)];

fn get_score(key: &str, map: &[(&str, isize)]) -> isize {
    map.iter()
        .find(|&(k, _)| k == &key)
        .map(|&(_, v)| v)
        .unwrap()
}

fn did_win(split: &Vec<&str>) -> usize {
    let diff = get_score(split[0], &OPPONENT_SCORE) - get_score(split[1], &SHAPE_SCORE);
    if split == &vec!["C", "X"] {
        return 6;
    }
    match diff {
        0 => 3,
        -1 => 6,
        _ => 0,
    }
}

fn part1(file_lines: &Vec<String>) -> usize {
    let mut score: usize = 0;
    for line in file_lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        score += get_score(split[1], &SHAPE_SCORE) as usize + did_win(&split);
    }
    score
}

fn choice_score(split: &Vec<&str>) -> usize {
    if split[1] == "X" {
        let tmp = get_score(split[0], &OPPONENT_SCORE) as usize + 2;
        return if tmp > 3 { tmp - 3 } else { tmp };
    } else if split[1] == "Y" {
        return get_score(split[0], &OPPONENT_SCORE) as usize;
    }
    let tmp = get_score(split[0], &OPPONENT_SCORE) as usize + 1;
    return if tmp > 3 { tmp - 3 } else { tmp };
}

fn part2(file_lines: &Vec<String>) -> usize {
    let mut score: usize = 0;
    for line in file_lines {
        let split = line.split(" ").collect::<Vec<&str>>();
        score += choice_score(&split) + get_score(split[1], &OUTCOME_SCORE) as usize;
    }
    score
}

fn main() {
    let file_lines = load_file_lines("input.txt");

    println!("[Part 1] Answer: {}", part1(&file_lines));
    println!("[Part 2] Answer: {}", part2(&file_lines));
}
