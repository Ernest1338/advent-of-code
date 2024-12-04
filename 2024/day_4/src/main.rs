// #![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn debug_map(map: &Vec<Vec<char>>, points: &Vec<(usize, usize)>) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if points.contains(&(x, y)) {
                print!(".");
            } else {
                print!("{}", map[x][y]);
            }
        }
        println!();
    }
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let map: Vec<Vec<char>> = input_file
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let c = map[x][y];
            if c != 'X' {
                continue;
            }

            if x + 3 < map.len() // right
                && map[x + 1][y] == 'M'
                && map[x + 2][y] == 'A'
                && map[x + 3][y] == 'S'
            {
                // println!("right: {x} {y}");
                // debug_map(&map, &vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]);
                answer += 1;
            }

            if x >= 3 // left
                && map[x - 1][y] == 'M'
                && map[x - 2][y] == 'A'
                && map[x - 3][y] == 'S'
            {
                // println!("left: {x} {y}");
                // debug_map(&map, &vec![(x, y), (x - 1, y), (x - 2, y), (x - 3, y)]);
                answer += 1;
            }

            if y + 3 < map.len() // up
                && map[x][y + 1] == 'M'
                && map[x][y + 2] == 'A'
                && map[x][y + 3] == 'S'
            {
                // println!("up: {x} {y}");
                // debug_map(&map, &vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]);
                answer += 1;
            }

            if y >= 3 // down
                && map[x][y - 1] == 'M'
                && map[x][y - 2] == 'A'
                && map[x][y - 3] == 'S'
            {
                // println!("up: {x} {y}");
                // debug_map(&map, &vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]);
                answer += 1;
            }

            if x + 3 < map.len() && y + 3 < map.len() // diagonal 1
                && map[x + 1][y + 1] == 'M'
                && map[x + 2][y + 2] == 'A'
                && map[x + 3][y + 3] == 'S'
            {
                // println!("d1: {x} {y}");
                // debug_map(&map, &vec![(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)]);
                answer += 1;
            }

            if x >= 3 && y + 3 < map.len() // diagonal 2
                && map[x - 1][y + 1] == 'M'
                && map[x - 2][y + 2] == 'A'
                && map[x - 3][y + 3] == 'S'
            {
                // println!("d2: {x} {y}");
                // debug_map(&map, &vec![(x, y), (x - 1, y + 1), (x - 2, y + 2), (x - 3, y + 3)]);
                answer += 1;
            }

            if y >= 3 && x + 3 < map.len() // diagonal 3
                && map[x + 1][y - 1] == 'M'
                && map[x + 2][y - 2] == 'A'
                && map[x + 3][y - 3] == 'S'
            {
                // println!("d3: {x} {y}");
                // debug_map(&map, &vec![(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)]);
                answer += 1;
            }

            if y >= 3 && x >= 3 // diagonal 4
                && map[x - 1][y - 1] == 'M'
                && map[x - 2][y - 2] == 'A'
                && map[x - 3][y - 3] == 'S'
            {
                // println!("d4: {x} {y}");
                // debug_map(&map, &vec![(x, y), (x - 1, y - 1), (x - 2, y - 2), (x - 3, y - 3)]);
                answer += 1;
            }
        }
    }
    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let map: Vec<Vec<char>> = input_file
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    for x in 1..map.len() - 1 {
        for y in 1..map[0].len() - 1 {
            let c = map[x][y];
            if c != 'A' {
                continue;
            }

            // case 1
            if map[x - 1][y + 1] == 'M'
                && map[x + 1][y + 1] == 'M'
                && map[x - 1][y - 1] == 'S'
                && map[x + 1][y - 1] == 'S'
            {
                // println!("case1: {x} {y}");
                // debug_map(&map, &vec![
                //     (x, y),
                //     (x - 1, y + 1),
                //     (x + 1, y + 1),
                //     (x - 1, y - 1),
                //     (x + 1, y - 1),
                // ]);
                answer += 1;
            }

            // case 2
            if map[x - 1][y + 1] == 'S'
                && map[x + 1][y + 1] == 'M'
                && map[x - 1][y - 1] == 'S'
                && map[x + 1][y - 1] == 'M'
            {
                // println!("case2: {x} {y}");
                // debug_map(&map, &vec![
                //     (x, y),
                //     (x - 1, y + 1),
                //     (x + 1, y + 1),
                //     (x - 1, y - 1),
                //     (x + 1, y - 1),
                // ]);
                answer += 1;
            }

            // case 3
            if map[x - 1][y + 1] == 'S'
                && map[x + 1][y + 1] == 'S'
                && map[x - 1][y - 1] == 'M'
                && map[x + 1][y - 1] == 'M'
            {
                // println!("case3: {x} {y}");
                // debug_map(&map, &vec![
                //     (x, y),
                //     (x - 1, y + 1),
                //     (x + 1, y + 1),
                //     (x - 1, y - 1),
                //     (x + 1, y - 1),
                // ]);
                answer += 1;
            }

            // case 4
            if map[x - 1][y + 1] == 'M'
                && map[x + 1][y + 1] == 'S'
                && map[x - 1][y - 1] == 'M'
                && map[x + 1][y - 1] == 'S'
            {
                // println!("case4: {x} {y}");
                // debug_map(&map, &vec![
                //     (x, y),
                //     (x - 1, y + 1),
                //     (x + 1, y + 1),
                //     (x - 1, y - 1),
                //     (x + 1, y - 1),
                // ]);
                answer += 1;
            }
        }
    }
    answer
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
