#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn part1(input_file: &str) -> usize {
    let seeds = input_file
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let maps: Vec<Vec<Vec<usize>>> = input_file
        .split("\n\n")
        .skip(1)
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| {
            v.lines()
                .skip(1)
                .map(|x| {
                    x.split_whitespace()
                        .map(|y| y.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect();
    let mut lowest = usize::MAX;
    for seed in &seeds {
        let mut cur_seed = seed.to_owned();
        for map in &maps {
            for inner_map in map {
                // TODO: better (if value > min < max something like that)
                if (inner_map[1]..inner_map[1] + inner_map[2]).contains(&cur_seed) {
                    cur_seed = cur_seed - inner_map[1] + inner_map[0];
                    break;
                }
            }
        }
        if cur_seed < lowest {
            lowest = cur_seed;
        }
    }
    lowest
}

fn part2(input_file: &str) -> usize {
    let seeds = input_file
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let maps: Vec<Vec<Vec<usize>>> = input_file
        .split("\n\n")
        .skip(1)
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| {
            v.lines()
                .skip(1)
                .map(|x| {
                    x.split_whitespace()
                        .map(|y| y.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>()
        })
        .collect();
    let mut seed_ranges: Vec<(usize, usize)> = Vec::new();
    for s in (0..seeds.len()).step_by(2) {
        seed_ranges.push((seeds[s], seeds[s + 1]));
    }
    let mut lowest = usize::MAX;
    for range in &seed_ranges {
        println!("{range:?}");
        for seed in range.0..range.0 + range.1 {
            let mut cur_seed = seed.to_owned();
            for map in &maps {
                for inner_map in map {
                    // TODO: better (if value > min < max something like that)
                    if (inner_map[1]..inner_map[1] + inner_map[2]).contains(&cur_seed) {
                        cur_seed = cur_seed - inner_map[1] + inner_map[0];
                        break;
                    }
                }
            }
            if cur_seed < lowest {
                lowest = cur_seed;
            }
        }
    }
    lowest
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
