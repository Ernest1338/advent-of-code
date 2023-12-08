#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::str::Lines;

fn parse(lines: Lines<'_>) -> HashMap<String, (String, String)> {
    let mut out: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        let mut split = line.split_whitespace();
        let key = split.nth(0).unwrap();
        let v1 = &split.nth(1).unwrap()[1..4];
        let v2 = &split.next().unwrap()[0..3];
        out.insert(key.to_string(), (v1.to_string(), v2.to_string()));
    }
    out
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let mut lines = input_file.lines();
    let mut cur = "AAA";
    let lr = lines.next().unwrap().chars().collect::<Vec<char>>();
    let map = parse(lines);
    let mut i = 0;
    while cur != "ZZZ" {
        answer += 1;
        match lr[i] {
            'L' => {
                cur = &map[cur].0;
            }
            'R' => {
                cur = &map[cur].1;
            }
            _ => (),
        }
        i += 1;
        if i == lr.len() {
            i = 0;
        }
    }
    answer
}

fn part2_try1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let mut lines = input_file.lines();
    let lr = lines.next().unwrap().chars().collect::<Vec<char>>();
    let map = parse(lines);
    let mut i = 0;
    let keys: Vec<&str> = input_file
        .lines()
        .skip(1)
        .map(|v| v.split_whitespace().next().unwrap())
        .collect::<Vec<&str>>();
    let mut curs: Vec<&str> = Vec::new();
    for k in &keys {
        if k.ends_with("A") {
            curs.push(k);
        }
    }
    while !curs.iter().all(|v| v.ends_with("Z")) {
        answer += 1;
        if answer % 100000 == 0 {
            println!("{answer}");
        }
        // for x in 0..curs.len() {
        //     if lr[i] == 'L' {
        //         curs[x] = &map[curs[x]].0;
        //     } else if lr[i] == 'R' {
        //         curs[x] = &map[curs[x]].1;
        //     }
        // }
        // println!("{curs:?}");
        curs.par_iter_mut().for_each(|curs_x| {
            if lr[i] == 'L' {
                // println!("{} = {}", curs_x, map[*curs_x].0);
                *curs_x = &map[*curs_x].0;
            } else if lr[i] == 'R' {
                // println!("{} = {}", curs_x, map[*curs_x].1);
                *curs_x = &map[*curs_x].1;
            }
        });
        i += 1;
        if i == lr.len() {
            i = 0;
        }
        // println!("{curs:?}");
    }
    answer
}

fn get_key(s: &str) -> usize {
    let key_chars = s.chars().collect::<Vec<char>>();
    3 * (key_chars[0] as usize) + 5 * (key_chars[1] as usize) + 7 * (key_chars[2] as usize)
}

fn parse_part2(lines: Lines<'_>) -> Vec<(usize, usize)> {
    let mut out = vec![(0, 0); 1500];
    for line in lines {
        let mut split = line.split_whitespace();
        // let key_chars = split.nth(0).unwrap().chars().collect::<Vec<char>>();
        // let key =
        //     3 * (key_chars[0] as usize) + 5 * (key_chars[1] as usize) + 7 * (key_chars[2] as usize);
        let key = get_key(split.nth(0).unwrap());
        // let v1_chars = &split.nth(1).unwrap()[1..4].chars().collect::<Vec<char>>();
        // let v1 =
        //     3 * (v1_chars[0] as usize) + 5 * (v1_chars[1] as usize) + 7 * (v1_chars[2] as usize);
        let v1 = get_key(&split.nth(1).unwrap()[1..4]);
        // let v2_chars = &split.next().unwrap()[0..3].chars().collect::<Vec<char>>();
        // let v2 =
        //     3 * (v2_chars[0] as usize) + 5 * (v2_chars[1] as usize) + 7 * (v2_chars[2] as usize);
        let v2 = get_key(&split.next().unwrap()[0..3]);
        out[key] = (v1, v2);
    }
    out
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    let mut lines = input_file.lines();
    let lr = lines.next().unwrap().chars().collect::<Vec<char>>();
    let map = parse(lines);
    let mut i = 0;
    let keys: Vec<&str> = input_file
        .lines()
        .skip(1)
        .map(|v| v.split_whitespace().next().unwrap())
        .collect::<Vec<&str>>();
    let mut curs: Vec<&str> = Vec::new();
    for k in &keys {
        if k.ends_with("A") {
            curs.push(k);
        }
    }
    while !curs.iter().all(|v| v.ends_with("Z")) {
        answer += 1;
        if answer % 100000 == 0 {
            println!("{answer}");
        }
        // for x in 0..curs.len() {
        //     if lr[i] == 'L' {
        //         curs[x] = &map[curs[x]].0;
        //     } else if lr[i] == 'R' {
        //         curs[x] = &map[curs[x]].1;
        //     }
        // }
        // println!("{curs:?}");
        curs.par_iter_mut().for_each(|curs_x| {
            if lr[i] == 'L' {
                // println!("{} = {}", curs_x, map[*curs_x].0);
                *curs_x = &map[*curs_x].0;
            } else if lr[i] == 'R' {
                // println!("{} = {}", curs_x, map[*curs_x].1);
                *curs_x = &map[*curs_x].1;
            }
        });
        i += 1;
        if i == lr.len() {
            i = 0;
        }
        // println!("{curs:?}");
    }
    answer
}

fn main() {
    let input_file = load_file("input.txt");

    // println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
