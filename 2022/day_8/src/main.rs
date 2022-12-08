#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn part1(input_file: String) -> usize {
    let mut answer: usize = 0;
    let size = input_file.lines().collect::<Vec<&str>>()[0].len();
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in input_file.lines() {
        let mut tmp: Vec<i32> = Vec::new();
        for i in line.chars() {
            tmp.push(i.to_string().parse::<i32>().unwrap());
        }
        grid.push(tmp);
    }
    // dbg!(&grid);
    for x in 1..size - 1 {
        for y in 1..size - 1 {
            let val = grid[x][y];
            // println!("{x}:{y} {}", &val);
            // checking column
            let mut highest_left = -1;
            let mut highest_right = -1;
            let left_range = 0..y;
            for c in left_range {
                if &grid[x][c] > &highest_left {
                    highest_left = grid[x][c];
                }
            }
            let right_range = y + 1..size;
            for c in right_range {
                if &grid[x][c] > &highest_right {
                    highest_right = grid[x][c];
                }
            }
            // checking row
            let mut highest_top = -1;
            let mut highest_down = -1;
            let top_range = 0..x;
            for c in top_range {
                if &grid[c][y] > &highest_top {
                    highest_top = grid[c][y];
                }
            }
            let down_range = x + 1..size;
            for c in down_range {
                if &grid[c][y] > &highest_down {
                    highest_down = grid[c][y];
                }
            }
            if highest_left < val || highest_right < val || highest_top < val || highest_down < val
            {
                answer += 1;
            }
        }
    }
    // add to that the ones on the edge
    answer += size * 2;
    answer += (size - 2) * 2;
    answer
}

fn part2(input_file: String) -> usize {
    let mut answer: usize;
    let size = input_file.lines().collect::<Vec<&str>>()[0].len();
    let mut grid: Vec<Vec<i32>> = Vec::new();
    for line in input_file.lines() {
        let mut tmp: Vec<i32> = Vec::new();
        for i in line.chars() {
            tmp.push(i.to_string().parse::<i32>().unwrap());
        }
        grid.push(tmp);
    }
    // dbg!(&grid);
    let mut scenic_scores: Vec<usize> = Vec::new();
    for x in 1..size - 1 {
        for y in 1..size - 1 {
            let val = grid[x][y];
            // println!("{x}:{y} {}", &val);
            // checking column
            let mut view_left = 0;
            let mut view_right = 0;
            let left_range = (0..y).rev();
            for c in left_range {
                if &grid[x][c] < &val {
                    view_left += 1;
                } else {
                    view_left += 1;
                    break;
                }
            }
            let right_range = y + 1..size;
            for c in right_range {
                if &grid[x][c] < &val {
                    view_right += 1;
                } else {
                    view_right += 1;
                    break;
                }
            }
            // checking row
            let mut view_up = 0;
            let mut view_down = 0;
            let up_range = (0..x).rev();
            for c in up_range {
                if &grid[c][y] < &val {
                    view_up += 1;
                } else {
                    view_up += 1;
                    break;
                }
            }
            let down_range = x + 1..size;
            for c in down_range {
                if &grid[c][y] < &val {
                    view_down += 1;
                } else {
                    view_down += 1;
                    break;
                }
            }
            scenic_scores.push(view_left * view_right * view_up * view_down);
        }
    }
    answer = *scenic_scores.iter().max().unwrap();
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
