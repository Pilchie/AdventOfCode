use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let args : Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let program = Program::parse(&input)?;
    let mut memory = HashMap::new();
    program.run(&mut memory);
    println!("The answer is: {}", memory.iter().fold(0, |acc, (_, v)| acc+v));

    Ok(())
}

#[derive(Debug)]
pub enum Error {
    Parse(std::num::ParseIntError),
    IO(std::io::Error),
}

impl From<std::num::ParseIntError> for Error {
    fn from(pie: std::num::ParseIntError) -> Self {
        Error::Parse(pie)
    }
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Error::IO(ioe)
    }
}

pub enum Instruction {
    Mask(String),
    Mem(usize, usize),
}

pub struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn parse(input: &str) -> Result<Self, Error> {
        let mut instrs = Vec::new();
        for line in input.lines() {
            if line.starts_with("mask = ") {
                let mask = &line["mask = ".len()..];
                instrs.push(Instruction::Mask(String::from(mask)));
                println!("parsed Instruction::Mask({})", mask);
            } else if line.starts_with("mem[") {
                let mut addr = 0;
                for (i, c) in line.char_indices() {
                    if c == ']' {
                        addr = line[4..i].parse()?;
                    } else if c == '=' {
                        let val = line[i+2..].parse()?;
                        println!("parsed Instruction::Mem({}, {})", addr, val);
                        instrs.push(Instruction::Mem(addr, val));
                        break;
                    }
                }
            } else {
                panic!("Unknown instruction {}", line);
            }
        }

        Ok(Self {
            instructions: instrs,
        })
    }

    pub fn run(&self, memory: &mut HashMap<usize, usize>) {
        let mut cur_mask = "";
        let mut bitmask_or : usize = 0b0;
        let mut bitmask_and : usize = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111;
        for instruction in &self.instructions {
            match instruction {
                Instruction::Mask(mask) => {
                    cur_mask = mask;
                    bitmask_or = 0b0;
                    bitmask_and = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111;
                    for (i, c) in mask.char_indices() {
                        if c == '1' {
                            bitmask_or += 1<<(35-i);
                        } else if c == '0' {
                            bitmask_and &= !(1<<(35-i));
                        }
                    }
                    // println!("Setting bitmask_or to '{:b}' and bitmask_and to '{:b}'", bitmask_or, bitmask_and);

                },
                Instruction::Mem(addr, val) => {
                    let res = (val & bitmask_and) | bitmask_or;
                    println!("Setting memory at {}", addr);
                    println!(" value:  {:>36b} - {}", val, val);
                    println!(" mask:   {}", cur_mask);
                    println!(" result: {:>36b} - {}", res, res);
                    memory.insert(*addr, res);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<(), Error> {
        let program = Program::parse("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0")?;

        let mut memory = HashMap::new();
        program.run(&mut memory);

        assert_eq!(165, memory.iter().fold(0, |acc, (_, v)| acc+v));
        Ok(())
    }
}