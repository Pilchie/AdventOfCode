use std::collections::HashMap;
use std::convert::TryInto;
use std::hash::{Hash, Hasher};
use termion::{clear, cursor};

mod intcode;

struct Position {
    x: u16,
    y: u16,
}

impl PartialEq for Position {
    fn eq(&self, rhs: &Position) -> bool {
        return self.x == rhs.x && self.y == rhs.y;
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, h: &mut H) {
        h.write_u16(self.x);
        h.write_u16(self.y);
    }
}
struct TerminalInputOutputSystem {
    places: HashMap<Position, char>,
    value1: Option<i64>,
    value2: Option<i64>,
}

impl intcode::InputOutputSystem for TerminalInputOutputSystem {
    fn print_output(&mut self, value: i64) {
        match self.value1 {
            None => self.value1 = Some(value),
            Some(v1) => {
                match self.value2 {
                    None => self.value2 = Some(value),
                    Some(v2) => {
                        let x = (v1 + 1).try_into().unwrap();
                        let y = (v2 + 1).try_into().unwrap();
                        let tile = TerminalInputOutputSystem::map_tile(value);
                        print!("{}{}", cursor::Goto(x, y), tile);
                        self.value1 = None;
                        self.value2 = None;
                        self.places.insert(Position{ x, y }, tile);
                    }
                }
            }
        }
    }

    fn get_input(&mut self) -> i64 {
        0
    }
}

impl TerminalInputOutputSystem {
    fn new() -> TerminalInputOutputSystem {
        print!("{}", clear::All);
        TerminalInputOutputSystem {
            places: HashMap::new(),
            value1: None,
            value2: None,
        }
    }

    fn map_tile(v: i64) -> char {
        match v {
            0 => ' ',
            1 => '█',
            2 => '#',
            3 => '-',
            4 => '⚫',
            _ => panic!("unexpected tile")
        }
    }
}

pub fn play_game(program: &mut Vec<i64>) {
    let mut io = TerminalInputOutputSystem::new();
    let mut computer = intcode::IntCode::new(&mut io);
    computer.run_to_completion(program);
    println!();

    let count_blocks = io.places.values().filter(|&c| c == &'#').count();
    println!("Count of blocks: {}", count_blocks)
}