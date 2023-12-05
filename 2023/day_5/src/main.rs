#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

use std::ops::Range;

#[derive(Debug)]
struct MapRange {
    source_range: Range<usize>,
    destination_range: Range<usize>,
}

#[derive(Debug)]
struct Map {
    map_from: String,
    map_to: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn from(map: &str) -> Self {
        let mut map_lines = map.lines();
        let map_from_to = map_lines.next().unwrap().split_whitespace().next().unwrap();
        let map_from = map_from_to.split("-to-").next().unwrap().to_string();
        let map_to = map_from_to.split("-to-").nth(1).unwrap().to_string();
        let mut ranges: Vec<MapRange> = Vec::new();
        for r in map_lines {
            ranges.push(MapRange::from(r));
        }
        Self {
            map_from,
            map_to,
            ranges,
        }
    }
}

impl MapRange {
    fn from(range: &str) -> Self {
        let split = range
            .split_whitespace()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let destination_range_start = split[0];
        let source_range_start = split[1];
        let amount = split[2];
        Self {
            source_range: source_range_start..source_range_start + amount,
            destination_range: destination_range_start..destination_range_start + amount,
        }
    }
}

fn part1(input_file: &str) -> usize {
    let seeds = input_file
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut maps: Vec<Map> = Vec::new();
    for map in input_file.split("\n\n").skip(1) {
        maps.push(Map::from(map));
    }
    let mut locations: Vec<usize> = Vec::new();
    for seed in seeds {
        let mut cur_value = seed;
        for m in &maps {
            for r in &m.ranges {
                if r.source_range.contains(&cur_value) {
                    let index = r.source_range.clone().position(|x| x == cur_value).unwrap();
                    cur_value = r.destination_range.clone().nth(index).unwrap();
                    break;
                }
            }
        }
        locations.push(cur_value);
    }
    locations
        .into_iter()
        .min_by(|a, b| a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Less))
        .unwrap()
}

fn part2(input_file: &str) -> usize {
    // well well well. that's not gonna work
    let mut answer: usize = 0;
    // for line in input_file.lines() {}
    answer
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
