#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

// up    => x - 1
// down  => x + 1
// left  => y - 1
// right => y + 1

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct ScanResult {
    up: char,
    down: char,
    left: char,
    right: char,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<char>>,
    pos: Position,
    orig_pos: Position,
    dir: Direction,
    moves: usize,
    visited: Vec<Vec<bool>>,
    loop_detect: Vec<bool>,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

fn find_guard(map: &[Vec<char>]) -> Position {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '^' {
                return Position { x: y, y: x };
            }
        }
    }
    Position { x: 0, y: 0 }
}

impl Map {
    fn from_str(input: &str) -> Self {
        let map: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let cur_pos = find_guard(&map);
        let mut visited = vec![vec![false; map.len()]; map[0].len()];
        visited[cur_pos.x][cur_pos.y] = true;
        Self {
            map,
            pos: cur_pos.clone(),
            orig_pos: cur_pos,
            dir: Direction::Up,
            moves: 0,
            visited,
            loop_detect: vec![false; 1000],
        }
    }

    fn scan(&self) -> Option<ScanResult> {
        let (mut up, mut down, mut left, mut right) = ('.', '.', '.', '.');

        // up

        // down

        // left

        // rigt

        Some(ScanResult {
            up,
            down,
            left,
            right,
        })
    }

    fn make_move(&mut self) -> Result<(), &str> {
        match self.dir {
            Direction::Up => {
                if self.pos.x == 0 {
                    return Err("");
                }
                let next = self.map[self.pos.x - 1][self.pos.y];
                if next == '#' {
                    self.dir = self.dir.next();
                } else {
                    self.pos = Position {
                        x: self.pos.x - 1,
                        y: self.pos.y,
                    };
                    self.moves += 1;
                    if self.visited[self.pos.x][self.pos.y] {
                        if self.loop_detect.iter().all(|&x| x) {
                            return Err("loop");
                        }
                        if self.loop_detect.len() >= 1000 {
                            self.loop_detect.remove(0);
                        }
                        self.loop_detect.push(true);
                    }
                    self.visited[self.pos.x][self.pos.y] = true;
                }
            }
            Direction::Down => {
                if self.pos.x == self.map.len() - 1 {
                    return Err("");
                }
                let next = self.map[self.pos.x + 1][self.pos.y];
                if next == '#' {
                    self.dir = self.dir.next();
                } else {
                    self.pos = Position {
                        x: self.pos.x + 1,
                        y: self.pos.y,
                    };
                    self.moves += 1;
                    if self.visited[self.pos.x][self.pos.y] {
                        if self.loop_detect.iter().all(|&x| x) {
                            return Err("loop");
                        }
                        if self.loop_detect.len() >= 100 {
                            self.loop_detect.remove(0);
                        }
                        self.loop_detect.push(true);
                    }
                    self.visited[self.pos.x][self.pos.y] = true;
                }
            }
            Direction::Left => {
                if self.pos.y == 0 {
                    return Err("");
                }
                let next = self.map[self.pos.x][self.pos.y - 1];
                if next == '#' {
                    self.dir = self.dir.next();
                } else {
                    self.pos = Position {
                        x: self.pos.x,
                        y: self.pos.y - 1,
                    };
                    self.moves += 1;
                    if self.visited[self.pos.x][self.pos.y] {
                        if self.loop_detect.iter().all(|&x| x) {
                            return Err("loop");
                        }
                        if self.loop_detect.len() >= 100 {
                            self.loop_detect.remove(0);
                        }
                        self.loop_detect.push(true);
                    }
                    self.visited[self.pos.x][self.pos.y] = true;
                }
            }
            Direction::Right => {
                if self.pos.y == self.map.len() - 1 {
                    return Err("");
                }
                let next = self.map[self.pos.x][self.pos.y + 1];
                if next == '#' {
                    self.dir = self.dir.next();
                } else {
                    self.pos = Position {
                        x: self.pos.x,
                        y: self.pos.y + 1,
                    };
                    self.moves += 1;
                    if self.visited[self.pos.x][self.pos.y] {
                        if self.loop_detect.iter().all(|&x| x) {
                            return Err("loop");
                        }
                        if self.loop_detect.len() >= 100 {
                            self.loop_detect.remove(0);
                        }
                        self.loop_detect.push(true);
                    }
                    self.visited[self.pos.x][self.pos.y] = true;
                }
            }
        };
        Ok(())
    }

    fn debug(&self) {
        for x in 0..self.map.len() {
            for y in 0..self.map.len() {
                if self.pos.x == x && self.pos.y == y {
                    print!("!");
                } else {
                    print!("{}", self.map[x][y]);
                }
            }
            println!();
        }
    }
}

fn part1(input_file: &str) -> usize {
    let mut map = Map::from_str(input_file);
    while map.make_move().is_ok() {
        // map.debug();
    }
    map.visited
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&v| v)
        .count()
}

fn part2(input_file: &str) -> usize {
    let mut answer = 0;
    let mut map = Map::from_str(input_file);
    for x in 0..map.map.len() {
        for y in 0..map.map.len() {
            map.map[x][y] = '#';
            let mut res = map.make_move();
            while res.is_ok() {
                res = map.make_move();
                // map.debug();
            }
            if let Err(e) = res {
                if e == "loop" {
                    answer += 1;
                }
            }
            map = Map::from_str(input_file);
        }
    }
    answer
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
