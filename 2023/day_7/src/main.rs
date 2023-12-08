#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn card_value(card: char) -> usize {
    match card {
        '2' => 1,
        '3' => 2,
        '4' => 3,
        '5' => 4,
        '6' => 5,
        '7' => 6,
        '8' => 7,
        '9' => 8,
        'T' => 9,
        'J' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => 0,
    }
}

fn stronger(card1: char, card2: char) -> bool {
    card_value(card1) > card_value(card2)
}

fn hand_value(hand: &Vec<char>) -> usize {
    // one pair => 15
    // two pair => 16
    // three of a kind => 17
    // full house => 18
    // four of a kind => 19
    // five of a kind => 20
    0
}

fn part1(input_file: &str) -> usize {
    let mut answer: usize = 0;
    for hand in input_file.lines() {
        let cards: Vec<char> = hand.split_whitespace().nth(0).unwrap().chars().collect();
        let bid: usize = hand
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let value = hand_value(&cards);
        println!("{value} {cards:?} {bid}");
    }
    answer
}

fn part2(input_file: &str) -> usize {
    let mut answer: usize = 0;
    // for line in input_file.lines() {}
    answer
}

fn main() {
    let input_file = load_file("example.txt");

    println!("[Part 1] Answer: {}", part1(&input_file));
    println!("[Part 2] Answer: {}", part2(&input_file));
}
