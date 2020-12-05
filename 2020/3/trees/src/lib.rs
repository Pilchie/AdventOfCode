use std::fs::File;
use std::io::{self, BufRead};

pub struct Map {
    trees: Vec<Vec<bool>>
}

impl Map {
    pub fn parse_file(path: &str) -> Map {
        let file = File::open(path);
    
        let reader = io::BufReader::new(file.unwrap());
        let mut trees = Vec::new();
        for line in reader.lines() {
            trees.push(line.unwrap());
        }

        Map::parse_lines(&trees)
    }

    pub fn parse_string(map: &str) -> Map {
        let mut trees = Vec::new();
        for line in map.split_whitespace() {
            trees.push(String::from(line));
        }
        Map::parse_lines(&trees)
    }

    fn parse_lines(map: &[String]) -> Map {
        let mut t = Vec::new();
        for r in map {
            let mut row = Vec::new();
            for c in r.chars() {
                row.push(c=='#');
            }
            t.push(row);
        }
        Map {
            trees: t
        }
    }

    pub fn is_tree(&self, row: usize, col: usize) -> bool {
        let len = self.trees[0].len();
        let c = col % len;
        self.trees[row][c]
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