#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn part1_fail(input_file: String) {
    // This is broken but it's a good starting point for solving the part 2
    // TODO: Handle the dup case (part1 - has_dups code)
    let mut answer: usize = 0;
    let mut plane_lines: Vec<&str> = input_file.split("\n").collect::<Vec<&str>>();
    if plane_lines.last().unwrap() == &"" { plane_lines.pop(); }
    let mut plane: Vec<Vec<usize>> = vec![vec![]; plane_lines.len()];
    for line_index in 0..plane_lines.len() {
        for s in plane_lines[line_index].chars() {
            plane[line_index].push(s.to_string().parse().unwrap());
        }
    }
    let mut low_points: Vec<Vec<usize>> = vec![];
    for x in 0..plane.len() {
        for y in 0..plane[x].len() {
            dbg!(x, y);
            let mut found_low_point: bool = false;
            let mut cur_cords: Vec<usize> = vec![x, y];
            while !found_low_point {
                if cur_cords[0] as i32 - 1 >= 0 { // can go up
                    //dbg!("can go up");
                    if plane[cur_cords[0] - 1][cur_cords[1]] < plane[cur_cords[0]][cur_cords[1]] { // up
                        dbg!("up");
                        cur_cords = vec![cur_cords[0] - 1, cur_cords[1]];
                        continue;
                    }
                }
                if cur_cords[1] as i32 - 1 >= 0 { // can go left
                    //dbg!("can go left");
                    if plane[cur_cords[0]][cur_cords[1] - 1] < plane[cur_cords[0]][cur_cords[1]] { // left
                        dbg!("going left");
                        cur_cords = vec![cur_cords[0], cur_cords[1] - 1];
                        continue;
                    }
                }
                if cur_cords[0] as i32 + 1 < plane.len() as i32 { // can go down
                    //dbg!("can go down");
                    if plane[cur_cords[0] + 1][cur_cords[1]] < plane[cur_cords[0]][cur_cords[1]] { // down
                        dbg!("going down");
                        cur_cords = vec![cur_cords[0] + 1, cur_cords[1]];
                        continue;
                    }
                }
                if cur_cords[1] as i32 + 1 < plane[0].len() as i32 { // can go right
                    //dbg!("can go right");
                    if plane[cur_cords[0]][cur_cords[1] + 1] < plane[cur_cords[0]][cur_cords[1]] { // right
                        dbg!("going right");
                        cur_cords = vec![cur_cords[0], cur_cords[1] + 1];
                        continue;
                    }
                }
                found_low_point = true;
                dbg!(&cur_cords);
                low_points.push(cur_cords.to_owned());
            }
        }
    }

    low_points.sort_unstable();
    low_points.dedup();
    dbg!(&low_points);

    for point in low_points {
        answer += plane[point[0]][point[1]] + 1;
    }

    println!("Answer: {}", answer);
}

fn part1(input_file: String) {
    let mut answer: usize = 0;
    let mut plane_lines: Vec<&str> = input_file.split("\n").collect::<Vec<&str>>();
    if plane_lines.last().unwrap() == &"" { plane_lines.pop(); }
    let mut plane: Vec<Vec<usize>> = vec![vec![]; plane_lines.len()];
    for line_index in 0..plane_lines.len() {
        for s in plane_lines[line_index].chars() {
            plane[line_index].push(s.to_string().parse().unwrap());
        }
    }
    let mut low_points: Vec<Vec<usize>> = vec![];
    for x in 0..plane.len() {
        for y in 0..plane[x].len() {
            let mut is_low_point: bool = true;
            let mut has_dups: bool = false;
            if x as i32 - 1 >= 0 { // up
                if plane[x-1][y] == plane[x][y] { has_dups = true; }
                if plane[x-1][y] < plane[x][y] { is_low_point = false; }
            }
            if y as i32 - 1 >= 0 { // left
                if plane[x][y-1] == plane[x][y] { has_dups = true; }
                if plane[x][y-1] < plane[x][y] { is_low_point = false; }
            }
            if x as i32 + 1 < plane.len() as i32 { // down
                if plane[x+1][y] == plane[x][y] { has_dups = true; }
                if plane[x+1][y] < plane[x][y] { is_low_point = false; }
            }
            if y as i32 + 1 < plane[0].len() as i32 { // right
                if plane[x][y+1] == plane[x][y] { has_dups = true; }
                if plane[x][y+1] < plane[x][y] { is_low_point = false; }
            }
            if is_low_point {
                if !has_dups {
                    low_points.push(vec![x, y]);
                }
            }
        }
    }

    for point in low_points {
        answer += plane[point[0]][point[1]] + 1;
    }

    println!("Answer: {}", answer);
}
fn part2(input_file: String) {
    let mut answer: usize = 0;
    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 1;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2(input_file); }
}
