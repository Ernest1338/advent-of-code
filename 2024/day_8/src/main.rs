#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

// map[y][x]

struct Map {
    map: Vec<Vec<char>>,
    pos: Position,
    antinodes: Vec<Vec<char>>,
}

struct Position {
    x: usize,
    y: usize,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let map: Vec<Vec<char>> = input
            .lines()
            .rev()
            .map(|line| line.chars().collect())
            .collect();
        let map_len = &map.len();
        Self {
            map,
            pos: Position { x: 0, y: 0 },
            antinodes: vec![vec!['.'; *map_len]; *map_len],
        }
    }

    fn debug(&self) {
        let map_len = self.map.len();
        for y in 0..map_len {
            for x in 0..map_len {
                print!("{}", self.map[map_len - 1 - y][x]);
            }
            println!("");
        }
    }
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let map = Map::from_str(input_file);
    map.debug();
    // dbg!(&map);
    // for line in input_file.lines() {}
    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    // for line in input_file.lines() {}
    answer
}

fn main() {
    let input_file = load_file("example.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
