#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cords {
    pub x: i32,
    pub y: i32,
}

impl Cords {
    fn new() -> Cords {
        Cords { x: 0, y: 0 }
    }
}
fn are_touching(head: &Cords, tail: &Cords) -> bool {
    (head.x - 1..=head.x + 1).contains(&tail.x) && (head.y - 1..=head.y + 1).contains(&tail.y)
}

fn visualize(rope_elements: &[Cords]) {
    for y in (0..10).rev() {
        for x in 0..10 {
            let mut to_write = String::from(".");
            for (i, a) in rope_elements.iter().enumerate() {
                if a.x == x && a.y == y {
                    to_write = format!("{i}");
                }
            }
            print!("{to_write}");
        }
        println!();
    }
    for a in 0..12 {
        print!("-");
    }
    println!();
}

fn find_move(head: &Cords, tail: &Cords) -> Cords {
    let new_move = Cords::new();
    let mut possible_moves: Vec<Cords> = vec![
        // prioritize direct touches
        Cords {
            x: head.x,
            y: head.y + 1,
        },
        Cords {
            x: head.x,
            y: head.y - 1,
        },
        Cords {
            x: head.x - 1,
            y: head.y,
        },
        Cords {
            x: head.x + 1,
            y: head.y,
        },
        // if not found, check corners
        Cords {
            x: head.x + 1,
            y: head.y + 1,
        },
        Cords {
            x: head.x + 1,
            y: head.y - 1,
        },
        Cords {
            x: head.x - 1,
            y: head.y - 1,
        },
        Cords {
            x: head.x - 1,
            y: head.y + 1,
        },
    ];
    for pos_move in possible_moves {
        if (pos_move.x - 1..=pos_move.x + 1).contains(&tail.x)
            && (pos_move.y - 1..=pos_move.y + 1).contains(&tail.y)
        {
            return pos_move;
        }
    }
    new_move
}

fn get_ammount_of_tails_positions(input_file: &str, rope_length: usize) -> usize {
    let mut rope_elements: Vec<Cords> = vec![Cords::new(); rope_length];
    let range = (1..rope_length).rev().collect::<Vec<usize>>();
    let mut tail_positions: Vec<Cords> = vec![Cords { x: 0, y: 0 }];
    for line in input_file.lines() {
        let line_split = line.split(' ').collect::<Vec<&str>>();
        let dir = line_split[0];
        let steps = line_split[1].parse::<usize>().unwrap();
        for i in 0..steps {
            match dir {
                "U" => {
                    rope_elements.last_mut().unwrap().y += 1;
                }
                "D" => {
                    rope_elements.last_mut().unwrap().y -= 1;
                }
                "L" => {
                    rope_elements.last_mut().unwrap().x -= 1;
                }
                "R" => {
                    rope_elements.last_mut().unwrap().x += 1;
                }
                _ => {}
            }
            for a in &range {
                if !are_touching(&rope_elements[*a], &rope_elements[a - 1]) {
                    rope_elements[*a - 1] = find_move(&rope_elements[*a], &rope_elements[a - 1]);
                }
            }
            if !tail_positions.contains(&rope_elements[0]) {
                tail_positions.push(rope_elements[0]);
            }
            // visualize(&rope_elements);
        }
    }
    tail_positions.len()
}

fn part1(input_file: String) -> usize {
    get_ammount_of_tails_positions(&input_file, 2)
}

fn part2(input_file: String) -> usize {
    get_ammount_of_tails_positions(&input_file, 10)
}

fn main() {
    let part = 2;

    let input_file = load_file("input.txt");

    if part == 1 {
        print_answer(part1(input_file));
    } else if part == 2 {
        print_answer(part2(input_file));
    }
}
