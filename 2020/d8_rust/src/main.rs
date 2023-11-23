#[derive(Debug, Clone)]
struct Instruction {
    opcode: String,
    value: isize,
}

#[derive(Debug, Clone)]
struct Cpu {
    acc: isize,
    pc: isize,
    instructions: Vec<Instruction>,
    visited: Vec<isize>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            acc: 0,
            pc: 0,
            instructions: Vec::new(),
            visited: Vec::new(),
        }
    }

    fn load_instructions(&mut self, data: &str) {
        for line in data.lines() {
            let split: Vec<&str> = line.split_whitespace().collect();
            let opcode = split[0].to_string();
            let value = split[1].parse::<isize>().unwrap();
            self.instructions.push(Instruction { opcode, value });
        }
    }

    fn execute_until_loop(&mut self) {
        while self.pc != self.instructions.len() as isize {
            if self.visited.contains(&self.pc) {
                break;
            }
            self.visited.push(self.pc);
            let instr = &self.instructions[self.pc as usize];
            match instr.opcode.as_str() {
                "acc" => {
                    self.acc += instr.value;
                }
                "jmp" => {
                    self.pc += instr.value;
                    continue;
                }
                // Anything else or NOP -> nothing
                _ => (),
            }
            self.pc += 1;
        }
    }

    fn does_halt(&mut self) -> bool {
        while self.pc != self.instructions.len() as isize {
            if self.visited.contains(&self.pc) {
                return true;
            }
            self.visited.push(self.pc);
            let instr = &self.instructions[self.pc as usize];
            match instr.opcode.as_str() {
                "acc" => {
                    self.acc += instr.value;
                }
                "jmp" => {
                    self.pc += instr.value;
                    continue;
                }
                // Anything else or NOP -> nothing
                _ => (),
            }
            self.pc += 1;
        }
        false
    }

    #[allow(dead_code)]
    fn execute(&mut self) {
        while self.pc != self.instructions.len() as isize {
            let instr = &self.instructions[self.pc as usize];
            println!("executing instr nr {}: {instr:?}", self.pc);
            match instr.opcode.as_str() {
                "acc" => {
                    self.acc += instr.value;
                }
                "jmp" => {
                    self.pc += instr.value;
                    continue;
                }
                // Anything else or NOP -> nothing
                _ => (),
            }
            self.pc += 1;
        }
    }
}

fn part1(data: &str) -> isize {
    let mut cpu = Cpu::new();
    cpu.load_instructions(data);
    cpu.execute_until_loop();

    cpu.acc
}

fn part2(data: &str) -> isize {
    let mut cpu = Cpu::new();
    cpu.load_instructions(data);
    let instructions = cpu.instructions;

    // NOTE: warning - hacks ahead
    for (i, instr) in instructions.iter().enumerate() {
        if instr.opcode == "nop" || instr.opcode == "jmp" {
            let mut tmp_cpu = Cpu::new();
            let mut tmp_data = instructions.clone();
            if instr.opcode == "nop" {
                tmp_data[i] = Instruction {
                    opcode: "jmp".to_string(),
                    value: instr.value,
                };
            } else if instr.opcode == "jmp" {
                tmp_data[i] = Instruction {
                    opcode: "nop".to_string(),
                    value: instr.value,
                };
            }
            let mut tmp_data_string = String::new();
            for tmp_instr in tmp_data {
                tmp_data_string.push_str(&format!("{} {}\n", tmp_instr.opcode, tmp_instr.value));
            }
            tmp_cpu.load_instructions(&tmp_data_string);
            if !tmp_cpu.does_halt() {
                return tmp_cpu.acc;
            }
        }
    }

    0
}

fn main() {
    let use_input = true;

    let data = std::fs::read_to_string(if use_input {
        "input.txt"
    } else {
        "example.txt"
    })
    .unwrap();

    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}
