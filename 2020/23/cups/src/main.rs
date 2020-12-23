use std::ops::RangeInclusive;

fn main() {
    let mut g = GameState::new_part2("523764819");
    for _ in 0..10_000_000 {
        g = g.play_round();
    }
    println!("{}", g.products());
}

pub struct GameState {
    cups: Vec<usize>,
    current_index: usize,
    move_number: usize,
}

impl GameState {
    pub fn new(input: &str) -> Self {
        let cups: Vec<_> = input.chars().map(|c| String::from(c).parse::<usize>().unwrap()).collect();
        Self {
            cups: cups,
            current_index: 0,
            move_number: 1,
        }
    }

    pub fn new_part2(input: &str) -> Self {
        let mut cups: Vec<_> = input.chars().map(|c| String::from(c).parse::<usize>().unwrap()).collect();
        for i in cups.len()+1..1_000_001 {
            cups.push(i);
        }

        Self {
            cups: cups,
            current_index: 0,
            move_number: 1,
        }
    }

    pub fn play_round(&self) -> Self {
        println!("-- move {} --", self.move_number);
        // print_state();

        let (mut cups, pick_up) = remove_after(&self.cups, self.current_index, 3);
        // println!("pick up {:?}", pick_up);
        let mut destination = decrement_wrap(self.cups[self.current_index], &RangeInclusive::new(1, self.cups.len()));
        while pick_up.contains(&destination) {
            destination = decrement_wrap(destination, &RangeInclusive::new(1, self.cups.len())); 
        }

        println!("Destination is {}", destination);
        let dest_index = increment_wrap(cups.iter().position(|x| x == &destination).unwrap(), &RangeInclusive::new(0, self.cups.len() - 1));
        // println!("Destination is {}, before index {}", destination, dest_index);

        for i in 0..pick_up.len() {
            cups.insert(dest_index, pick_up[pick_up.len() - 1 - i]);
        }

        let current_index_after = cups.iter().position(|x| x == &self.current()).unwrap();

        Self {
            cups: cups,
            current_index: increment_wrap(current_index_after, &RangeInclusive::new(0, self.cups.len() - 1)),
            move_number: self.move_number + 1,
        }
    }

    pub fn print_state(&self) {
        println!("cups: ");
        for i in 0..self.cups.len() {
            if i == self.current_index {
                print!("({})", self.cups[i]);
            } else {
                print!(" {} ", self.cups[i]);
            }
        }
        println!("");
    }

    pub fn current(&self) -> usize {
        self.cups[self.current_index]
    }

    pub fn order_after_one(&self) -> String {
        let one = self.cups.iter().position(|x| x == &1).unwrap();
        let mut cur = one;
        let mut res = String::from("");
        for _ in 0..self.cups.len() - 1 {
            cur = increment_wrap(cur, &RangeInclusive::new(0, self.cups.len() - 1));
            res = format!("{}{}", res, self.cups[cur]);
        }
        res
    }

    pub fn products(&self) -> usize {
        let one = self.cups.iter().position(|x| x == &1).unwrap();
        let mut cur = increment_wrap(one, &RangeInclusive::new(0, self.cups.len() - 1));
        let after_one = self.cups[cur];
        cur = increment_wrap(cur, &RangeInclusive::new(0, self.cups.len() - 1));
        let after_after_one = self.cups[cur];
        after_one * after_after_one
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

fn remove_after<T: Copy>(input: &[T], index: usize, count: usize) -> (Vec<T>, Vec<T>)
{
    let mut remaining = input.to_vec();
    let mut removed = Vec::new();

    for i in 1..count+1 {
        if index + 1 < remaining.len() {
            remaining.remove(index + 1);
        } else {
            remaining.remove(0);
        }

        if index + i < input.len() {
            removed.push(input[index + i]);
        } else {
            removed.push(input[index + i - input.len()]);
        }
    }

    (remaining, removed)
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test() {
        let mut state = GameState::new("389125467");
        assert_all(&[3,8,9,1,2,5,4,6,7], &state.cups);
        assert_eq!(3, state.current());

        state = state.play_round();
        assert_all(&[3, 2, 8, 9, 1, 5, 4, 6, 7], &state.cups);
        assert_eq!(2, state.current());

        state = state.play_round();
        assert_all(&[3, 2, 5, 4, 6, 7, 8, 9, 1], &state.cups);
        assert_eq!(5, state.current());

        state = state.play_round();
        // assert_all(&[7, 2, 5, 8, 9, 1, 3, 4, 6], &state.cups);
        assert_eq!(8, state.current());

        state = state.play_round();
        // assert_all(&[3, 2, 5, 8, 4, 6, 7, 9, 1], &state.cups);
        assert_eq!(4, state.current());

        state = state.play_round();
        // assert_all(&[9, 2, 5, 8, 4, 1, 3, 6, 7], &state.cups);
        assert_eq!(1, state.current());

        state = state.play_round();
        // assert_all(&[7, 2, 5, 8, 4, 1, 9, 3, 6], &state.cups);
        assert_eq!(9, state.current());

        state = state.play_round();
        // assert_all(&[8, 3, 6, 7, 4, 1, 9, 2, 5], &state.cups);
        assert_eq!(2, state.current());

        state = state.play_round();
        // assert_all(&[7, 4, 1, 5, 8, 3, 9, 2, 6], &state.cups);
        assert_eq!(6, state.current());

        state = state.play_round();
        // assert_all(&[5, 7, 4, 1, 8, 3, 9, 2, 6], &state.cups);
        assert_eq!(5, state.current());

        state = state.play_round();
        // assert_all(&[], &state.cups);
        assert_eq!(8, state.current());

        assert_eq!("92658374", &state.order_after_one())
    }

    fn assert_all<T: std::fmt::Debug+PartialEq>(expected: &[T], actual: &[T]) {
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], actual[i]);
        }
    }
}

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test() {
        let mut state = GameState::new_part2("389125467");
        for _ in 0..10_000_000 {
            state = state.play_round();
        }

        assert_eq!(149245887792, state.products());
    }
}