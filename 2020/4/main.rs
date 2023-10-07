use std::fs::read_to_string;

fn main() {
    let data = read_to_string("input.txt").unwrap();
    let mut result = 0;
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    'outer: for passports in data.split("\n\n") {
        for c in &required {
            if !passports.contains(c) {
                continue 'outer;
            }
        }
        for passport in passports.split_whitespace() {
            let fields = passport.split(':').collect::<Vec<&str>>();
            match fields[0] {
                "byr" => {
                    let num = fields[1].parse::<usize>().unwrap();
                    if !(1920..=2002).contains(&num) {
                        continue 'outer;
                    }
                }
                "iyr" => {
                    let num = fields[1].parse::<usize>().unwrap();
                    if !(2010..=2020).contains(&num) {
                        continue 'outer;
                    }
                }
                "eyr" => {
                    let num = fields[1].parse::<usize>().unwrap();
                    if !(2020..=2030).contains(&num) {
                        continue 'outer;
                    }
                }
                "hgt" => {
                    if fields[1].ends_with("cm") {
                        let num: usize = fields[1].replace("cm", "").parse().unwrap();
                        if !(150..=193).contains(&num) {
                            continue 'outer;
                        }
                    } else if fields[1].ends_with("in") {
                        let num: usize = fields[1].replace("in", "").parse().unwrap();
                        if !(59..=76).contains(&num) {
                            continue 'outer;
                        }
                    } else {
                        continue 'outer;
                    }
                }
                "hcl" => {
                    if !(fields[1].starts_with('#') && fields[1].len() == 7) {
                        continue 'outer;
                    }
                }
                "ecl" => {
                    let options = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    if !options.contains(&fields[1]) {
                        continue 'outer;
                    }
                }
                "pid" => {
                    if !(fields[1].parse::<usize>().is_ok() && fields[1].len() == 9) {
                        continue 'outer;
                    }
                }
                _ => (),
            }
        }
        result += 1;
    }

    println!("{result}");
}
