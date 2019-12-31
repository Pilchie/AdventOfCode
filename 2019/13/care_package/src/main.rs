use care_package::play_game;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut program = parse_input_file(&args[1]);
    play_game(&mut program);
}


fn parse_input_file(input_filename: &str) -> Vec<i64> {
    let contents = fs::read_to_string(input_filename).expect("Something went wrong reading the file");

    let mut input: Vec<i64> = Vec::new();
    for i in contents.split(',') {
        input.push(i.parse::<i64>().unwrap());
    }

    input
}
