use std::collections::HashSet;
use std::env;
use std::fs;
use std::num::ParseIntError;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    if let Ok(x) = Emulator::run(&input) {
        println!("Acc is {} before looping", x);
    } else {
        println!("Didn't run");
    }

    Ok(())
}

pub struct Emulator {}

impl Emulator {
    pub fn run(lines: &str) -> Result<isize, Error> {
        let mut program = Vec::new();
        for line in lines.lines() {
            let instr = Instruction::parse(line)?;
            program.push(instr);
        }

        let mut seen = HashSet::new();
        let mut pc: isize = 0;
        let mut acc = 0;

        while !seen.contains(&pc) {
            seen.insert(pc);
            let instr = &program[pc as usize];
            let (a, p) = instr.exec();
            acc += a;
            pc += p;
        }

        Ok(acc)
    }
}

pub enum Error {
    Parse(ParseIntError),
    UnexpectedInstruction,
}

impl From<ParseIntError> for Error {
    fn from(pie: ParseIntError) -> Error {
        Error::Parse(pie)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop,
}

impl Instruction {
    pub fn parse(input: &str) -> Result<Instruction, Error> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let val = parts[1].parse::<isize>()?;

        match parts[0] {
            "nop" => Ok(Instruction::Nop),
            "acc" => Ok(Instruction::Acc(val)),
            "jmp" => Ok(Instruction::Jmp(val)),
            _ => Err(Error::UnexpectedInstruction),
        }
    }

    pub fn exec(&self) -> (isize, isize) {
        match self {
            Instruction::Nop => (0, 1),
            Instruction::Acc(x) => (*x, 1),
            Instruction::Jmp(x) => (0, *x),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn first() {
        let program = "nop +0  | 1
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        if let Ok(x) = Emulator::run(program) {
            assert_eq!(5, x);
        }
    }
}
