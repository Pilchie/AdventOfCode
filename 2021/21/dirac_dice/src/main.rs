fn main() -> Result<(), std::io::Error> {
    let args: Vec<_> = std::env::args().collect();
    let input = std::fs::read_to_string(&args[1])?;

    let mut lines = input.lines();

    let mut p1 = Player::new(lines.next().unwrap().split_ascii_whitespace().nth_back(0).unwrap().parse::<u16>().unwrap());
    let mut p2 = Player::new(lines.next().unwrap().split_ascii_whitespace().nth_back(0).unwrap().parse::<u16>().unwrap());

    let mut die = 0;
    let mut count = 0;

    loop {
        p1 = p1.play(&mut die, &mut count);
        println!("Player 1 rolls x+y+{} and moves to space {} for a total score of {}", die, p1.pos, p1.score);
        if p1.score >= 1000 {
            println!("p1 wins with {}, p2 at {}, die at {}, product is {}", p1.score, p2.score, die + 2, p2.score as u32 * count as u32);
            break;
        }

        p2 = p2.play(&mut die, &mut count);
        println!("Player 2 rolls x+y+{} and moves to space {} for a total score of {}", die, p2.pos, p2.score);
        if p2.score >= 1000 {
            println!("p2 wins with {}, p1 at {}, die at {}, product is {}", p2.score, p1.score, die + 2, p1.score as u32 * count as u32);
            break;
        }
    }

    Ok(())
}

struct Player {
    pos: u16,
    score: u16,
}

impl Player {
    fn new(pos: u16) -> Self {
        Self {
            pos,
            score: 0,
        }
    }

    fn play(&self, die: &mut u16, count: &mut u16) -> Self {
        inc(die, count);
        let mut moves = *die;
        inc(die, count);
        moves += *die;
        inc(die, count);
        moves += *die;

        let mut new_pos = (self.pos + moves) % 10;
        if new_pos == 0 {
             new_pos = 10; 
        }
        Self {
            pos: new_pos,
            score: self.score + new_pos,
        }
    }
}

fn inc(val: &mut u16, count: &mut u16) {
    *count += 1;
    *val += 1;
    if *val > 100 {
        *val = 1;
    }
}