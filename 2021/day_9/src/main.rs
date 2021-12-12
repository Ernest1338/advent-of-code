#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn part1_fail(input_file: String) {
    // This is now fixed and it's the base of the part 2 solution
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
            let mut has_dups: bool = false;
            while !found_low_point {
                println!("cur cords: {:?}", &cur_cords);
                if cur_cords[0] as i32 - 1 >= 0 { // can go up
                    dbg!("can go up");
                    if plane[cur_cords[0] - 1][cur_cords[1]] == plane[cur_cords[0]][cur_cords[1]] {
                        has_dups = true;
                    }
                    if plane[cur_cords[0] - 1][cur_cords[1]] < plane[cur_cords[0]][cur_cords[1]] { // up
                        dbg!("going up");
                        cur_cords = vec![cur_cords[0] - 1, cur_cords[1]];
                        continue;
                    }
                }
                if cur_cords[1] as i32 - 1 >= 0 { // can go left
                    dbg!("can go left");
                    if plane[cur_cords[0]][cur_cords[1] - 1] == plane[cur_cords[0]][cur_cords[1]] {
                        has_dups = true;
                    }
                    if plane[cur_cords[0]][cur_cords[1] - 1] < plane[cur_cords[0]][cur_cords[1]] { // left
                        dbg!("going left");
                        cur_cords = vec![cur_cords[0], cur_cords[1] - 1];
                        continue;
                    }
                }
                if cur_cords[0] as i32 + 1 < plane.len() as i32 { // can go down
                    dbg!("can go down");
                    if plane[cur_cords[0] + 1][cur_cords[1]] == plane[cur_cords[0]][cur_cords[1]] {
                        has_dups = true;
                    }
                    if plane[cur_cords[0] + 1][cur_cords[1]] < plane[cur_cords[0]][cur_cords[1]] { // down
                        dbg!("going down");
                        cur_cords = vec![cur_cords[0] + 1, cur_cords[1]];
                        continue;
                    }
                }
                if cur_cords[1] as i32 + 1 < plane[0].len() as i32 { // can go right
                    dbg!("can go right");
                    println!("cur cords: {:?}", &cur_cords);
                    dbg!(plane[cur_cords[0]][cur_cords[1]], plane[cur_cords[0]][cur_cords[1] + 1]);
                    if plane[cur_cords[0]][cur_cords[1] + 1] < plane[cur_cords[0]][cur_cords[1]] {
                        has_dups = true;
                    }
                    if plane[cur_cords[0]][cur_cords[1] + 1] < plane[cur_cords[0]][cur_cords[1]] { // right
                        dbg!("going right");
                        cur_cords = vec![cur_cords[0], cur_cords[1] + 1];
                        continue;
                    }
                }
                found_low_point = true;
                if !has_dups {
                    dbg!(&cur_cords);
                    low_points.push(cur_cords.to_owned());
                }
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
    let mut answer: usize = 1;
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
            //println!("x: {}, y: {}", x, y);
            if plane[x][y] == 9 { continue; } // if it's a 9'ine - ignore and continue
            let mut found_low_point: bool = false;
            let mut cur_cords: Vec<usize> = vec![x, y];
            let mut going_posibilities: Vec<usize> = vec![];
            let mut going_posibilities_sorted: Vec<usize>;
            let mut going_posibilities_directions: Vec<&str> = vec![];
            while !found_low_point {
                //println!("cur cords: {:?}", &cur_cords);
                if cur_cords[0] as i32 - 1 >= 0 { // can go up
                    //dbg!("can go up");
                    if plane[cur_cords[0] - 1][cur_cords[1]] <= plane[cur_cords[0]][cur_cords[1]] { // up
                        going_posibilities.push(plane[cur_cords[0] - 1][cur_cords[1]]);
                        going_posibilities_directions.push("up");
                    }
                }
                if cur_cords[1] as i32 - 1 >= 0 { // can go left
                    //dbg!("can go left");
                    if plane[cur_cords[0]][cur_cords[1] - 1] <= plane[cur_cords[0]][cur_cords[1]] { // left
                        going_posibilities.push(plane[cur_cords[0]][cur_cords[1] - 1]);
                        going_posibilities_directions.push("left");
                    }
                }
                if cur_cords[0] as i32 + 1 < plane.len() as i32 { // can go down
                    //dbg!("can go down");
                    if plane[cur_cords[0] + 1][cur_cords[1]] <= plane[cur_cords[0]][cur_cords[1]] { // down
                        going_posibilities.push(plane[cur_cords[0] + 1][cur_cords[1]]);
                        going_posibilities_directions.push("down");
                    }
                }
                if cur_cords[1] as i32 + 1 < plane[0].len() as i32 { // can go right
                    //dbg!("can go right");
                    if plane[cur_cords[0]][cur_cords[1] + 1] <= plane[cur_cords[0]][cur_cords[1]] { // right
                        going_posibilities.push(plane[cur_cords[0]][cur_cords[1] + 1]);
                        going_posibilities_directions.push("right");
                    }
                }

                //dbg!(&going_posibilities, &going_posibilities_directions);

                going_posibilities_sorted = going_posibilities.to_owned();
                going_posibilities_sorted.sort_unstable();

                //dbg!(&going_posibilities_sorted);

                if going_posibilities.len() > 0 {
                    if going_posibilities_directions[going_posibilities.iter().position(|d| d == &going_posibilities_sorted[0]).unwrap()] == "up" {
                        //dbg!("going up");
                        cur_cords = vec![cur_cords[0] - 1, cur_cords[1]];
                        going_posibilities.clear();
                        going_posibilities_directions.clear();
                        going_posibilities_sorted.clear();
                        continue;
                    }
                    else if going_posibilities_directions[going_posibilities.iter().position(|d| d == &going_posibilities_sorted[0]).unwrap()] == "left" {
                        //dbg!("going left");
                        cur_cords = vec![cur_cords[0], cur_cords[1] - 1];
                        going_posibilities.clear();
                        going_posibilities_directions.clear();
                        going_posibilities_sorted.clear();
                        continue;
                    }
                    else if going_posibilities_directions[going_posibilities.iter().position(|d| d == &going_posibilities_sorted[0]).unwrap()] == "down" {
                        //dbg!("going down");
                        cur_cords = vec![cur_cords[0] + 1, cur_cords[1]];
                        going_posibilities.clear();
                        going_posibilities_directions.clear();
                        going_posibilities_sorted.clear();
                        continue;
                    }
                    else if going_posibilities_directions[going_posibilities.iter().position(|d| d == &going_posibilities_sorted[0]).unwrap()] == "right" {
                        //dbg!("going right");
                        cur_cords = vec![cur_cords[0], cur_cords[1] + 1];
                        going_posibilities.clear();
                        going_posibilities_directions.clear();
                        going_posibilities_sorted.clear();
                        continue;
                    }
                }

                found_low_point = true;
                //println!("found point: {:?}", &cur_cords);
                low_points.push(cur_cords.to_owned());

            }
        }
    }

    let mut low_points_with_dups: Vec<Vec<usize>> = low_points.to_owned();
    low_points.sort_unstable();
    low_points.dedup();
    let mut basins: Vec<usize> = vec![0; low_points.len()];

    for point in low_points_with_dups {
        basins[low_points.iter().position(|p| p == &point).unwrap()] += 1;
    }

    basins.sort_unstable();
    basins.reverse();

    for basin in basins[0..3].iter() {
        answer *= basin;
    }

    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2(input_file); }
}
