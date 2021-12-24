fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut lines = input.lines();

    let rollmap = build_rollmap();

    let p1 = Player::new(lines.next().unwrap().split_ascii_whitespace().nth_back(0).unwrap().parse::<u8>().unwrap());
    let p2 = Player::new(lines.next().unwrap().split_ascii_whitespace().nth_back(0).unwrap().parse::<u8>().unwrap());

    let mut state = TurnState::new(p1, p2);
    let mut won_states = Vec::new();
    let mut turn = 1;
    while !state.state.is_empty() {
        println!("Playing turn {}", turn);
        state = state.play_turn(&mut won_states, &rollmap);
        println!("   {} states in play", state.state.len());
        println!("   {} states ended", won_states.len());
        turn += 1;
    }

    let mut times_p1_won = 0;
    for (p1, _, combos) in won_states {
        if p1 >= 21 {
            times_p1_won += combos;
        }
    }

    println!("Player 1 wins in {} universes", times_p1_won);

    Ok(())
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Player {
    pos: u8,
    score: u8,
}

impl Player {
    fn new(pos: u8) -> Self {
        Player {
            pos,
            score: 0,
        }
    }

    fn advance(&self, roll: u8) -> Self {
        let mut new_pos = (self.pos + roll) % 10;
        if new_pos == 0 {
            new_pos = 10; 
        }
        Player {
            pos: new_pos,
            score: self.score + new_pos,
        }
    }
}

struct TurnState {
    state: Vec<(Player, Player, u64)>,
}

impl TurnState {
    fn new(p1: Player, p2: Player) -> Self {
        let mut state = Vec::new();
        state.push((p1, p2, 1));
        Self { state }
    }

    fn play_turn(&self, won_states: &mut Vec<(u8, u8, u64)>, rollmap: &Vec<(u8, u64)>) -> Self {
        let mut new = Vec::new();
        for (p1_state, p2_state, combos) in &self.state {
            for (p1_roll, p1_combo) in rollmap {
                let p1_new_state = p1_state.advance(*p1_roll);
                if p1_new_state.score >= 21 {
                    won_states.push((p1_new_state.score, 0, combos * p1_combo));
                } else {
                    for (p2_roll, p2_combo) in rollmap {
                        let p2_new_state = p2_state.advance(*p2_roll);
                        let new_combos = combos * p1_combo * p2_combo;
                        if p2_new_state.score >= 21 {
                            won_states.push((p1_new_state.score, p2_new_state.score, new_combos));
                        } else {
                            new.push((p1_new_state.clone(), p2_new_state, new_combos));
                        }
                    }
                }
            }
        }

        Self {
            state: new,
        }
    }
}

// Map of roll amount to number of times it occurs.Sized
fn build_rollmap() -> Vec<(u8, u64)>{
    let mut rollmap = Vec::new();
    rollmap.push((3, 1));
    rollmap.push((4, 3));
    rollmap.push((5, 6));
    rollmap.push((6, 7));
    rollmap.push((7, 6));
    rollmap.push((8, 3));
    rollmap.push((9, 1));
    rollmap
}