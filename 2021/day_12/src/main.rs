#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;
use std::fs::read_to_string;

fn permutations(sequence: Vec<&str>) -> Vec<Vec<&str>> {
    let mut result: Vec<Vec<&str>> = vec![vec![]];

    for (i, item) in permutations(sequence[0..sequence.len() - 1].into())
        .iter()
        .enumerate()
    {
        let mut n = (0..item.len() + 1).collect::<Vec<usize>>();
        if i % 2 == 0 {
            n.reverse();
        }
        for k in n {
            let mut items = Vec::new();
            for x in &item[0..k] {
                items.push(*x);
            }
            items.push(item[item.len() - 1]);
            for x in &item[k..item.len()] {
                items.push(*x);
            }
            result.push(items);
        }
    }

    return result;
}

fn part1(input_file: String) {
    // new (bad) idea: pregenerate all of the possible combinations and check if they make sense
    //let mut answer: usize = 0;
    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut combinations: Vec<Vec<&str>> = vec![];
    let mut pregen_comb: Vec<Vec<&str>> = vec![];
    let mut lines: Vec<&str> = input_file.split("\n").collect();
    if lines.last().unwrap().is_empty() {
        lines.pop();
    }
    for line in lines {
        let tmp_connection: Vec<&str> = line.split("-").collect();
        connections
            .entry(tmp_connection[0])
            .or_default()
            .push(tmp_connection[1]);
        connections
            .entry(tmp_connection[1])
            .or_default()
            .push(tmp_connection[0]);
    }
    /*for element in connections.get("start").unwrap() {
        dbg!(element);
        let mut tmp_combination: Vec<&str> = vec![];
        for el1 in connections.get(element).unwrap() {
            if el1 == &"end" { dbg!("END"); tmp_combination.push(el1); combinations.push(tmp_combination.to_owned()); break; }
            else { tmp_combination.push(el1); }
            if el1 != element && el1 != &"start" {
                dbg!(el1);
                for el2 in connections.get(el1).unwrap() {
                    if el2 == &"end" { dbg!("END"); tmp_combination.push(el2); combinations.push(tmp_combination.to_owned()); break; }
                    if el2 != el1 && el2 != element && el2 != &"start" {
                        dbg!(el2);
                    }
                }
            }
        }
    }*/
    let mut uniq_caves: Vec<&str> = vec![];
    for element in &connections {
        for cave in element.1.iter() {
            if cave != &"start" && cave != &"end" {
                if !uniq_caves.contains(cave) {
                    uniq_caves.push(cave.to_owned());
                }
            }
        }
    }
    dbg!(&uniq_caves, &connections, &pregen_comb, &combinations);
    combinations = permutations(uniq_caves);
    dbg!(&combinations);
    //println!("Answer: {}", answer);
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
