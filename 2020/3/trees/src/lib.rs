use std::fs::File;
use std::io::{self, BufRead};

pub struct Map {
    trees: Vec<String>
}

impl Map {
    pub fn parse_file(path: &str) -> Map {
        let file = File::open(path);
    
        let reader = io::BufReader::new(file.unwrap());
        let mut trees = Vec::new();
        for line in reader.lines() {
            trees.push(line.unwrap());
        }

        Map::parse_lines(trees)
    }

    pub fn parse_string(map: &str) -> Map {
        let mut trees = Vec::new();
        for line in map.split_whitespace() {
            trees.push(String::from(line));
        }
        Map::parse_lines(trees)
    }

    fn parse_lines(map: Vec<String>) -> Map {
        Map { trees: map }
    }

    pub fn is_tree(&self, row: usize, col: usize) -> bool {
        let len = self.trees[0].len();
        let c = col % len;
        self.trees[row].chars().nth(c).unwrap() == '#'
    }

    pub fn count_trees(&self, right: usize, down: usize) -> usize {
        let mut row = 0;
        let mut col = 0;
        let mut count = 0;
        while row < self.trees.len() {
            if self.is_tree(row, col) {
                count += 1;
            }
            row += down;
            col += right;
        }

        count
    }
}