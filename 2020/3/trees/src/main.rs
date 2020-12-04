use std::env;
use trees::Map;

fn main() {
    let args: Vec<String> = env::args().collect();
    let map = Map::parse_file(&args[1]);
    let mut result = 1;
    result *= map.count_trees(1, 1);
    result *= map.count_trees(3, 1);
    result *= map.count_trees(5, 1);
    result *= map.count_trees(7, 1);
    result *= map.count_trees(1, 2);

    println!("The solution is {}", result);
}
