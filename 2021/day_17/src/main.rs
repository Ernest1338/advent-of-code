#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

fn iterate_step(x: &mut i32, y: &mut i32, vel_x: &mut i32, vel_y: &mut i32) {
    *x += *vel_x;
    *y += *vel_y;
    if *vel_x > 0 {
        *vel_x -= 1;
    } else if *vel_x < 0 {
        *vel_x += 1;
    }
    *vel_y -= 1;
}

fn is_in_target(x: &i32, y: &i32, targ_x1: &i32, targ_x2: &i32, targ_y1: &i32, targ_y2: &i32) -> bool {
    if (targ_x1..=targ_x2).contains(&x) {
        if (targ_y1..=targ_y2).contains(&y) {
            return true;
        }
    }
    false
}

fn part1(input_file: String) {
    // let mut answer: usize = 0;

    let mut splitted: Vec<&str> = input_file.split(' ').collect();
    let targ_x_line = &splitted[2];
    let targ_y_line = &splitted[3];
    let targ_x_range = &targ_x_line[2..targ_x_line.len() - 1];
    let targ_y_range = &targ_y_line[2..targ_y_line.len() - 1];
    let targ_x1: i32 = targ_x_range.split("..").nth(0).unwrap().parse().unwrap();
    let targ_x2: i32 = targ_x_range.split("..").nth(1).unwrap().parse().unwrap();
    let targ_y1: i32 = targ_y_range.split("..").nth(0).unwrap().parse().unwrap();
    let targ_y2: i32 = targ_y_range.split("..").nth(1).unwrap().parse().unwrap();

    // dbg!(
    //     &targ_x_range,
    //     &targ_y_range,
    //     &targ_x1,
    //     &targ_x2,
    //     &targ_y1,
    //     &targ_y2
    // );

    let mut x: i32;
    let mut y: i32;
    let mut vel_x: i32;
    let mut vel_y: i32;

    let mut highest_y = 0;

    for i in -500..500 {
        for a in -500..500 {
            x = 0;
            y = 0;
            vel_x = i;
            vel_y = a;
            // println!("velx: {}, vely: {}", vel_x, vel_y);
            let mut high_y = 0;
            for _ in 0..1000 {
                iterate_step(&mut x, &mut y, &mut vel_x, &mut vel_y);
                if y > high_y {
                    high_y = y;
                }
                // dbg!(&high_y);
                // dbg!(&x, &y);
                if is_in_target(&x, &y, &targ_x1, &targ_x2, &targ_y1, &targ_y2) {
                    // dbg!("target");
                    if high_y > highest_y {
                        highest_y = high_y;
                    }
                }
            }
        }
    }

    print_answer(highest_y.try_into().unwrap());
}

fn part2(input_file: String) {
    // let mut answer: usize = 0;

    let mut splitted: Vec<&str> = input_file.split(' ').collect();
    let targ_x_line = &splitted[2];
    let targ_y_line = &splitted[3];
    let targ_x_range = &targ_x_line[2..targ_x_line.len() - 1];
    let targ_y_range = &targ_y_line[2..targ_y_line.len() - 1];
    let targ_x1: i32 = targ_x_range.split("..").nth(0).unwrap().parse().unwrap();
    let targ_x2: i32 = targ_x_range.split("..").nth(1).unwrap().parse().unwrap();
    let targ_y1: i32 = targ_y_range.split("..").nth(0).unwrap().parse().unwrap();
    let targ_y2: i32 = targ_y_range.split("..").nth(1).unwrap().parse().unwrap();

    // dbg!(
    //     &targ_x_range,
    //     &targ_y_range,
    //     &targ_x1,
    //     &targ_x2,
    //     &targ_y1,
    //     &targ_y2
    // );

    let mut x: i32;
    let mut y: i32;
    let mut vel_x: i32;
    let mut vel_y: i32;

    let mut how_many = 0;

    for i in -1000..1000 {
        for a in -1000..1000 {
            x = 0;
            y = 0;
            vel_x = i;
            vel_y = a;
            // println!("velx: {}, vely: {}", vel_x, vel_y);
            for _ in 0..1000 {
                iterate_step(&mut x, &mut y, &mut vel_x, &mut vel_y);
                // dbg!(&high_y);
                // dbg!(&x, &y);
                if is_in_target(&x, &y, &targ_x1, &targ_x2, &targ_y1, &targ_y2) {
                    // println!("velx: {}, vely: {}", vel_x, vel_y);
                    how_many += 1;
                    break;
                }
            }
        }
    }

    print_answer(how_many);
}

fn main() {
    let part = 2;

    let input_file = load_file("input.txt");

    if part == 1 {
        part1(input_file);
    } else if part == 2 {
        part2(input_file);
    }
}
