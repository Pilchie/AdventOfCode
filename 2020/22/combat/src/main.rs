fn main() {
    let args:Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1]).unwrap();
    let game = GameState::parse(&input);

    let (p1_wins, score) = GameState::play_part_2(&game, 1);
    println!("The winner's score was '{}', and p1 was winner: '{}'.", score, p1_wins);
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
        let p1 = self.player1[0] > self.player2[0];
        // println!("Player 1 was winner: {}", p1);
        self.new_with_winner(p1)
    }

    fn new_with_winner(&self, p1_wins: bool) -> Self {
            let mut p1 = self.player1[1..].to_vec();
        let mut p2 = self.player2[1..].to_vec();

        if p1_wins {
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

    pub fn play_part_2(start: &Self, game: usize) -> (bool, usize)
    {
        // println!("=== Game {} ===", game);
        let mut previous = Vec::new();
        let mut state = start.clone();
        while !(state.player1.is_empty() || state.player2.is_empty()) {
            // println!("-- Round {} (Game {}) --", previous.len() + 1, game);
            // println!("Player 1's deck: {:?}", state.player1);
            // println!("Player 2's deck: {:?}", state.player2);

            if previous.iter().any(|g| state.matches(g)) {
                println!("Player 1 wins because game was seen!");
                return (true, Self::score(&state.player1))
            }

            previous.push(state.clone());

            if state.player1[0] < state.player1.len()
                && state.player2[0] < state.player2.len()
            {
                // play recursive game
                let subp1 = &state.player1[1..state.player1[0]+1];
                let subp2 = &state.player2[1..state.player2[0]+1];
                let subgame = Self {
                    player1: subp1.to_vec(),
                    player2: subp2.to_vec(),
                };

                // println!("Playing a sub-game to decide");
                let (p1, _) = Self::play_part_2(&subgame, game + 1);
                state = state.new_with_winner(p1);
                println!("Player 1 won sub-game: {}", p1);
            } else {
                state = state.play_round();
            };
        }

        match state.player1.is_empty() {
            true => (false, Self::score(&state.player2)),
            false => (true, Self::score(&state.player1)),
        }
    }

    fn matches(&self, other: &GameState) -> bool {
        sequence_equal(&self.player1, &other.player1) 
        && sequence_equal(&self.player2, &other.player2)
    }

    pub fn score(cards: &[usize]) -> usize {
        let mut score = 0;
        for i in 0..cards.len() {
            score += cards[cards.len() - 1 - i] * (i + 1);
        }
        score
    }
}

pub fn sequence_equal<T: PartialEq>(left: &[T], right: &[T]) -> bool {
    if left.len() != right.len() {
        return false;
    }

    for i in 0..left.len() {
        if left[i] != right[i] {
            return false;
        }
    }

    true
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

#[cfg(test)]
mod tests_part2 {
    use super::*;

    #[test]
    fn test() {
        let game = GameState::parse("Player 1:
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

        let (_, score) = GameState::play_part_2(&game, 1);
        assert_eq!(291, score);
    }
}