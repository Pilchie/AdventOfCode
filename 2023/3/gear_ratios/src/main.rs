use std::io::BufRead;

struct Entry {
    x: usize,
    y: usize,
    length: usize,
}

impl Entry {
    fn is_symbol(b: u8) -> bool {
        return match b {
            b'0' => false,
            b'1' => false,
            b'2' => false,
            b'3' => false,
            b'4' => false,
            b'5' => false,
            b'6' => false,
            b'7' => false,
            b'8' => false,
            b'9' => false,
            b'.' => false,
            _ => true,
        };
    }

    fn check_row(y: usize, startx: usize, endx: usize, schematic: &Vec<Vec<u8>>) -> bool {
        for x in startx..endx {
            if x > 0 && x < schematic[y].len() && Self::is_symbol(schematic[y][x]) {
                return true;
            }
        }

        false
    }

    fn is_part_number(&self, schematic: &Vec<Vec<u8>>) -> bool {
        let startx = match self.x {
            0 => 0,
            _ => self.x - 1,
        };
        if self.y > 0 {
            if Self::check_row(self.y - 1, startx, self.x + self.length + 2, schematic) {
                return true;
            }
        }

        if self.x > 0 && Self::is_symbol(schematic[self.y][self.x - 1]) {
            return true;
        }

        if self.x + self.length < schematic[self.y].len() - 1  
            && Self::is_symbol(schematic[self.y][self.x + self.length + 2])
        {
            return true;
        }

        if self.y < schematic.len() - 1 {
            if Self::check_row(self.y + 1, startx, self.x + self.length + 2, schematic) {
                return true;
            }
        }

        false
    }

    fn value(&self, schematic: &Vec<Vec<u8>>) -> i32 {
        let mut val = 0;

        for i in 0..self.length {
            let b = schematic[self.y][self.x + i];
            val = val * 10 + (b - b'0') as i32;
        }

        val
    }
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let name = match args.len() {
        2 => &args[1],
        _ => "/workspaces/AdventOfCode/2023/3/gear_ratios/test.txt",
    };
    let reader = std::io::BufReader::new(std::fs::File::open(name)?);

    let mut schematic = Vec::new();
    for l in reader.lines() {
        let line = l?;
        let bytes: Vec<_> = line.bytes().collect();
        schematic.push(bytes);
    }

    let mut entries = Vec::new();
    for y in 0..schematic.len() {
        let mut in_num = false;
        let mut len = 0;
        for x in 0..schematic[y].len() {
            let b = schematic[y][x];
            if in_num {
                if b < b'0' || b > b'9' {
                    entries.push(Entry {
                        y: y,
                        x: x - len - 1,
                        length: len + 1,
                    });
                    in_num = false;
                    len = 0;
                } else {
                    len += 1;
                }
            } else {
                if b >= b'0' && b <= b'9' {
                    in_num = true;
                }
            }
        }
    }

    let sum: i32 = entries
        .iter()
        .filter(|e| e.is_part_number(&schematic))
        .map(|e| e.value(&schematic))
        .sum();

    for e in &entries {
        println!(
            "Entry at {},{} is '{}', value '{}",
            e.y,
            e.x,
            e.is_part_number(&schematic),
            e.value(&schematic)
        );
    }

    println!("The sum of the part numbers is {}", sum);
    Ok(())
}
