use trees::*;

#[test]
fn is_tree_tests() {
    let map = Map::parse_string(
   "..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#");

    assert_eq!(false, map.is_tree(0, 0));
    assert_eq!(true, map.is_tree(0, 2));
    assert_eq!(false, map.is_tree(4, 0));
    assert_eq!(true, map.is_tree(4, 1));
    assert_eq!(false, map.is_tree(0, 11));
    assert_eq!(false, map.is_tree(0, 12));
    assert_eq!(true, map.is_tree(0, 13));
    assert_eq!(true, map.is_tree(0, 14));
    assert_eq!(false, map.is_tree(0, 15));
}

#[test]
fn count_trees_test() {
    let map = Map::parse_string(
   "..##.......
    #...#...#..
    .#....#..#.
    ..#.#...#.#
    .#...##..#.
    ..#.##.....
    .#.#.#....#
    .#........#
    #.##...#...
    #...##....#
    .#..#...#.#");

    assert_eq!(7, map.count_trees(3, 1));
}

#[test]
fn count_trees_multiple_slopes() {
    assert_slope(2, 1, 1);
    assert_slope(7, 3, 1);
    assert_slope(3, 5, 1);
    assert_slope(4, 7, 1);
    assert_slope(2, 1, 2);
}

fn assert_slope(expected: usize, right: usize, down: usize) {
    let map = Map::parse_string(
       "..##.......
        #...#...#..
        .#....#..#.
        ..#.#...#.#
        .#...##..#.
        ..#.##.....
        .#.#.#....#
        .#........#
        #.##...#...
        #...##....#
        .#..#...#.#");

    assert_eq!(expected, map.count_trees(right, down));
}