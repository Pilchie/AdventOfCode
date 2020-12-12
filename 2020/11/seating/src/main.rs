use std::env;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = env::args().collect();
    let input = fs::read_to_string(&args[1])?; 
    let layout = Layout::parse(&input);
    println!("There are {} occupied seats at steady state", layout.find_steady_state().occupied());

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    Floor,
    Empty,
    Occupied,
}

pub struct Layout {
    map: Vec<Vec<Position>>,
}

impl Layout {
    pub fn parse(input: &str) -> Layout {
        let mut map = Vec::new();
        for y in input.lines() {
            let mut row = Vec::new();
            for x in y.chars() {
                row.push(match x {
                    '.' => Position::Floor,
                    'L' => Position::Empty,
                    '#' => Position::Occupied,
                    _ => panic!(),
                });
            }
            map.push(row);
        }
        Layout { map: map }
    }

    fn apply_round(&self) -> Layout {
        let mut map = Vec::new();
        for y in 0..self.map.len() {
            let mut row = Vec::new();
            for x in 0..self.map[y].len() {
                let cur = &self.map[y][x];
                let adjacent_occupied = self.adjacent_occupied(y as isize, x as isize);
                let new = match cur {
                    Position::Empty => match adjacent_occupied {
                        0 => Position::Occupied,
                        _ => *cur,
                    },
                    Position::Occupied => match adjacent_occupied >= 4 {
                        true => Position::Empty,
                        _ => *cur,
                    },
                    _ => *cur,
                };
                // println!("Looked at (y, x): ({},{}). Cur is '{:?}', adjacent_occupied is '{}', new is '{:?}'",
                //     y, x, cur, adjacent_occupied, new);

                row.push(new);
            }
            map.push(row);
        }
        Layout { map: map }
    }

    fn adjacent_occupied(&self, y: isize, x: isize) -> usize {
        let mut occupied = 0;

        for i in 0..3 {
            occupied += self.one_if_occupied(y - 1, x + i - 1);
            occupied += self.one_if_occupied(y + 1, x + i - 1);
        }
        occupied += self.one_if_occupied(y, x - 1);
        occupied += self.one_if_occupied(y, x + 1);

        occupied
    }

    fn one_if_occupied(&self, y: isize, x: isize) -> usize {
        match self.get_with_bounds(y, x) {
            Position::Occupied => 1,
            _ => 0,
        }
    }

    fn get_with_bounds(&self, y: isize, x: isize) -> Position {
        if y < 0 || y as usize >= self.map.len() {
            return Position::Floor;
        }

        if x < 0 || x as usize >= self.map[y as usize].len() {
            return Position::Floor;
        }

        return self.map[y as usize][x as usize];
    }

    pub fn find_steady_state(&self) -> Layout {
        let mut prev = self.clone();
        loop {
            println!("Current layout is:");
            println!("{:?}", prev);
            let cur = prev.apply_round();

            if prev == cur {
                return cur;
            }

            prev = cur;
        }
    }

    pub fn occupied(&self) -> usize {
        let mut count = 0;
        for x in &self.map {
            for y in x {
                count += match y {
                    Position::Occupied => 1,
                    _ => 0,
                }
            }
        }

        count
    }
}

impl PartialEq for Layout {
    fn eq(&self, other: &Self) -> bool {
        if self.map.len() != other.map.len() {
            return false;
        }

        for i in 0..self.map.len() {
            if self.map[i].len() != other.map[i].len() {
                return false;
            }
            for j in 0..self.map[i].len() {
                if self.map[i][j] != other.map[i][j] {
                    return false;
                }
            }
        }

        true
    }
}

impl Clone for Layout {
    fn clone(&self) -> Self {
        let mut new = Vec::new();
        for r in &self.map {
            new.push(r.clone());
        }

        Layout { map: new }
    }
}

impl std::fmt::Debug for Layout {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for row in &self.map {
            for col in row {
                formatter.write_str(match col {
                    Position::Empty => "L",
                    Position::Occupied => "#",
                    Position::Floor => ".",
                })?;
            }
            formatter.write_str("\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let layout = Layout::parse(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        );

        let res = layout.find_steady_state().occupied();
        assert_eq!(37, res);
    }
}
