use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let mut g = Game::new(&[0,6,1,7,2,19,20]);
    if let Some(x) = g.nth(2019) {
        println!("The 2020th number is: {}", x);
    } else {
        println!("The sequence ended");
    }
}

pub struct Game {
    last_num: usize,
    last_index: usize,
    starts: VecDeque<usize>,
    prevs: HashMap<usize, (usize, Option<usize>)>,  // a map from number, to the last two indices it was referenced at
}

impl Game {
    pub fn new(starting: &[usize]) -> Self {
        let mut g = Game {
            last_num: 0,
            last_index: 0,
            starts: starting.iter().map(|x| *x).collect(),
            prevs: HashMap::new(),
        };

        starting.iter().for_each(|x| g.insert(*x));

        g
    }

    fn insert(&mut self, num: usize) {
        self.last_index += 1;

        let res = match self.prevs.get(&num) {
            Some((p1, _)) => (self.last_index, Some(*p1)),
            None => (self.last_index, None),
        };

        self.prevs.insert(num, res);
    }
}

impl Iterator for Game {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.starts.pop_front() {
            self.last_num = x;
            return Some(x);
        }

        let res = match self.prevs.get(&self.last_num) {
            Some((p1, x)) => match x {
                Some(p2) => p1 - p2,
                None => 0,
            },
            None => 0,
        };

        self.insert(res);
        self.last_num = res;
        Some(self.last_num)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text() {
        let mut g = Game::new(&[0, 3, 6]);
        assert_eq!(Some(0), g.next());  // turn 1
        assert_eq!(Some(3), g.next());  // turn 2
        assert_eq!(Some(6), g.next());  // turn 3
        assert_eq!(Some(0), g.next());  // turn 4
        assert_eq!(Some(3), g.next());  // turn 5
        assert_eq!(Some(3), g.next());  // turn 6
        assert_eq!(Some(1), g.next());  // turn 7
        assert_eq!(Some(0), g.next());  // turn 8
        assert_eq!(Some(4), g.next());  // turn 9
        assert_eq!(Some(0), g.next());  // turn 10
    }

    #[test]
    fn text_2020th() {
        let mut g = Game::new(&[0, 3, 6]);
        // nth is 0 based, so use 2019 instead of 2020.
        assert_eq!(Some(436), g.nth(2019));
    }

    #[test]
    fn example1() {
        let mut g = Game::new(&[1, 3, 2]);
        // nth is 0 based, so use 2019 instead of 2020.
        assert_eq!(Some(1), g.nth(2019));
    }

    #[test]
    fn example2() {
        let mut g = Game::new(&[2, 1, 3]);
        // nth is 0 based, so use 2019 instead of 2020.
        assert_eq!(Some(10), g.nth(2019));
    }

    #[test]
    fn example3() {
        let mut g = Game::new(&[1, 2, 3]);
        // nth is 0 based, so use 2019 instead of 2020.
        assert_eq!(Some(27), g.nth(2019));
    }

    #[test]
    fn example4() {
        let mut g = Game::new(&[2, 3, 1]);
        // nth is 0 based, so use 2019 instead of 2020.
        assert_eq!(Some(78), g.nth(2019));
    }
    #[test]
    fn example5() {
        let mut g = Game::new(&[3, 2, 1]);
        // nth is 0 based, so use 2019 instead of 2020.
        assert_eq!(Some(438), g.nth(2019));
    }
    #[test]
    fn example6() {
        let mut g = Game::new(&[3, 1, 2]);
        // nth is 0 based, so use 2019 instead of 2020.
        assert_eq!(Some(1836), g.nth(2019));
    }
}
