use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut computer = Computer::parse(&contents);
    let program = parse_program(&contents);

    computer.run(&program);
    println!();
}

fn parse_program(input: &str) -> Vec<u8> {
    let mut res = Vec::new();
    let mut start = false;
    for line in input.lines() {
        if start {
            let (_, p) = line.split_once(": ").unwrap();
            for istr in p.split(",") {
                res.push(istr.parse::<u8>().unwrap());
            }
        } else if line.is_empty() {
            start = true;
        }
    }
    res
}

struct Instructions {
    functions: Vec<Box<dyn Fn(u8, &mut Computer) -> Option<usize>>>,
}

impl Instructions {
    fn new() -> Self {
        let mut functions: Vec<Box<dyn Fn(u8, &mut Computer) -> Option<usize>>> = Vec::new();
        functions.push(Box::new(Self::adv));
        functions.push(Box::new(Self::bxl));
        functions.push(Box::new(Self::bst));
        functions.push(Box::new(Self::jnz));
        functions.push(Box::new(Self::bxc));
        functions.push(Box::new(Self::out));
        functions.push(Box::new(Self::bdv));
        functions.push(Box::new(Self::cdv));
        Self { functions }
    }

    fn divide(operand: u8, computer: &Computer) -> u32 {
        let numerator = computer.registers[0];
        let two: u32 = 2;
        let denominator = two.pow(computer.combo(operand));
        numerator / denominator
    }

    fn adv(operand: u8, computer: &mut Computer) -> Option<usize> {
        computer.registers[0] = Self::divide(operand, computer);
        None
    }

    fn bxl(operand: u8, computer: &mut Computer) -> Option<usize> {
        computer.registers[1] = computer.registers[1] ^ operand as u32;
        None
    }

    fn bst(operand: u8, computer: &mut Computer) -> Option<usize> {
        computer.registers[1] = computer.combo(operand) % 8;
        None
    }

    fn jnz(operand: u8, computer: &mut Computer) -> Option<usize> {
        if computer.registers[0] == 0 {
            return None;
        }

        Some(operand as usize)
    }

    fn bxc(_operand: u8, computer: &mut Computer) -> Option<usize> {
        computer.registers[1] = computer.registers[1] ^ computer.registers[2];
        None
    }

    fn out(operand: u8, computer: &mut Computer) -> Option<usize> {
        print!("{},", computer.combo(operand) % 8);
        None 
    }

    fn bdv(operand: u8, computer: &mut Computer) -> Option<usize> {
        computer.registers[1] = Self::divide(operand, computer);
        None
    }

    fn cdv(operand: u8, computer: &mut Computer) -> Option<usize> {
        computer.registers[2] = Self::divide(operand, computer);
        None
    }
}

struct Computer {
    registers: Vec<u32>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let mut registers = Vec::new();
        for line in input.lines() {
            if line.starts_with("Register ") {
                let (_, s) = line.split_once(": ").unwrap();
                registers.push(s.parse::<u32>().unwrap());
            }
        }
        Self { registers }
    }

    fn run(&mut self, program: &[u8]) {
        let instructions = Instructions::new();

        let mut ip: usize = 0;
        while ip < program.len() {
            let i = program[ip];
            let o = program[ip + 1];
            match instructions.functions[i as usize](o, self) {
                Some(newip) => ip = newip,
                None => ip += 2,
            };
        }
    }

    fn combo(&self, val: u8) -> u32 {
        if val <= 3 {
            return val as u32;
        } else if val <= 6 {
            return self.registers[(val - 4) as usize];
        } else {
            unreachable!();
        }
    }
}
