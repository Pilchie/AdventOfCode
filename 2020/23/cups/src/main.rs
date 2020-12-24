use std::collections::LinkedList;
use std::ops::RangeInclusive;

fn main() {
    let mut g = GameState::new_part2("523764819");
    for _ in 0..10_000_000 {
        g.play_round();
    }
    println!("{}", g.products());
}

pub struct GameState {
    cups: LinkedList<usize>,
    current_index: usize,
    move_number: usize,
}

impl GameState {
    pub fn new(input: &str) -> Self {
        let cups: LinkedList<_> = input.chars().map(|c| String::from(c).parse::<usize>().unwrap()).collect();
        Self {
            cups: cups,
            current_index: 0,
            move_number: 1,
        }
    }

    pub fn new_part2(input: &str) -> Self {
        let mut cups: LinkedList<_> = input.chars().map(|c| String::from(c).parse::<usize>().unwrap()).collect();
        for i in cups.len()+1..1_000_001 {
            cups.push_back(i);
        }

        Self {
            cups: cups,
            current_index: 0,
            move_number: 1,
        }
    }

    pub fn play_round(&mut self) {
        // println!("-- move {} --", self.move_number);
        // self.print_state();

        let orig_len = self.cups.len();
        let current = self.current();
        let mut pick_up = remove_after(&mut self.cups, self.current_index, 3);
        // println!("pick up {:?}", pick_up);
        let mut destination = decrement_wrap(current, &RangeInclusive::new(1, orig_len));
        while pick_up.contains(&destination) {
            destination = decrement_wrap(destination, &RangeInclusive::new(1, orig_len)); 
        }

        let dest_index = increment_wrap(self.cups.iter().position(|x| x == &destination).unwrap(), &RangeInclusive::new(0, self.cups.len() - 1));
        // println!("Destination is {}, before index {}", destination, dest_index);

        let mut temp = self.cups.split_off(dest_index);
        self.cups.append(&mut pick_up);
        self.cups.append(&mut temp);

        let curr_index_after = self.cups.iter().position(|x| x == &current).unwrap();

        self.current_index = increment_wrap(curr_index_after, &RangeInclusive::new(0, self.cups.len() - 1));
        self.move_number += 1;
    }

    fn print_state(&self) {
        print!("cups: ");
        let mut i = 0;
        for c in &self.cups {
            if i == self.current_index {
                print!("({})", c);
            } else {
                print!(" {} ", c);
            }
            i += 1;
        }
        println!("");
    }

    pub fn current(&self) -> usize {
        *self.cups.iter().nth(self.current_index).unwrap()
    }

    pub fn order_after_one(&self) -> String {
        let mut res = String::from("");
        let mut after_one = false;
        for c in &self.cups {
            if after_one {
                res = format!("{}{}", res, c);
            }
            if c == &1 {
                after_one = true;
            }
        }
        for c in &self.cups {
            if c == &1 {
                break;
            }
            res = format!("{}{}", res, c);
        }
        res
    }

    pub fn products(&self) -> usize {
        let mut count = 0;
        let mut after_one = false;
        let mut res = 1;
        for c in &self.cups {
            if after_one && count < 2 {
                res *= c;
                count += 1;
            }
            if c == &1 {
                after_one = true;
            }

            if count == 2 {
                return res;
            }
        }

        for c in &self.cups {
            res *= c;
            count += 1;
            if count == 2 {
                break;
            }
        }
        res
    }
}

fn decrement_wrap(val: usize, bounds: &RangeInclusive<usize>) -> usize {
    if &val == bounds.start() {
        *bounds.end()
    } else {
        val - 1
    }
}

fn increment_wrap(val: usize, bounds: &RangeInclusive<usize>) -> usize {
    if &val == bounds.end() {
        *bounds.start()
    } else {
        val + 1
    }
}

fn remove_after<T: Copy>(input: &mut LinkedList<T>, index: usize, count: usize) -> LinkedList<T> {
    let mut removed = input.split_off(index + 1);
    if count < removed.len() {
        let mut temp = removed.split_off(count);
        input.append(&mut temp);
    } else {
        let mut t = input.split_off(count - removed.len());
        removed.append(input);
        input.append(&mut t);
    }

    removed
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test() {
        let mut state = GameState::new("389125467");
        // assert_all(&mut [3, 8, 9, 1, 2, 5, 4, 6, 7].iter(), &mut state.cups.iter());
        assert_eq!(3, state.current());

        state.play_round();
        // assert_all(&mut [3, 2, 8, 9, 1, 5, 4, 6, 7].iter(), &mut state.cups.iter());
        assert_eq!(2, state.current());

        state.play_round();
        // assert_all(&mut [3, 2, 5, 4, 6, 7, 8, 9, 1].iter(), &mut state.cups.iter());
        assert_eq!(5, state.current());

        state.play_round();
        // assert_all(&mut [7, 2, 5, 8, 9, 1, 3, 4, 6].iter(), &mut state.cups.iter());
        assert_eq!(8, state.current());

        state.play_round();
        // assert_all(&mut [3, 2, 5, 8, 4, 6, 7, 9, 1].iter(), &mut state.cups.iter());
        assert_eq!(4, state.current());

        state.play_round();
        // assert_all(&mut [9, 2, 5, 8, 4, 1, 3, 6, 7].iter(), &mut state.cups.iter());
        assert_eq!(1, state.current());

        state.play_round();
        // assert_all(&mut [7, 2, 5, 8, 4, 1, 9, 3, 6].iter(), &mut state.cups.iter());
        assert_eq!(9, state.current());

        state.play_round();
        // assert_all(&mut [8, 3, 6, 7, 4, 1, 9, 2, 5].iter(), &mut state.cups.iter());
        assert_eq!(2, state.current());

        state.play_round();
        // assert_all(&mut [7, 4, 1, 5, 8, 3, 9, 2, 6].iter(), &mut state.cups.iter());
        assert_eq!(6, state.current());

        state.play_round();
        // assert_all(&mut [5, 7, 4, 1, 8, 3, 9, 2, 6].iter(), &mut state.cups.iter());
        assert_eq!(5, state.current());

        state.play_round();
        assert_eq!(8, state.current());

        assert_eq!("92658374", &state.order_after_one())
    }

    fn assert_all<I1, I2, T>(expected: &mut I1, actual: &mut I2)
        where I1: Iterator<Item=T>,
              I2: Iterator<Item=T>,
              T: std::fmt::Debug+PartialEq  {
        loop {
            if let Some(e) = expected.next() {
                match actual.next() {
                    Some(a) => assert_eq!(e, a),
                    None => assert!(false, format!("expected: '{:?}', actual end", e)),
                }
            } else {
                match actual.next() {
                    Some (a) => assert!(false, format!("expected end, actual: '{:?}'", a)),
                    None => return,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test() {
        let mut state = GameState::new_part2("389125467");
        for _ in 0..100_000 {
            state.play_round();
        }

        assert_eq!(149245887792, state.products());
    }
}