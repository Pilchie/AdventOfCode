use trees::*;

#[test]
fn is_tree_tests() {
    let map = Map::parseString(
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
    let map = Map::parseString(
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

    assert_eq!(7, map.count_trees());
}