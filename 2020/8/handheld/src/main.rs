use std::collections::HashSet;
use std::env;
use std::fs;
use std::num::ParseIntError;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])?;
    if let Ok(program) = Program::parse(&input) {
        for x in 0..program.len() {
            let mut modified_program = program.to_vec();
            let new_instr = match program[x] {
                Instruction::Nop(x) => Instruction::Jmp(x),
                Instruction::Jmp(x) => Instruction::Nop(x),
                Instruction::Acc(x) => Instruction::Acc(x),
            };
            modified_program[x] = new_instr;
            let (acc, pc) = Emulator::run(&modified_program);
            if pc == program.len() {
                println!("Completed the program with acc value {}", acc);
                break;
            }

        }
    } else {
        println!("Didn't run");
    }

    Ok(())
}

pub struct Program {}
impl Program {
    pub fn parse(lines: &str) -> Result<Vec<Instruction>, Error> {
        let mut program = Vec::new();
        for line in lines.lines() {
            let instr = Instruction::parse(line)?;
            program.push(instr);
        }

        Ok(program)
    }
}

pub struct Emulator {}

impl Emulator {
    pub fn run(program: &[Instruction]) -> (isize, usize) {
        let mut seen = HashSet::new();
        let mut pc = 0;
        let mut acc = 0;

        while !seen.contains(&pc) && pc < program.len() as isize {
            seen.insert(pc);
            let instr = &program[pc as usize];
            let (a, p) = instr.exec();
            acc += a;
            pc += p;
        }

        (acc, pc as usize)
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
#[derive(Clone)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Instruction {
    pub fn parse(input: &str) -> Result<Instruction, Error> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let val = parts[1].parse::<isize>()?;

        match parts[0] {
            "nop" => Ok(Instruction::Nop(0)),
            "acc" => Ok(Instruction::Acc(val)),
            "jmp" => Ok(Instruction::Jmp(val)),
            _ => Err(Error::UnexpectedInstruction),
        }
    }

    pub fn exec(&self) -> (isize, isize) {
        match self {
            Instruction::Nop(_) => (0, 1),
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
        let input = "nop +0  | 1
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        if let Ok(program) = Program::parse(input) {
            let (acc, _) = Emulator::run(&program);
            assert_eq!(5, acc);
        }
    }
}
