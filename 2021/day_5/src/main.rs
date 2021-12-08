#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn is_strait_line(points: Vec<usize>) -> bool {
    let mut result: bool = false;
    if (points[0] == points[2]) || (points[1] == points[3]) {
        result = true
    }
    return result;
}

fn part1(input_file: String) {
    // func strait_line (bool) - case when x1=x2 or y1=y2
    let mut answer: usize = 0;
    let mut input: Vec<&str> = input_file.split("\n").collect();
    if input.last().unwrap() == &"" { input.pop(); }

    let mut points: Vec<Vec<usize>> = vec![];
    let mut tmp_vec: Vec<usize> = vec![];
    for line in input {
        tmp_vec.clear();
        for split in line.split(" -> ") {
            for number in split.split(",") {
                tmp_vec.push(number.parse::<usize>().unwrap());
            }
        }
        points.push(tmp_vec.to_owned());
    }

    let mut plane: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000]; // 1000 x 1000 plane

    let mut distance: i32;
    let mut horizontal: bool;
    for current_points in points {
        if is_strait_line(current_points.to_owned()) {
            distance = std::cmp::max((current_points[0] as i32 - current_points[2] as i32).abs(), (current_points[1] as i32 - current_points[3] as i32).abs());
            if current_points[0] != current_points[2] {
                horizontal = true;
            }
            else {
                horizontal = false;
            }
            if horizontal { // of index 1 and 3 are the same
                for i in 0..=distance {
                    plane[current_points[1]][std::cmp::min(current_points[0], current_points[2]) + i as usize] += 1;
                }
            }
            else { // of index 0 and 2 are the same
                for i in 0..=distance {
                    plane[std::cmp::min(current_points[1], current_points[3]) + i as usize][current_points[0]] += 1;
                }
            }
        }
    }

    for line in plane {
        for number in line {
            if number >= 2 {
                answer += 1;
            }
        }
    }

    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let mut answer: usize = 0;
    let mut input: Vec<&str> = input_file.split("\n").collect();
    if input.last().unwrap() == &"" { input.pop(); }

    let mut points: Vec<Vec<usize>> = vec![];
    let mut tmp_vec: Vec<usize> = vec![];
    for line in input {
        tmp_vec.clear();
        for split in line.split(" -> ") {
            for number in split.split(",") {
                tmp_vec.push(number.parse::<usize>().unwrap());
            }
        }
        points.push(tmp_vec.to_owned());
    }

    let mut plane: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000]; // 1000 x 1000 plane

    let mut distance: i32;
    let mut horizontal: bool;
    let mut x_neg: i32;
    let mut y_neg: i32;
    for current_points in points {
        if is_strait_line(current_points.to_owned()) {
            distance = std::cmp::max((current_points[0] as i32 - current_points[2] as i32).abs(), (current_points[1] as i32 - current_points[3] as i32).abs());
            if current_points[0] != current_points[2] {
                horizontal = true;
            }
            else {
                horizontal = false;
            }
            if horizontal { // of index 1 and 3 are the same
                for i in 0..=distance {
                    plane[current_points[1]][std::cmp::min(current_points[0], current_points[2]) + i as usize] += 1;
                }
            }
            else { // of index 0 and 2 are the same
                for i in 0..=distance {
                    plane[std::cmp::min(current_points[1], current_points[3]) + i as usize][current_points[0]] += 1;
                }
            }
        }
        else {
            distance = std::cmp::max((current_points[0] as i32 - current_points[2] as i32).abs(), (current_points[1] as i32 - current_points[3] as i32).abs());
            if current_points[0] > current_points[2] {
                y_neg = -1;
            }
            else {
                y_neg = 1;
            }
            if current_points[1] > current_points[3] {
                x_neg = -1;
            }
            else {
                x_neg = 1;
            }
            for i in 0..=distance {
                plane[(current_points[1] as i32 + (i as i32 * x_neg)) as usize][(current_points[0] as i32 + (i as i32 * y_neg)) as usize] += 1;
            }
        }
    }

    for line in plane {
        for number in line {
            if number >= 2 {
                answer += 1;
            }
        }
    }

    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2(input_file); }
}
