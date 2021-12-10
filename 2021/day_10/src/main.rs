#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;
use std::collections::HashMap;

fn part1(input_file: String) {
    let mut answer: usize = 0;
    let mut lines: Vec<&str> = input_file.split("\n").collect();
    if lines.last().unwrap().is_empty() { lines.pop(); }
    let opening_brackets: Vec<char> = vec!['(', '[', '{', '<'];
    let mut closing_brackets_lookup: HashMap<char, char> = HashMap::new();
    closing_brackets_lookup.insert(')', '(');
    closing_brackets_lookup.insert(']', '[');
    closing_brackets_lookup.insert('}', '{');
    closing_brackets_lookup.insert('>', '<');
    let mut points_lookup: HashMap<char, usize> = HashMap::new();
    points_lookup.insert(')', 3);
    points_lookup.insert(']', 57);
    points_lookup.insert('}', 1197);
    points_lookup.insert('>', 25137);
    let mut work_vec: Vec<char> = vec![];
    for line in &lines {
        for chr in line.chars() {
            if opening_brackets.contains(&chr) {
                work_vec.push(chr);
            }
            else {
                if work_vec.last().unwrap() == &closing_brackets_lookup[&chr] {
                    work_vec.pop();
                }
                else {
                    answer += points_lookup[&chr];
                    break;
                }
            }
        }
        work_vec.clear();
    }
    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let answer: usize;
    let mut lines: Vec<&str> = input_file.split("\n").collect();
    let mut incomplete: Vec<&str> = vec![];
    if lines.last().unwrap().is_empty() { lines.pop(); }
    let opening_brackets: Vec<char> = vec!['(', '[', '{', '<'];
    let mut closing_brackets_lookup: HashMap<char, char> = HashMap::new();
    closing_brackets_lookup.insert(')', '(');
    closing_brackets_lookup.insert(']', '[');
    closing_brackets_lookup.insert('}', '{');
    closing_brackets_lookup.insert('>', '<');
    let mut opening_brackets_lookup: HashMap<char, char> = HashMap::new();
    opening_brackets_lookup.insert('(', ')');
    opening_brackets_lookup.insert('[', ']');
    opening_brackets_lookup.insert('{', '}');
    opening_brackets_lookup.insert('<', '>');
    let mut points_lookup: HashMap<char, usize> = HashMap::new();
    points_lookup.insert(')', 1);
    points_lookup.insert(']', 2);
    points_lookup.insert('}', 3);
    points_lookup.insert('>', 4);
    let mut work_vec: Vec<char> = vec![];
    for line in &lines {
        let mut corrupted: bool = false;
        for chr in line.chars() {
            if opening_brackets.contains(&chr) {
                work_vec.push(chr);
            }
            else {
                if work_vec.last().unwrap() == &closing_brackets_lookup[&chr] {
                    work_vec.pop();
                }
                else {
                    corrupted = true;
                }
            }
        }
        if !corrupted {
            incomplete.push(line);
        }
        work_vec.clear();
    }
    let mut complete_vec: Vec<char> = vec![];
    let mut scores_vec: Vec<usize> = vec![];
    for line in &incomplete {
        for chr in line.chars() {
            if opening_brackets.contains(&chr) {
                work_vec.push(chr);
            }
            else {
                if work_vec.last().unwrap() == &closing_brackets_lookup[&chr] {
                    work_vec.pop();
                }
            }
        }
        for chr in work_vec.iter().rev() {
            complete_vec.push(opening_brackets_lookup[chr]);
        }
        let mut score: usize = 0;
        for chr in &complete_vec {
            score *= 5;
            score += points_lookup[chr];
        }
        scores_vec.push(score);
        work_vec.clear();
        complete_vec.clear();
    }
    scores_vec.sort();
    answer = scores_vec[(scores_vec.len()-1)/2];
    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2(input_file); }
}
