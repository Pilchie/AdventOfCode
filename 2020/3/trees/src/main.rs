use std::env;
use trees::Map;

fn main() {
    let args: Vec<String> = env::args().collect();
    let map = Map::parse_file(&args[1]);
    println!("You'll hit {} trees", map.count_trees());
}
