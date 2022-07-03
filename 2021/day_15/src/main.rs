#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn prand_bool() -> bool {
    let mut out_bool: bool = true;
    let rand_var: Vec<usize> = vec![1, 2, 3];
    let prand_usize: usize = (&rand_var as *const Vec<usize> as i32).abs() as usize;
    let mut prand_sum: usize = 0;
    for index in 0..prand_usize.to_string().len() {
        prand_sum += prand_usize / (10 ^ index) % 10;
    }
    if prand_sum%2==1 {
        out_bool = false;
    }
    return out_bool;
}

fn part1_naive(map: Vec<Vec<u8>>) -> usize {
    let mut out_risk: usize = 0;
    let size_of_map: (usize, usize) = (map[0].len(), map[0].len());
    let mut position: (usize, usize) = (0, 0);

    while position != size_of_map {
        dbg!(position);
        dbg!(prand_bool());
        if prand_bool() {
            position.0 += 1;
        }
        else {
            position.1 += 1;
        }
    }

    return out_risk;
}

fn part1_fail(input_file: String) {
    // Two ideas:
    // 1 - naive - random walker going right, down at random
    // 2 - better? - from the start go to a tile which has the smallest number and has not been visited yet, implement a optimizer for the final path
    let mut answer: usize = 0;
    let mut map: Vec<Vec<u8>> = input_file
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|d| d.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();
    if map.last().unwrap().is_empty() {
        map.pop();
    }
    dbg!(&map);
    dbg!(part1_naive(map));
    println!("Answer: {}", answer);
}

fn part1(input_file: String) {
    let mut answer: usize = 0;
    let mut map: Vec<Vec<u8>> = input_file
        .split("\n")
        .map(|l| {
            l.chars()
                .map(|d| d.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();
    if map.last().unwrap().is_empty() {
        map.pop();
    }
    //dbg!(&map);
    let mut new_map: Vec<Vec<usize>> = vec![];
    for (i, x) in map.iter().enumerate() {
        let mut num: usize;
        if i == 0 {
            num = 0;
        }
        else {
            num = x[0] as usize;
        }
        let mut tmp_element: Vec<usize> = vec![];
        tmp_element.push(num);
        for y in x {
            tmp_element.push(tmp_element.last().unwrap() + *y as usize);
        }
        new_map.push(tmp_element);
    }
    dbg!(&new_map);
    for x in new_map {
        for y in x {
            print!(" {}", y);
        }
        println!();
    }
    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let mut answer: usize = 0;
    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 1;

    let input_file: String = read_to_string("sample.txt").unwrap();

    if part == 1 {
        part1(input_file);
    } else if part == 2 {
        part2(input_file);
    }
}
