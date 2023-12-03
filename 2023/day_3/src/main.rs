#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let mut parsed: Vec<Vec<char>> = input_file
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    parsed.iter_mut().for_each(|l| {
        l.insert(0, '.');
        l.push('.');
    });
    parsed.insert(0, vec!['.'; parsed[0].len()]);
    parsed.push(vec!['.'; parsed[0].len()]);

    let mut num = String::new();
    let mut is_a_part = true;
    for x in 1..parsed[0].len() - 1 {
        for y in 1..parsed.len() - 1 {
            let v = parsed[x][y];
            if v.is_ascii_digit() {
                num.push(v);
                if (!parsed[x - 1][y - 1].is_ascii_digit() && parsed[x - 1][y - 1] != '.')
                    || (!parsed[x][y - 1].is_ascii_digit() && parsed[x][y - 1] != '.')
                    || (!parsed[x + 1][y - 1].is_ascii_digit() && parsed[x + 1][y - 1] != '.')
                    || (!parsed[x - 1][y].is_ascii_digit() && parsed[x - 1][y] != '.')
                    || (!parsed[x + 1][y].is_ascii_digit() && parsed[x + 1][y] != '.')
                    || (!parsed[x - 1][y + 1].is_ascii_digit() && parsed[x - 1][y + 1] != '.')
                    || (!parsed[x][y + 1].is_ascii_digit() && parsed[x][y + 1] != '.')
                    || (!parsed[x + 1][y + 1].is_ascii_digit() && parsed[x + 1][y + 1] != '.')
                {
                    is_a_part = false;
                }
            } else {
                if !is_a_part && !num.is_empty() {
                    answer += num.parse::<usize>().unwrap();
                }
                num.clear();
                is_a_part = true;
            }
        }
    }

    answer
}

fn is_a_gear(symbols: &Vec<char>) -> bool {
    fn how_much_nums(line: &[char]) -> usize {
        if line[0] == '.' && line[1] == '.' && line[2] == '.' {
            0
        } else if line[0].is_ascii_digit() && line[1] == '.' && line[2].is_ascii_digit() {
            2
        } else {
            1
        }
    }
    let mut adjecent = 0;
    adjecent += how_much_nums(&symbols[0..3]);
    if symbols[3].is_ascii_digit() {
        adjecent += 1;
    }
    if symbols[4].is_ascii_digit() {
        adjecent += 1;
    }
    adjecent += how_much_nums(&symbols[5..8]);
    adjecent == 2
}

fn extract_nums_from_gear(input: &Vec<Vec<char>>, gear: (usize, usize)) -> usize {
    let mut result = 1;
    fn extract_nums_from_line(line: &Vec<char>) -> Vec<(usize, (usize, usize))> {
        let mut nums: Vec<(usize, (usize, usize))> = Vec::new();
        let mut num = String::new();
        let mut begin = None;
        for (i, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                num.push(*c);
                if begin.is_none() {
                    begin = Some(i);
                }
            } else {
                if !num.is_empty() {
                    nums.push((num.parse::<usize>().unwrap(), (begin.unwrap(), i)));
                    num.clear();
                }
                begin = None;
            }
        }
        nums
    }
    let mut nums_around: Vec<usize> = Vec::new();
    let mut nums_in_line = extract_nums_from_line(&input[gear.0 - 1]);
    if input[gear.0 - 1][gear.1] == '.' {
        for num in &nums_in_line {
            if (num.1 .0..num.1 .1).contains(&(gear.1 - 1)) {
                nums_around.push(num.0);
            }
            if (num.1 .0..num.1 .1).contains(&(gear.1 + 1)) {
                nums_around.push(num.0);
            }
        }
    } else {
        for num in &nums_in_line {
            if (num.1 .0..num.1 .1).contains(&(gear.1 - 1)) {
                nums_around.push(num.0);
            } else if (num.1 .0..num.1 .1).contains(&(gear.1)) {
                nums_around.push(num.0);
            } else if (num.1 .0..num.1 .1).contains(&(gear.1 + 1)) {
                nums_around.push(num.0);
            }
        }
    }
    nums_in_line = extract_nums_from_line(&input[gear.0 + 1]);
    if input[gear.0 - 1][gear.1] == '.' {
        for num in &nums_in_line {
            if (num.1 .0..num.1 .1).contains(&(gear.1 - 1)) {
                nums_around.push(num.0);
            }
            if (num.1 .0..num.1 .1).contains(&(gear.1 + 1)) {
                nums_around.push(num.0);
            }
        }
    } else {
        for num in &nums_in_line {
            if (num.1 .0..num.1 .1).contains(&(gear.1 - 1)) {
                nums_around.push(num.0);
            } else if (num.1 .0..num.1 .1).contains(&(gear.1)) {
                nums_around.push(num.0);
            } else if (num.1 .0..num.1 .1).contains(&(gear.1 + 1)) {
                nums_around.push(num.0);
            }
        }
    }
    nums_in_line = extract_nums_from_line(&input[gear.0]);
    for num in &nums_in_line {
        if (num.1 .0..num.1 .1).contains(&(gear.1 - 1)) {
            nums_around.push(num.0);
        }
        if (num.1 .0..num.1 .1).contains(&(gear.1 + 1)) {
            nums_around.push(num.0);
        }
    }
    nums_around.dedup();
    if nums_around.len() != 2 {
        panic!("WTF {nums_around:?} {gear:?}");
    }
    for num in nums_around {
        result *= num;
    }
    result
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let mut parsed: Vec<Vec<char>> = input_file
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    parsed.iter_mut().for_each(|l| {
        l.insert(0, '.');
        l.push('.');
    });
    parsed.insert(0, vec!['.'; parsed[0].len()]);
    parsed.push(vec!['.'; parsed[0].len()]);

    let mut gears: Vec<(usize, usize)> = Vec::new();
    for x in 1..parsed[0].len() - 1 {
        for y in 1..parsed.len() - 1 {
            if parsed[x][y] == '*' {
                let p = parsed.clone();
                let around = vec![
                    p[x - 1][y - 1],
                    p[x - 1][y],
                    p[x - 1][y + 1],
                    p[x][y - 1],
                    p[x][y + 1],
                    p[x + 1][y - 1],
                    p[x + 1][y],
                    p[x + 1][y + 1],
                ];
                if is_a_gear(&around) {
                    gears.push((x, y));
                }
            }
        }
    }

    for gear in gears {
        answer += extract_nums_from_gear(&parsed, gear);
    }

    answer
}

fn main() {
    let input_file = load_file("input.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
