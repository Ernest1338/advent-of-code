use std::fs::read_to_string;

fn part1(input_file: String) {
    dbg!(input_file);
}

fn part2(input_file: String) {
    dbg!(input_file);
}

fn main() {
    let part: i32 = 1;

    let input_file: String = read_to_string(format!("input{}.txt", part)).unwrap();

    if part == 1 { part1(input_file); }
    else if part == 2 { part2(input_file); }
}
