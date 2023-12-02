use std::fs::read_to_string;

fn replace_digits(line: &str) -> String {
    line.replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
}

fn part(file: &str, part2: bool) -> usize {
    let mut sum = 0;
    for line in file.lines() {
        let line = if part2 {
            replace_digits(&line)
        } else {
            line.to_string()
        };
        let mut tmp = Vec::new();
        for chr in line.chars() {
            if chr.is_digit(10) {
                tmp.push(chr);
            }
        }
        sum += format!("{}{}", tmp.first().unwrap(), tmp.last().unwrap())
            .parse::<usize>()
            .unwrap();
    }
    sum
}

fn main() {
    let file = read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part(&file, false));
    println!("Part 2: {}", part(&file, true));
}
