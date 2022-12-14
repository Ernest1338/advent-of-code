#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Cords {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct Solver {
    cur_pos: Cords,
    end_pos: Cords,
    map: Vec<Vec<char>>,
    solution: Vec<Cords>,
}

impl Cords {
    fn new() -> Cords {
        Cords { x: 0, y: 0 }
    }

    fn from(x: usize, y: usize) -> Cords {
        Cords { x, y }
    }
}

fn get_dist(pos1: Cords, pos2: Cords) -> usize {
    pos1.x.abs_diff(pos2.x) + pos1.y.abs_diff(pos2.y)
}

const ALPHABET: [char; 27] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', '.',
];

impl Solver {
    fn from(cur_pos: Cords, end_pos: Cords, map: Vec<Vec<char>>) -> Solver {
        Solver {
            cur_pos,
            end_pos,
            map,
            solution: Vec::new(),
        }
    }

    fn find_new_move(&self) -> Cords {
        // TODO: this function is atrocious
        let dist_before_movement = get_dist(self.cur_pos, self.end_pos);
        // let surrounding = self.get_surround();
        if get_dist(
            Cords::from(self.cur_pos.x + 1, self.cur_pos.y),
            self.end_pos,
        ) < dist_before_movement
        {
            let cur_pos_height = ALPHABET
                .iter()
                .position(|&l| l == self.map[self.cur_pos.y][self.cur_pos.x])
                .unwrap();
            let next_move_height = ALPHABET
                .iter()
                .position(|&l| l == self.map[self.cur_pos.y][self.cur_pos.x + 1])
                .unwrap();
            if next_move_height == cur_pos_height || next_move_height == cur_pos_height + 1 {
                return Cords::from(self.cur_pos.x + 1, self.cur_pos.y);
            }
        }
        if get_dist(
            Cords::from(self.cur_pos.x - 1, self.cur_pos.y),
            self.end_pos,
        ) < dist_before_movement
        {
            let cur_pos_height = ALPHABET
                .iter()
                .position(|&l| l == self.map[self.cur_pos.y][self.cur_pos.x])
                .unwrap();
            let next_move_height = ALPHABET
                .iter()
                .position(|&l| l == self.map[self.cur_pos.y][self.cur_pos.x - 1])
                .unwrap();
            if next_move_height == cur_pos_height || next_move_height == cur_pos_height + 1 {
                return Cords::from(self.cur_pos.x - 1, self.cur_pos.y);
            }
        }
        if get_dist(
            Cords::from(self.cur_pos.x, self.cur_pos.y + 1),
            self.end_pos,
        ) < dist_before_movement
        {
            let cur_pos_height = ALPHABET
                .iter()
                .position(|&l| l == self.map[self.cur_pos.y][self.cur_pos.x])
                .unwrap();
            let next_move_height = ALPHABET
                .iter()
                .position(|&l| l == self.map[self.cur_pos.y + 1][self.cur_pos.x])
                .unwrap();
            if next_move_height == cur_pos_height || next_move_height == cur_pos_height + 1 {
                return Cords::from(self.cur_pos.x, self.cur_pos.y + 1);
            }
        }
        if get_dist(
            Cords::from(self.cur_pos.x, self.cur_pos.y - 1),
            self.end_pos,
        ) < dist_before_movement
        {
            let cur_pos_height = ALPHABET
                .iter()
                .position(|&l| l == self.map[self.cur_pos.y][self.cur_pos.x])
                .unwrap();
            let next_move_height = ALPHABET
                .iter()
                .position(|&l| l == self.map[self.cur_pos.y - 1][self.cur_pos.x])
                .unwrap();
            if next_move_height == cur_pos_height || next_move_height == cur_pos_height + 1 {
                return Cords::from(self.cur_pos.x, self.cur_pos.y - 1);
            }
        }
        self.cur_pos
    }

    fn get_surround(&self) -> Vec<Vec<char>> {
        let mut surround: Vec<Vec<char>> = Vec::new();
        for x in self.cur_pos.x as i32 - 1..=self.cur_pos.x as i32 + 1 {
            surround.push(Vec::new());
            for y in self.cur_pos.y as i32 - 1..=self.cur_pos.y as i32 + 1 {
                if let Some(a) = self.map.get(y as usize) {
                    if let Some(b) = a.get(x as usize) {
                        surround.last_mut().unwrap().push(*b);
                    } else {
                        surround.last_mut().unwrap().push('.');
                    }
                } else {
                    surround.last_mut().unwrap().push('.');
                }
            }
        }
        surround
    }

    fn get_dist_to_end(&self) -> usize {
        self.end_pos.x.abs_diff(self.cur_pos.x) + self.end_pos.y.abs_diff(self.cur_pos.y)
    }

    fn make_move(&mut self) {
        self.cur_pos = self.find_new_move();
        self.solution.push(self.cur_pos);
        // std::process::Command::new("sleep").args(["0.05"]).output().unwrap();
        // self.visualize();
    }

    fn solve(&mut self) {
        while self.cur_pos != self.end_pos {
            self.make_move();
        }
    }

    fn visualize(&self) {
        for i in 0..self.map[0].len() {
            print!("_");
        }
        println!();
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if self.cur_pos.x == x && self.cur_pos.y == y {
                    print!("S");
                } else if self.end_pos.x == x && self.end_pos.y == y {
                    print!("E");
                } else {
                    print!("{}", self.map[y][x]);
                }
            }
            println!();
        }
        for i in 0..self.map[0].len() {
            print!("‾");
        }
        println!();
    }
}

fn input_to_map(input_file: &str) -> Vec<Vec<char>> {
    input_file
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn part1(input_file: String) -> usize {
    let mut answer: usize = 0;

    let sample_start = Cords::from(0, 0);
    let sample_end = Cords::from(5, 2);
    let input_start = Cords::from(0, 20);
    let input_end = Cords::from(55, 20);

    let map = input_to_map(&input_file);
    let mut solver = Solver::from(sample_start, sample_end, map);
    // let mut solver = Solver::from(input_start, input_end, map);
    solver.visualize();
    solver.solve();
    let solution = solver.solution;
    dbg!(&solution);

    answer
}

fn part2(input_file: String) -> usize {
    let mut answer: usize = 0;
    answer
}

fn main() {
    // TODO: djikstra...
    let part = 1;

    let input_file = load_file("sample.txt");

    if part == 1 {
        print_answer(part1(input_file));
    } else if part == 2 {
        print_answer(part2(input_file));
    }
}
