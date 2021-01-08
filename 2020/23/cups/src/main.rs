use std::cmp::min;
use std::collections::HashMap;
use std::ops::RangeInclusive;

fn main() {
    let mut g = GameState::new_part2("523764819");
    for _ in 0..10_000_000 {
        g.play_round();
    }
    println!("{}", g.products());
}

pub struct GameState {
    cups: HashMap<usize, usize>,
    current_cup: usize,
    move_number: usize,
}

impl GameState {
    pub fn new(input: &str) -> Self {
        let mut cups = HashMap::new();
        let first = input[0..1].parse::<usize>().unwrap();
        let mut cur = first;
        for c in input[1..].chars() {
            let cc = String::from(c).parse::<usize>().unwrap();
            cups.insert(cur, cc);
            cur = cc;
        }
        cups.insert(cur, first);

        Self {
            cups: cups,
            current_cup: first,
            move_number: 1,
        }
    }

    pub fn new_part2(input: &str) -> Self {
        let mut cups = HashMap::new();
        let first = input[0..1].parse::<usize>().unwrap();
        let mut cur = first;
        for c in input[1..].chars() {
            let cc = String::from(c).parse::<usize>().unwrap();
            println!("parsing {}->{}", cur, cc);
            cups.insert(cur, cc);
            cur = cc;
        }

        for cc in cups.len()+2..1_000_001 {
            cups.insert(cur, cc);
            cur = cc;
        }
        println!("linking {}->{}", cur, first);
        cups.insert(cur, first);

        Self {
            cups: cups,
            current_cup: first,
            move_number: 1,
        }
    }

    pub fn play_round(&mut self) {
        // println!("-- move {} --", self.move_number);
        // self.print_state();

        let mut pickup_cups = Vec::new();
        let mut cup = self.cups[&self.current_cup];

        // print!("pick up: ");
        for _ in 0..3 {
            // print!("{} ", cup);
            pickup_cups.push(cup);
            cup = self.cups[&cup];
       }
    //    println!("");

        let mut destination = decrement_wrap(self.current_cup, &RangeInclusive::new(1, self.cups.len()));
        while pickup_cups.contains(&destination) {
            destination = decrement_wrap(destination, &RangeInclusive::new(1, self.cups.len())); 
        }

        // println!("destination: {}", destination);

        self.cups.insert(self.current_cup, self.cups[pickup_cups.last().unwrap()]);
        self.cups.insert(*pickup_cups.last().unwrap(), self.cups[&destination]);
        self.cups.insert(destination, pickup_cups[0]);

        self.current_cup = self.cups[&self.current_cup];
        self.move_number += 1;
    }

    fn print_state(&self) {
        print!("cups: ");
        let mut cup = self.current_cup;
        print!("({})", cup);
        cup = self.cups[&cup];
        for _ in 1..min(self.cups.len() -1, 15) {
            print!(" {} ", cup);
            cup = self.cups[&cup];
        }
        println!("");
    }

    pub fn current(&self) -> usize {
        self.current_cup
    }

    pub fn order_after_one(&self) -> String {
        let mut res = String::from("");
        let mut cur = self.cups[&1];
        for _ in 0..self.cups.len() - 1
        {
            res = format!("{}{}", res, cur);
            cur = self.cups[&cur];
        }
        res
    }

    pub fn products(&self) -> usize {
        let c = self.cups[&1];
        c * self.cups[&c]
    }
}

fn decrement_wrap(val: usize, bounds: &RangeInclusive<usize>) -> usize {
    if &val == bounds.start() {
        *bounds.end()
    } else {
        val - 1
    }
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
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test() {
        let mut state = GameState::new_part2("389125467");
        state.print_state();
        for _ in 0..10_000_000 {
            state.play_round();
        }

        assert_eq!(149245887792, state.products());
    }
}