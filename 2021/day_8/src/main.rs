#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn part1(input_file: String) {
    let mut answer: usize = 0;
    let mut connections_and_digits: Vec<Vec<Vec<&str>>> = input_file.trim().split("\n").collect::<Vec<&str>>().iter().map(|s| s.split(" | ").collect::<Vec<&str>>().iter().map(|x| x.split(" ").collect::<Vec<&str>>()).collect()).collect();
    let mut digits: Vec<&str> = vec![];
    for i in 0..connections_and_digits.len() {
        for digit in connections_and_digits[i][1].iter() {
            digits.push(digit);
        }
    }
    let uniq_digit_lentghts: Vec<usize> = vec![2, 3, 4, 7];
    for digit in &digits {
        if uniq_digit_lentghts.contains(&digit.len()) {
            answer += 1;
        }
    }
    println!("Answer: {}", answer);
}

fn part2(input_file: String) {
    // TODO: simplify the solution by creating a vec diff function
    let mut answer: String = String::new();
    let mut final_answer: usize = 0;
    let mut connections_and_digits: Vec<Vec<Vec<&str>>> = input_file.trim().split("\n").collect::<Vec<&str>>().iter().map(|s| s.split(" | ").collect::<Vec<&str>>().iter().map(|x| x.split(" ").collect::<Vec<&str>>()).collect()).collect();
    let mut connections: Vec<&str>;
    let mut digits: Vec<&str>;
    for message in connections_and_digits {
        let mut one: Vec<char> = vec![];
        let mut four: Vec<char> = vec![];
        let mut seven: Vec<char> = vec![];
        let mut eight: Vec<char> = vec![];
        let mut segments: Vec<char> = vec!['.'; 7];
        let mut digit_lookup_vec: Vec<String> = vec![];

        connections = message.first().unwrap().to_vec();
        digits = message.last().unwrap().to_vec();

        for digit in &connections {
            match digit.len() {
                2 => for chr in digit.chars() { one.push(chr); },
                4 => for chr in digit.chars() { four.push(chr); },
                3 => for chr in digit.chars() { seven.push(chr); },
                7 => for chr in digit.chars() { eight.push(chr); },
                _ => continue,
            }
        }

        for chr in &seven { if !one.contains(&chr) { segments[0] = chr.to_owned(); } } // top segment

        let mut four_one_dif: Vec<char> = vec![];
        for chr in &four { if !one.contains(&chr) { four_one_dif.push(chr.to_owned()); } }

        for digit in &connections {
            if digit.len() == 5 { // 3
                if digit.contains(one[0]) && digit.contains(one[1]) && digit.contains(segments[0]) {
                    if digit.contains(four_one_dif[0]) || digit.contains(four_one_dif[1]) {
                        if digit.contains(four_one_dif[0]) { segments[3] = four_one_dif[0]; } // middle segment
                        else if digit.contains(four_one_dif[1]) { segments[3] = four_one_dif[1]; } // middle segment
                        for chr in digit.to_owned().chars() {
                            if !one.contains(&chr) && !four_one_dif.contains(&chr) && chr != segments[0] {
                                segments[6] = chr; // bottom segment
                            }
                        }
                    }
                }
            }
        }

        for digit in &connections {
            if digit.len() == 5 { // 5
                if digit.contains(segments[0]) && digit.contains(segments[6]) && digit.contains(four_one_dif[0]) && digit.contains(four_one_dif[1]) {
                    for chr in digit.to_owned().chars() {
                        if chr != segments[0] && chr != segments[6] && chr != four_one_dif[0] && chr != four_one_dif[1] {
                            segments[5] = chr; // bottom right segment
                        }
                    }
                }
            }
        }

        for chr in one { if chr != segments[5] { segments[2] = chr; } } // top right segment

        for digit in &connections {
            if digit.len() == 5 { // 2
                if digit.contains(segments[0]) && digit.contains(segments[2]) && digit.contains(segments[6]) {
                    if digit.contains(four_one_dif[0]) || digit.contains(four_one_dif[1]) {
                        for chr in digit.to_owned().chars() {
                            if chr != segments[0] && chr != segments[2] && chr != segments[6] && chr != four_one_dif[0] && chr != four_one_dif[1] && chr != segments[5] {
                                segments[4] = chr; // bottom left segment
                            }
                        }
                    }
                }
            }
        }

        for digit in &connections {
            if digit.len() == 7 { // 8
                for chr in digit.to_owned().chars() {
                    if !segments.contains(&chr) {
                        segments[1] = chr;
                    }
                }
            }
        }

        digit_lookup_vec.push(format!("{}{}{}{}{}{}", segments[0], segments[1], segments[2], segments[4], segments[5], segments[6]));
        digit_lookup_vec.push(format!("{}{}", segments[2], segments[5]));
        digit_lookup_vec.push(format!("{}{}{}{}{}", segments[0], segments[2], segments[3], segments[4], segments[6]));
        digit_lookup_vec.push(format!("{}{}{}{}{}", segments[0], segments[2], segments[3], segments[5], segments[6]));
        digit_lookup_vec.push(format!("{}{}{}{}", segments[1], segments[2], segments[3], segments[5]));
        digit_lookup_vec.push(format!("{}{}{}{}{}", segments[0], segments[1], segments[3], segments[5], segments[6]));
        digit_lookup_vec.push(format!("{}{}{}{}{}{}", segments[0], segments[1], segments[3], segments[4], segments[5], segments[6]));
        digit_lookup_vec.push(format!("{}{}{}", segments[0], segments[2], segments[5]));
        digit_lookup_vec.push(format!("{}{}{}{}{}{}{}", segments[0], segments[1], segments[2], segments[3], segments[4], segments[5], segments[6]));
        digit_lookup_vec.push(format!("{}{}{}{}{}{}", segments[0], segments[1], segments[2], segments[3], segments[5], segments[6]));

        let mut tmp_vec1: Vec<String>;
        let mut tmp_vec2: Vec<String>;
        let mut final_num: usize = 0;
        for digit in digits {
            for diglookup_index in 0..digit_lookup_vec.len() {
                let mut yes: bool = true;
                for chr in digit.to_owned().chars() {
                    if !digit_lookup_vec[diglookup_index].contains(chr) {
                        yes = false;
                    }
                }
                if yes && digit.len() == digit_lookup_vec[diglookup_index].len() { final_num = diglookup_index; break; }
            }
            answer += &final_num.to_string();
        }
        answer += &"\n".to_string();
    }
    let mut answer_vec: Vec<&str> = answer.split("\n").collect();
    answer_vec.pop();
    for num in answer_vec {
        final_answer += num.parse::<usize>().unwrap();
    }
    println!("Answer: {}", final_answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2(input_file); }
}
