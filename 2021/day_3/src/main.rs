#![allow(dead_code, unused_variables, unused_mut)]

use std::fs::read_to_string;

fn part1(input_file: String) {
    let answer: usize;
    let mut binary_vec_str: Vec<&str> = input_file.split("\n").collect();
    if binary_vec_str.last().unwrap().to_owned() == "" { binary_vec_str.pop(); } // remove the last element if it's empty

    let len_of_binary: usize = binary_vec_str[0].len();
    let mut gamma_vec: Vec<i32> = vec![0; len_of_binary];

    for number in binary_vec_str {
        for digit_index in 0..number.len() {
            if number.chars().nth(digit_index).unwrap() == '0' {
                gamma_vec[digit_index] -= 1;
            }
            else {
                gamma_vec[digit_index] += 1;
            }
        }
    }

    let mut gamma_binary_value: String = String::new();
    for value in &gamma_vec {
        if value > &0 {
            gamma_binary_value += &"1".to_string();
        }
        else {
            gamma_binary_value += &"0".to_string();
        }
    }

    let mut epsilon_binary_value: String = "".to_string();
    for digit in gamma_binary_value.chars() {
        if digit == '0' {
            epsilon_binary_value += &"1".to_string();
        }
        else {
            epsilon_binary_value += &"0".to_string();
        }
    }

    answer = usize::from_str_radix(&gamma_binary_value, 2).unwrap() * usize::from_str_radix(&epsilon_binary_value, 2).unwrap();

    println!("Answer: {}", answer);
}

fn part2_fail(input_file: String) {
    let answer: usize;
    let mut binary_vec_str: Vec<&str> = input_file.split("\n").collect();
    if binary_vec_str.last().unwrap().to_owned() == "" { binary_vec_str.pop(); } // remove the last element if it's empty
    let mut binary_vec_str_copy: Vec<&str> = binary_vec_str.to_owned();
    dbg!(&binary_vec_str);

    let len_of_binary: usize = binary_vec_str[0].len();
    let mut dominating_vec: Vec<i32> = vec![0; len_of_binary];

    for number in &binary_vec_str {
        for digit_index in 0..number.len() {
            if number.chars().nth(digit_index).unwrap() == '0' {
                dominating_vec[digit_index] -= 1;
            }
            else {
                dominating_vec[digit_index] += 1;
            }
        }
    }
    dbg!(&dominating_vec);
    let mut dominating_binary: String = String::new();
    for value in &dominating_vec {
        if value > &0 {
            dominating_binary += &"1".to_string();
        }
        else {
            dominating_binary += &"0".to_string();
        }
    }
    dbg!(&dominating_binary);

    let mut new_binary_vec: Vec<&str> = vec![];

    let mut count = 0;
    while &binary_vec_str.len() > &2 {
        for number in &binary_vec_str {
            if number.chars().nth(count) == dominating_binary.chars().nth(count) {
                new_binary_vec.push(number);
            }
        }
        count += 1;
        dbg!(&new_binary_vec);
        binary_vec_str = new_binary_vec.to_owned();
        new_binary_vec.clear();
    }
    dbg!(&binary_vec_str);

    let mut oxygen_generator_rating: String = String::new();
    if binary_vec_str.len() == 1 {
        oxygen_generator_rating = binary_vec_str[0].to_string();
    }
    else if binary_vec_str.len() == 2 {
        if binary_vec_str[0].chars().last().unwrap() == '1' {
            oxygen_generator_rating = binary_vec_str[0].to_string();
        }
        else {
            oxygen_generator_rating = binary_vec_str[1].to_string();
        }
    }
    dbg!(&oxygen_generator_rating);

    let mut non_dominating_binary: String = "".to_string();
    for digit in dominating_binary.chars() {
        if digit == '0' {
            non_dominating_binary += &"1".to_string();
        }
        else {
            non_dominating_binary += &"0".to_string();
        }
    }
    //dbg!(&non_dominating_binary);

    count = 0;
    while &binary_vec_str_copy.len() > &2 {
        for number in &binary_vec_str_copy {
            if number.chars().nth(count) == non_dominating_binary.chars().nth(count) {
                new_binary_vec.push(number);
            }
        }
        count += 1;
        //dbg!(&new_binary_vec);
        binary_vec_str_copy = new_binary_vec.to_owned();
        new_binary_vec.clear();
    }
    //dbg!(&binary_vec_str_copy);

    let mut co2_scrubber_rating: String = String::new();
    if binary_vec_str_copy.len() == 1 {
        co2_scrubber_rating = binary_vec_str_copy[0].to_string();
    }
    else if binary_vec_str_copy.len() == 2 {
        if binary_vec_str_copy[0].chars().last().unwrap() == '0' {
            co2_scrubber_rating = binary_vec_str_copy[0].to_string();
        }
        else {
            co2_scrubber_rating = binary_vec_str_copy[1].to_string();
        }
    }
    dbg!(&co2_scrubber_rating);
    dbg!(dominating_vec);
    dbg!(dominating_binary);
    dbg!(non_dominating_binary);

    answer = usize::from_str_radix(&oxygen_generator_rating, 2).unwrap() * usize::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    println!("Answer: {}", answer);
}

fn part2_second_try(input_file: String) {
    let answer: usize;
    let mut binary_vec_str: Vec<&str> = input_file.split("\n").collect();
    if binary_vec_str.last().unwrap().to_owned() == "" { binary_vec_str.pop(); } // remove the last element if it's empty
    let mut binary_vec_str_copy: Vec<&str> = binary_vec_str.to_owned();

    let oxygen_generator_rating: String;
    let co2_scrubber_rating: String;

    let mut tmp_vec: Vec<&str> = vec![];

    let mut ones: usize = 0;
    let mut zeros: usize = 0;
    let mut dominant: usize;
    let mut new_binary_vec: Vec<&str> = vec![];
    for d_place in 0..binary_vec_str[0].len() {
        for number_index in 0..binary_vec_str.len() {
            if binary_vec_str[number_index].chars().nth(d_place).unwrap() == '0' {
                zeros += 1;
            }
            else {
                ones += 1;
            }
        }

        if zeros > ones { // zeros are dominant
            dominant = 0;
        }
        else { // ones are dominant
            dominant = 1;
        }

        for number in &binary_vec_str {
            if number.chars().nth(d_place).unwrap().to_string() == dominant.to_string() {
                new_binary_vec.push(number);
                tmp_vec.push(number);
            }
        }

        binary_vec_str = new_binary_vec.to_owned();
        new_binary_vec.clear();

        zeros = 0;
        ones = 0;
    }
    oxygen_generator_rating = tmp_vec.last().unwrap().to_string();
    tmp_vec.clear();

    for d_place in 0..binary_vec_str_copy[0].len() {
        for number_index in 0..binary_vec_str_copy.len() {
            if binary_vec_str_copy[number_index].chars().nth(d_place).unwrap() == '0' {
                zeros += 1;
            }
            else {
                ones += 1;
            }
        }

        if zeros > ones { // zeros are dominant
            dominant = 0;
        }
        else { // ones are dominant
            dominant = 1;
        }

        for number in &binary_vec_str_copy {
            if number.chars().nth(d_place).unwrap().to_string() != dominant.to_string() {
                new_binary_vec.push(number);
                tmp_vec.push(number);
            }
        }

        binary_vec_str_copy = new_binary_vec.to_owned();
        new_binary_vec.clear();

        zeros = 0;
        ones = 0;
    }
    co2_scrubber_rating = tmp_vec.last().unwrap().to_string();

    answer = usize::from_str_radix(&oxygen_generator_rating, 2).unwrap() * usize::from_str_radix(&co2_scrubber_rating, 2).unwrap();

    println!("Answer: {}", answer);
}

fn main() {
    let part: i32 = 2;

    let input_file: String = read_to_string("input.txt").unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2_second_try(input_file); }
    else if part == 3 { part2_fail(input_file); }
}
