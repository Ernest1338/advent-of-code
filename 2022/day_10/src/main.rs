#![allow(dead_code, unused_variables, unused_mut)]

use aoc_std::*;

const INTERESTING_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220];

#[derive(Debug, Clone)]
struct Cpu {
    cycle: i32,
    reg_x: i32,
    signal_strengths: Vec<(i32, i32)>,
    display: [[char; 40]; 6],
}

impl Cpu {
    fn new() -> Cpu {
        let mut cpu = Cpu {
            cycle: 1,
            reg_x: 1,
            signal_strengths: Vec::new(),
            display: [['.'; 40]; 6],
        };
        cpu.display[0][0] = '#'; // hack
        cpu
    }

    fn execute(&mut self, instruction: &str) {
        let instruction_split: Vec<&str> = instruction.split(' ').collect();
        let opcode = instruction_split[0];
        let mut value: i32 = 0;
        if instruction_split.len() > 1 {
            value = instruction_split[1].parse::<i32>().unwrap();
        }
        match opcode {
            "addx" => {
                self.increment_cycle();
                self.reg_x += value;
                self.increment_cycle();
            }
            "noop" => {
                self.increment_cycle();
            }
            _ => {}
        }
    }

    fn increment_cycle(&mut self) {
        self.update_display();
        self.cycle += 1;
        if INTERESTING_CYCLES.contains(&self.cycle) {
            self.signal_strengths.push((self.cycle, self.reg_x));
        }
    }

    fn update_display(&mut self) {
        if (self.reg_x - 1..=self.reg_x + 1).contains(&(&self.cycle % 40)) {
            self.display[(self.cycle as f32 / 40.0).floor() as usize][(self.cycle % 40) as usize] =
                '#';
        }
    }

    fn render_display(&self) {
        for x in self.display {
            for y in x {
                print!("{y}");
            }
            println!();
        }
    }
}

fn part1(input_file: String) -> usize {
    let mut cpu = Cpu::new();
    input_file
        .lines()
        .for_each(|instruction| cpu.execute(instruction));
    cpu.signal_strengths
        .iter()
        .map(|signal| (signal.0 * signal.1) as usize)
        .sum::<usize>()
}

fn part2(input_file: String) {
    let mut cpu = Cpu::new();
    input_file
        .lines()
        .for_each(|instruction| cpu.execute(instruction));
    cpu.render_display();
}

fn main() {
    let part = 2;

    let input_file = load_file("input.txt");

    if part == 1 {
        print_answer(part1(input_file));
    } else if part == 2 {
        part2(input_file);
    }
}
