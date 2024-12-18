use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut computer = Computer::parse(&contents);
    let program = parse_program(&contents);

    let outputs_p1 = computer.run(&program);
    dump(&outputs_p1);

    println!("Part two");
    print!("Program is ");
    dump(&program);
    let mut min = 0;
    for i in 1..program.len()+1 {
        min = min << 3;
        let output_to_match = &program[program.len() - i..];
        for j in 0..16 {
            print!("  trying min {} to match {:?}", min + j, output_to_match);
            computer.reset_to(min + j);
            let output = computer.run(&program);
            println!(" output is {:?}", output);
            if output == output_to_match {
                min = min + j;
                println!("Min is now {}, output is {:?}", min, output);
                break;
            }
        }
    }

    println!("Final min is {}", min);
}

fn dump(outputs: &[u8]) {
    for o in outputs {
        print!("{},", o);
    }
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
    functions: Vec<Box<dyn Fn(u8, &mut Computer, &mut Vec<u8>) -> Option<usize>>>,
}

impl Instructions {
    fn new() -> Self {
        let mut functions: Vec<Box<dyn Fn(u8, &mut Computer, &mut Vec<u8>) -> Option<usize>>> =
            Vec::new();
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

    fn divide(operand: u8, computer: &Computer) -> i64 {
        let numerator = computer.registers[0];
        let two: i64 = 2;
        let denominator = two.pow(computer.combo(operand).try_into().unwrap());
        let res = numerator / denominator;
        //print!("{} DIV {} = {} ", numerator, denominator, res);
        res
    }

    fn adv(operand: u8, computer: &mut Computer, _: &mut Vec<u8>) -> Option<usize> {
        computer.registers[0] = Self::divide(operand, computer);
        //println!("Stored in A");
        None
    }

    fn bxl(operand: u8, computer: &mut Computer, _: &mut Vec<u8>) -> Option<usize> {
        let b = computer.registers[1];
        let res = b ^ operand as i64;
        computer.registers[1] = res;
        //println!("{} XOR {} = {}, Stored in B", b, operand, res);
        None
    }

    fn bst(operand: u8, computer: &mut Computer, _: &mut Vec<u8>) -> Option<usize> {
        let val = computer.combo(operand);
        let res = val % 8;
        computer.registers[1] = res;
        //println!("{} MOD 8 = {}, Stored in B", val, res);
        None
    }

    fn jnz(operand: u8, computer: &mut Computer, _: &mut Vec<u8>) -> Option<usize> {
        if computer.registers[0] == 0 {
            return None;
        }

        Some(operand as usize)
    }

    fn bxc(_operand: u8, computer: &mut Computer, _: &mut Vec<u8>) -> Option<usize> {
        let b = computer.registers[1];
        let c = computer.registers[2];
        let res = b ^ c;
        //println!("{} XOR {} = {} Stored in B", b, c, res);
        computer.registers[1] = res;
        None
    }

    fn out(operand: u8, computer: &mut Computer, outputs: &mut Vec<u8>) -> Option<usize> {
        let res = (computer.combo(operand) % 8) as u8;
        outputs.push(res);
        //println!("OUT {}", res);
        None
    }

    fn bdv(operand: u8, computer: &mut Computer, _: &mut Vec<u8>) -> Option<usize> {
        computer.registers[1] = Self::divide(operand, computer);
        //println!( "Stored in B");
        None
    }

    fn cdv(operand: u8, computer: &mut Computer, _: &mut Vec<u8>) -> Option<usize> {
        computer.registers[2] = Self::divide(operand, computer);
        //println!( "Stored in C");
        None
    }
}

struct Computer {
    registers: Vec<i64>,
}

impl Computer {
    fn parse(input: &str) -> Self {
        let mut registers = Vec::new();
        for line in input.lines() {
            if line.starts_with("Register ") {
                let (_, s) = line.split_once(": ").unwrap();
                registers.push(s.parse::<i64>().unwrap());
            }
        }
        Self { registers }
    }

    fn run(&mut self, program: &[u8]) -> Vec<u8> {
        let instructions = Instructions::new();
        let mut output = Vec::new();
        let mut ip: usize = 0;
        while ip < program.len() {
            let i = program[ip];
            let o = program[ip + 1];
            match instructions.functions[i as usize](o, self, &mut output) {
                Some(newip) => ip = newip,
                None => ip += 2,
            };
        }
        output
    }

    fn combo(&self, val: u8) -> i64 {
        if val <= 3 {
            return val as i64;
        } else if val <= 6 {
            return self.registers[(val - 4) as usize];
        } else {
            unreachable!();
        }
    }

    fn reset_to(&mut self, reg_a: i64) {
        self.registers[0] = reg_a;
        self.registers[1] = 0;
        self.registers[2] = 0;
    }
}
