#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn print_octopuses(octopuses: &Vec<Vec<i32>>) {
    for line in octopuses {
        for octopus in line {
            print!("{}", octopus);
        }
        println!();
    }
    println!();
}

fn step(octopuses: Vec<Vec<i32>>) -> (Vec<Vec<i32>>, usize) {
    let mut out_octopuses: Vec<Vec<i32>> = octopuses.to_owned();
    for line in 0..out_octopuses.len() { // incrementing every value in the vector
        for octopus in 0..out_octopuses[line].len() {
            out_octopuses[line][octopus] = out_octopuses[line][octopus] + 1;
        }
    }
    let mut coords_of_flashes: Vec<Vec<usize>> = vec![];
    loop {
        let mut run: bool = false;
        for line in &out_octopuses {
            if line.iter().any(|v| v > &9) {
                run = true;
            }
        }
        if run {
            for line in 0..out_octopuses.len() {
                for octopus in 0..out_octopuses[line].len() {
                    if out_octopuses[line][octopus] > 9 {
                        coords_of_flashes.push(vec![line, octopus]);
                        out_octopuses[line][octopus] = 0;
                        if line as i32 - 1 >= 0 { // top
                            out_octopuses[line - 1][octopus] = out_octopuses[line - 1][octopus] + 1; // top
                            if octopus + 1 < out_octopuses[line].len() { out_octopuses[line - 1][octopus + 1] = out_octopuses[line - 1][octopus + 1] + 1; } // top-right
                            if octopus as i32 - 1 >= 0 { out_octopuses[line - 1][octopus - 1] = out_octopuses[line - 1][octopus - 1] + 1; } // top-left
                        }
                        if octopus as i32 - 1 >= 0 { // left
                            out_octopuses[line][octopus - 1] = out_octopuses[line][octopus - 1] + 1; // left
                        }
                        if octopus + 1 < out_octopuses[line].len() { // right
                            out_octopuses[line][octopus + 1] = out_octopuses[line][octopus + 1] + 1; // right
                        }
                        if line + 1 < out_octopuses.len() { // bottom
                            out_octopuses[line + 1][octopus] = out_octopuses[line + 1][octopus] + 1; // bottom
                            if octopus + 1 < out_octopuses[line].len() { out_octopuses[line + 1][octopus + 1] = out_octopuses[line + 1][octopus + 1] + 1; } // bottom-right
                            if octopus as i32 - 1 >= 0 { out_octopuses[line + 1][octopus - 1] = out_octopuses[line + 1][octopus - 1] + 1; } // bottom-left
                        }
                    }
                }
            }
            //dbg!(&coords_of_flashes);
        }
        else {
            break;
        }
    }
    for coord in &coords_of_flashes { out_octopuses[coord[0]][coord[1]] = 0; }
    return (out_octopuses, coords_of_flashes.len());
}

fn part1(input_file: String) {
    // while octopuses.contains(values>9)
    // func which does the incrementing
    let mut answer: usize = 0;
    let mut octopuses: Vec<Vec<i32>> = input_file.split("\n").map(|c| c.chars().map(|d| d.to_string().parse::<i32>().unwrap()).collect()).collect::<Vec<Vec<i32>>>();
    if octopuses.last().unwrap().is_empty() { octopuses.pop(); }
    for _ in 0..100 {
        answer += step(octopuses.to_owned()).1;
        //print_octopuses(&octopuses);
        octopuses = step(octopuses).0;
    }
    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let mut answer: usize = 0;
    let mut octopuses: Vec<Vec<i32>> = input_file.split("\n").map(|c| c.chars().map(|d| d.to_string().parse::<i32>().unwrap()).collect()).collect::<Vec<Vec<i32>>>();
    if octopuses.last().unwrap().is_empty() { octopuses.pop(); }
    for i in 0..1000 {
        octopuses = step(octopuses).0;
        //print_octopuses(&octopuses);
        if octopuses.iter().all(|o| o == &octopuses[0]) { answer = i + 1; break; }
    }
    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2(input_file); }
}
