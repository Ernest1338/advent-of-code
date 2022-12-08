#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn get_sums(input_file: String) -> Vec<usize> {
    let mut dirs: Vec<Vec<String>> = Vec::new();
    let mut depth = 0;
    let mut depths: Vec<usize> = Vec::new();
    let mut lines: Vec<&str> = input_file.lines().collect();
    for line in &lines {
        let line_split: Vec<&str> = line.split(' ').collect();
        if line_split[1] == "cd" {
            if line_split[2] == ".." {
                depth -= 1;
            } else {
                depth += 1;
            }
        }
        depths.push(depth);
    }
    for i in (0..*depths.last().unwrap()).rev() {
        depths.push(i);
        lines.push("$ cd ..");
    }
    let mut last_depth = 0;
    let mut indexes: Vec<(usize, usize)> = Vec::new();
    let mut index_vec: Vec<usize> = Vec::new();
    for (i, d) in depths.iter().enumerate() {
        match last_depth.cmp(d) {
            std::cmp::Ordering::Less => {
                index_vec.push(i);
            }
            std::cmp::Ordering::Greater => {
                indexes.push((index_vec.pop().unwrap(), i));
            }
            _ => {}
        }
        last_depth = *d;
    }
    for index in &indexes {
        let mut tmp: Vec<String> = Vec::new();
        for el in lines.iter().take(index.1 + 1).skip(index.0) {
            tmp.push(el.to_string());
        }
        dirs.push(tmp);
    }
    let mut sums: Vec<usize> = Vec::new();
    for dir in dirs {
        let mut sum = 0;
        for l in dir {
            let l_split: Vec<&str> = l.split(' ').collect();
            if let Ok(ok) = l_split[0].parse::<usize>() {
                sum += ok;
            }
        }
        sums.push(sum);
    }
    sums
}

fn part1(input_file: String) -> usize {
    let mut answer: usize = 0;
    let mut sums = get_sums(input_file);
    sums.pop();
    for s in sums {
        if s < 100000 {
            answer += s;
        }
    }
    answer
}

fn part2(input_file: String) -> usize {
    let mut answer: usize;
    let mut sums = get_sums(input_file);
    let used = sums.pop().unwrap();
    let unused = 70_000_000 - used;
    let needed = 30_000_000;
    sums.sort();
    sums.reverse();
    let mut big_enoughs: Vec<usize> = Vec::new();
    for s in sums {
        if unused + s >= 30_000_000 {
            big_enoughs.push(s);
        }
    }
    answer = *big_enoughs.last().unwrap();
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
