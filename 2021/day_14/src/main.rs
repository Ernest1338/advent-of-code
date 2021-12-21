#![allow(dead_code, unused_variables, unused_mut)]

use std::collections::HashMap;
use std::fs::read_to_string;

fn insert_polymer(polymer: Vec<char>, insert_rules: HashMap<&str, char>) -> Vec<char> {
    let mut out_polymer: Vec<char> = vec![];

    out_polymer.push(polymer[0]);
    for window in polymer.windows(2) {
        //out_polymer.push(window[0]);
        out_polymer.push(insert_rules[&*window.iter().collect::<String>()]);
        //out_polymer.push(insert_rules["BH"]);
        //dbg!(insert_rules[&*window.iter().collect::<String>()]);
        out_polymer.push(window[1]);
    }

    return out_polymer;
}

fn insert_pairs_efficiently(pairs: Vec<usize>, insert_rules: HashMap<&str, char>) -> Vec<usize> {
    let mut out_pairs: Vec<usize> = pairs.to_owned();

    for i in 0..pairs.len() {
        if pairs[i] != 0 {
            /*println!(
                "pair: {}, inserting: {} and {}",
                pair_at_index(i),
                &format!(
                    "{}{}",
                    pair_at_index(i).chars().nth(0).unwrap(),
                    insert_rules[&*pair_at_index(i)]
                ),
                &format!(
                    "{}{}",
                    insert_rules[&*pair_at_index(i)],
                    pair_at_index(i).chars().nth(1).unwrap()
                )
            );*/
            out_pairs[index_of_pair(&format!(
                "{}{}",
                pair_at_index(i).chars().nth(0).unwrap(),
                insert_rules[&*pair_at_index(i)]
            ))] += pairs[i];
            out_pairs[index_of_pair(&format!(
                "{}{}",
                insert_rules[&*pair_at_index(i)],
                pair_at_index(i).chars().nth(1).unwrap()
            ))] += pairs[i];
            /*dbg!(format!(
                "{}{}",
                pair_at_index(i).chars().nth(0).unwrap(),
                insert_rules[&*pair_at_index(i)]
            ));
            dbg!(format!(
                "{}{}",
                insert_rules[&*pair_at_index(i)],
                pair_at_index(i).chars().nth(1).unwrap()
            ));*/
            out_pairs[i] -= pairs[i];
        }
    }

    return out_pairs;
}

fn index_of_pair(pair: &str) -> usize {
    let mut index: usize = 0;
    let alphabet: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let first_char = pair.chars().nth(0).unwrap();
    let first_char_pos = alphabet.iter().position(|&e| e == first_char).unwrap();
    let second_char = pair.chars().nth(1).unwrap();
    let second_char_pos = alphabet.iter().position(|&e| e == second_char).unwrap();

    index += first_char_pos * 26 + second_char_pos;

    return index;
}

fn pair_at_index(index: usize) -> String {
    let mut pair: String;
    let alphabet: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];
    let first_char_pos: usize = (index as f64 / 26.0).floor() as usize;
    let second_char_pos: usize = index % 26 as usize;

    pair = format!("{}{}", alphabet[first_char_pos], alphabet[second_char_pos]);

    return pair;
}

fn print_pairs(pairs: &Vec<usize>) {
    println!();
    for i in 0..pairs.len() {
        if pairs[i] != 0 {
            for x in 0..pairs[i] {
                println!("pair({}): {}", i, pair_at_index(i));
            }
        }
    }
    println!();
}

fn part1(input_file: String) {
    // make use of .windows()
    let answer: usize;
    let mut polymer: Vec<char> = input_file.split("\n\n").collect::<Vec<&str>>()[0]
        .trim()
        .chars()
        .collect();
    //dbg!(&polymer);
    let mut insertion_rules: HashMap<&str, char> = HashMap::new();
    for rule in input_file.split("\n\n").collect::<Vec<&str>>()[1]
        .trim()
        .split("\n")
    {
        insertion_rules.insert(
            rule.split(" -> ").nth(0).unwrap(),
            rule.split(" -> ").nth(1).unwrap().chars().nth(0).unwrap(),
        );
    }
    //dbg!(&insertion_rules);
    for i in 0..10 {
        // 10 steps
        polymer = insert_polymer(polymer, insertion_rules.to_owned());
        //dbg!(&polymer.len());
    }
    let mut uniq_chrs: Vec<char> = polymer.to_owned();
    uniq_chrs.sort();
    uniq_chrs.dedup();
    //dbg!(&uniq_chrs);
    let mut chrs_values: Vec<usize> = vec![0; uniq_chrs.len()];
    for chr in 0..uniq_chrs.len() {
        for element in &polymer {
            if *element == uniq_chrs[chr] {
                chrs_values[chr] += 1;
            }
        }
    }
    //dbg!(&chrs_values);

    answer = chrs_values.iter().max().unwrap() - chrs_values.iter().min().unwrap();

    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    let answer: usize;
    let mut insertion_rules: HashMap<&str, char> = HashMap::new();
    for rule in input_file.split("\n\n").collect::<Vec<&str>>()[1]
        .trim()
        .split("\n")
    {
        insertion_rules.insert(
            rule.split(" -> ").nth(0).unwrap(),
            rule.split(" -> ").nth(1).unwrap().chars().nth(0).unwrap(),
        );
    }
    //dbg!(&insertion_rules);
    let mut pairs: Vec<usize> = vec![0; 676];
    //dbg!(&pairs);
    //dbg!(index_of_pair("BA"));
    //dbg!(pair_at_index(26));
    let mut starting_polymer: Vec<char> = input_file.split("\n\n").collect::<Vec<&str>>()[0]
        .trim()
        .chars()
        .collect();
    let mut starting_pairs: Vec<String> = starting_polymer
        .windows(2)
        .map(|e| e.iter().collect::<String>())
        .collect();
    for pair in starting_pairs {
        pairs[index_of_pair(&pair)] += 1;
    }
    //dbg!(&pairs);
    //print_pairs(&pairs);
    for i in 0..40 {
        // 40 steps
        //dbg!(i);
        pairs = insert_pairs_efficiently(pairs, insertion_rules.to_owned());
        //print_pairs(&pairs);
        //dbg!(i, &pairs);
    }
    //dbg!(&pairs);

    let uniq_chars: Vec<char> = vec![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ];

    let mut chrs_values: Vec<usize> = vec![0; 26];

    for pair_index in 0..pairs.len() {
        if pairs[pair_index] != 0 {
            for chr_index in 0..uniq_chars.len() {
                if pair_at_index(pair_index).chars().nth(0).unwrap() == uniq_chars[chr_index] {
                    chrs_values[chr_index] += pairs[pair_index];
                }
                if pair_at_index(pair_index).chars().nth(1).unwrap() == uniq_chars[chr_index] {
                    chrs_values[chr_index] += pairs[pair_index];
                }
            }
        }
    }

    let mut chrs_values_no_zeros: Vec<usize> = vec![];

    for value_index in 0..chrs_values.len() {
        if chrs_values[value_index] != 0 {
            chrs_values[value_index] =
                ((chrs_values[value_index] + 1) as f64 / 2.0).floor() as usize;
            chrs_values_no_zeros.push(chrs_values[value_index]);
        }
    }

    /*for i in 0..chrs_values.len() {
        println!("{}: {}", uniq_chars[i], chrs_values[i]);
    }*/

    answer =
        chrs_values_no_zeros.iter().max().unwrap() - chrs_values_no_zeros.iter().min().unwrap();

    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 {
        part1(input_file);
    } else if part == 2 {
        part2(input_file);
    }
}
