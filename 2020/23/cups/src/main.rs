fn main() {
    let mut g = GameState::new("523764819");
    for _ in 0..100 {
        g = g.play_round();
    }
    println!("{}", g.order_after_one());
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

    pub fn play_round(&self) -> Self {
        println!("-- move {} --", self.move_number);
        println!("cups: ");
        for i in 0..self.cups.len() {
            if i == self.current_index {
                print!("({})", self.cups[i]);
            } else {
                print!(" {} ", self.cups[i]);
            }
        }
        println!("");

        let (mut cups, pick_up) = remove_after(&self.cups, self.current_index, 3);
        println!("pick up {:?}, leaving {:?}", pick_up, cups);
        let mut destination = decrement_wrap(self.cups[self.current_index]);
        while pick_up.contains(&destination) {
            destination = decrement_wrap(destination);
        }

        let dest_index = increment_wrap(cups.iter().position(|x| x == &destination).unwrap());
        println!("Destination is {}, before index {}", destination, dest_index);

        for i in 0..pick_up.len() {
            cups.insert(dest_index, pick_up[pick_up.len() - 1 - i]);
        }

        let current_index_after = cups.iter().position(|x| x == &self.current()).unwrap();

        Self {
            cups: cups.clone(),
            current_index: increment_wrap(current_index_after),
            move_number: self.move_number + 1,
        }
    }

    pub fn current(&self) -> usize {
        self.cups[self.current_index]
    }

    pub fn order_after_one(&self) -> String {
        let one = self.cups.iter().position(|x| x == &1).unwrap();
        let mut cur = increment_wrap(one);
        let mut res = String::from("");
        for _ in 0..self.cups.len() - 1 {
            res = format!("{}{}", res, self.cups[cur]);
            cur = increment_wrap(cur);
        }
        res
    }
}

fn decrement_wrap(val: usize) -> usize {
    let x = val - 1;
    match x {
        0 => 9,
        _ => x,
    }
}

fn increment_wrap(val: usize) -> usize {
    let x = val + 1;
    match x {
        9 => 0,
        _ => x,
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