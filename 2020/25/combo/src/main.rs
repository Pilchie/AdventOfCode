fn main() {
    let card_public_key = 6270530;
    let door_public_key = 14540258;

    let mut loop_number = 1;
    loop {
        let res = transform(7, loop_number);
        if  res == card_public_key {
            println!("Encryption key is {}", transform(door_public_key, loop_number));
            break;
        } else if res == door_public_key {
            println!("Encryption key is {}", transform(card_public_key, loop_number));
            break;
        }

        loop_number += 1;
        if loop_number % 1000 == 0 {
            println!("Trying {}", loop_number);
        }
    }
}

pub fn transform(subject: usize, loop_number: usize) -> usize {
    let mut value = 1;
    for _ in 0..loop_number {
        value *= subject;
        value = value % 20201227;
    }

    value
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test_card() {
        let card_public_key = 5764801;
        let card_loop_number = 8;
        assert_eq!(card_public_key, transform(7, card_loop_number));
    }

    #[test]
    fn test_door() {
        let door_public_key = 17807724;
        let door_loop_number = 11;
        assert_eq!(door_public_key, transform(7, door_loop_number));
    }

    #[test]
    fn test_keys() {
        let card_public_key = 5764801;
        let card_loop_number = 8;
        let door_public_key = 17807724;
        let door_loop_number = 11;
        let card_key = transform(door_public_key, card_loop_number);
        let door_key = transform(card_public_key, door_loop_number);
        assert_eq!(door_key, card_key);
        assert_eq!(14897079, door_key);
    }
}