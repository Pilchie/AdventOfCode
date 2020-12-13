use std::cmp::min;
use std::env;
use std::fs;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = env::args().collect();
    let input = fs::read_to_string(&args[1])?; 
    let layout = Layout::parse(&input);
    println!("There are {} occupied seats at steady state", layout.find_steady_state(false).occupied());

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

    fn apply_round(&self, adjacent: bool) -> Layout {
        let mut map = Vec::new();
        for y in 0..self.map.len() {
            let mut row = Vec::new();
            for x in 0..self.map[y].len() {
                let new = match adjacent {
                    true => self.transform_adjacent(y, x),
                    false => self.transform_visible(y, x),
                };
                row.push(new);
            }
            map.push(row);
        }
        Layout { map: map }
    }

    fn transform_adjacent(&self, y: usize, x: usize) -> Position {
        let cur = self.map[y][x];
        let adjacent_occupied = self.adjacent_occupied(y as isize, x as isize);
        match cur {
            Position::Empty => match adjacent_occupied {
                0 => Position::Occupied,
                _ => cur,
            },
            Position::Occupied => match adjacent_occupied >= 4 {
                true => Position::Empty,
                _ => cur,
            },
            _ => cur,
        }
    }

    fn transform_visible(&self, y: usize, x: usize) -> Position {
        let cur = self.map[y][x];
        let visible_occupied = self.visible_occupied(y, x);
        match cur {
            Position::Empty => match visible_occupied {
                0 => Position::Occupied,
                _ => cur,
            },
            Position::Occupied => match visible_occupied >= 5 {
                true => Position::Empty,
                _ => cur,
            },
            _ => cur,
        }
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

    fn visible_occupied(&self, y: usize, x: usize) -> usize {
        // println!("Examining ({},{})", y, x);

        let mut occupied = 0;
        for dx in 1..x+1 {
            let seat = self.map[y][x-dx];
            if seat == Position::Occupied {
                // print!("left ");
                occupied += 1;
                break;
            } else if seat == Position::Empty {
                break;
            }
        }

        for dx in 1..self.map[y].len() - x {
            let seat = self.map[y][x+dx];
            if seat == Position::Occupied {
                // print!("right ");
                occupied += 1;
                break;
            } else if seat == Position::Empty {
                break;
            }
        }

        for dy in 1..y+1 {
            let seat = self.map[y-dy][x];
            if seat == Position::Occupied {
                // print!("up ");
                occupied += 1;
                break;
            } else if seat == Position::Empty {
                break;
            }
        }

        for dy in 1..self.map.len() - y {
            let seat = self.map[y+dy][x];
            if seat == Position::Occupied {
                // print!("down ");
                occupied += 1;
                break;
            } else if seat == Position::Empty {
                break;
            }
        }

        let mut m = min(x, y);
        for d in 1..m+1 {
            let seat = self.map[y-d][x-d];
            // println!("  Looking up/left at ({},{}), found {:?}", y-d, x-d, seat);
            if seat == Position::Occupied {
                // print!("up/left ");
                occupied += 1;
                break;
            } else if seat == Position::Empty {
                break;
            }
        }

        m = min(x + 1, self.map.len() - y);
        for d in 1..m {
            let seat = self.map[y+d][x-d];
            if seat == Position::Occupied {
                // print!("down/left ");
                occupied += 1;
                break;
            } else if seat == Position::Empty {
                break;
            }
        }

        m = min(self.map[y].len() - x, y + 1);
        for d in 1..m {
            let seat = self.map[y-d][x+d];
            // println!("  Checking ({}, {}), seat is {:?}", y-d, x+d, seat);
            if seat == Position::Occupied {
                // print!("up/right ");
                occupied += 1;
                break;
            } else if seat == Position::Empty {
                break;
            }
        }

        m = min(self.map[y].len() - x, self.map.len() - y);
        for d in 1..m {
            let seat = self.map[y+d][x+d];
            if seat == Position::Occupied {
                // print!("down/right ");
                occupied += 1;
                break;
            } else if seat == Position::Empty {
                break;
            }
        }

        // println!("Found {} occupied", occupied);
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

    pub fn find_steady_state(&self, adjacent: bool) -> Layout {
        let mut prev = self.clone();
        loop {
            println!("Current layout is:");
            println!("{:?}", prev);
            let cur = prev.apply_round(adjacent);

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

        let res = layout.find_steady_state(true).occupied();
        assert_eq!(37, res);
    }

    #[test]
    fn part2() {
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

        let res = layout.find_steady_state(false).occupied();
        assert_eq!(26, res);
    }

    #[test]
    pub fn visible_occupied1() {
        let layout = Layout::parse(".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....");

        assert_eq!(Position::Empty, layout.map[4][3]);
        assert_eq!(8, layout.visible_occupied(4, 3));
    }

    #[test]
    pub fn visible_occupied2() {
        let layout = Layout::parse(".............
.L.L.#.#.#.#.
.............");

        assert_eq!(Position::Empty, layout.map[1][1]);
        assert_eq!(0, layout.visible_occupied(1, 1));
    }

    #[test]
    pub fn visible_occupied3() {
        let layout = Layout::parse(".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.");

        assert_eq!(Position::Empty, layout.map[3][3]);
        assert_eq!(0, layout.visible_occupied(3, 3));
    }

    #[test]
    fn transform_part2_1() {
        let start = Layout::parse("#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##");

        let expected = Layout::parse("#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#");

        assert_eq!(expected, start.apply_round(false));
    }
}
