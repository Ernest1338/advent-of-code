use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct Game {
    id: usize,
    subsets: Vec<(usize, String)>,
}

impl Game {
    fn from(line: &str) -> Self {
        let split = line.split(":").collect::<Vec<&str>>();
        let id = split[0]
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        let mut subsets: Vec<(usize, String)> = Vec::new();
        for subset in split[1].split(";") {
            for s in subset.split(",").map(|v| v.trim()).collect::<Vec<&str>>() {
                let tmp = s.split_whitespace().collect::<Vec<&str>>();
                subsets.push((tmp[0].parse::<usize>().unwrap(), tmp[1].to_string()));
            }
        }
        Game { id, subsets }
    }
}

fn part1(file: &str) -> usize {
    let mut sum = 0;
    let games = file.lines().map(|l| Game::from(&l)).collect::<Vec<Game>>();
    for game in games {
        let mut possible = true;
        for subset in game.subsets {
            match subset.1.as_str() {
                "red" => {
                    if subset.0 > 12 {
                        possible = false;
                    }
                }
                "green" => {
                    if subset.0 > 13 {
                        possible = false;
                    }
                }
                "blue" => {
                    if subset.0 > 14 {
                        possible = false;
                    }
                }
                _ => (),
            }
        }
        if possible {
            sum += game.id;
        }
    }
    sum
}

fn part2(file: &str) -> usize {
    let mut sum = 0;
    let games = file.lines().map(|l| Game::from(&l)).collect::<Vec<Game>>();
    for game in games {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for subset in game.subsets {
            match subset.1.as_str() {
                "red" => {
                    if subset.0 > max_red {
                        max_red = subset.0;
                    }
                }
                "green" => {
                    if subset.0 > max_green {
                        max_green = subset.0;
                    }
                }
                "blue" => {
                    if subset.0 > max_blue {
                        max_blue = subset.0;
                    }
                }
                _ => (),
            }
        }
        sum += max_red * max_green * max_blue;
    }
    sum
}

fn main() {
    let file = read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}
