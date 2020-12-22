fn main() {
    let args:Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).unwrap();
    let mut game = GameState::parse(&input);

    while !(game.player1.is_empty() || game.player2.is_empty()) {
        game = game.play_round();
    }

    let score = match game.player1.is_empty() {
        true => GameState::score(&game.player2),
        false => GameState::score(&game.player1),
    };

    println!("The winner's score was {}.", score);
}

#[derive(Clone)]
pub struct GameState {
    player1: Vec<usize>,
    player2: Vec<usize>,

}

impl GameState {
    pub fn parse(input: &str) -> Self {
        let mut p1 = Vec::new();
        let mut p2 = Vec::new();
        let mut saw_blank = false;
        for line in input.lines() {
            if line.starts_with("Player") {
                continue;
            } else if line.is_empty() {
                saw_blank = true;
            } else if saw_blank {
                p2.push(line.parse::<usize>().unwrap());
            } else {
                p1.push(line.parse::<usize>().unwrap());
            }
        }
        Self {
            player1: p1,
            player2: p2,
        }
    }

    pub fn play_round(&self) -> Self {
        let mut p1 = self.player1[1..].to_vec();
        let mut p2 = self.player2[1..].to_vec();

        if self.player1[0] > self.player2[0] {
            p1.push(self.player1[0]);
            p1.push(self.player2[0]);
        } else {
            p2.push(self.player2[0]);
            p2.push(self.player1[0]);
        }

        Self {
            player1: p1,
            player2: p2,
        }
    }

    pub fn score(cards: &[usize]) -> usize {
        let mut score = 0;
        for i in 0..cards.len() {
            score += cards[cards.len() - 1 - i] * (i + 1);
        }
        score
    }
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test() {
        let mut game = GameState::parse("Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10");

        game = game.play_round();
        assert_all(&[2, 6, 3, 1, 9, 5], &game.player1);
        assert_all(&[8, 4, 7, 10], &game.player2);

        game = game.play_round();
        assert_all(&[6, 3, 1, 9, 5], &game.player1);
        assert_all(&[4, 7, 10, 8, 2], &game.player2);

        game = game.play_round();
        assert_all(&[3, 1, 9, 5, 6, 4], &game.player1);
        assert_all(&[7, 10, 8, 2], &game.player2);

        game = game.play_round();
        assert_all(&[1, 9, 5, 6, 4], &game.player1);
        assert_all(&[10, 8, 2, 7, 3], &game.player2);

        while !(game.player1.is_empty() || game.player2.is_empty()) {
            game = game.play_round();
        }

        assert!(game.player1.is_empty());
        assert_all(&[3, 2, 10, 6, 8, 5, 9, 4, 7, 1], &game.player2);

        assert_eq!(306, GameState::score(&game.player2));
    }

    fn assert_all<T: std::fmt::Debug+PartialEq>(expected: &[T], actual: &[T]) {
        assert_eq!(expected.len(), actual.len());
        for i in 0..expected.len() {
            assert_eq!(expected[i], actual[i]);
        }
    }
}