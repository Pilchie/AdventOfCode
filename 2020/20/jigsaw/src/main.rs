use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();
    
    let image = Image::parse(&mut BufReader::new(std::fs::File::open(&args[1])?))?;
    let res = image.solve();
    let product = res[0][0].id * res[0][res[0].len() - 1].id * res[res.len() - 1][0].id * res[res.len() - 1][res[res.len() - 1].len() - 1].id;
    let (count, final_image) = Image::count_rough(&Image::assemble(&res));
    println!("Product of corners: {}, rough seas: {}", product, count);

    for row in final_image {
        for c in row {
            print!("{}", c as char);
        }
        println!("");
    }
    Ok(())
}

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    ParseInt(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for Error {
    fn from(pie: std::num::ParseIntError) -> Self {
        Self::ParseInt(pie)
    }
}

impl From<std::io::Error> for Error {
    fn from(ioe: std::io::Error) -> Self {
        Self::IO(ioe)
    }
}

pub struct Image {
    tiles: Vec<Tile>,
}

impl Image {
    pub fn parse<T:BufRead>(reader:&mut T) -> Result<Self, Error> {
        let mut tiles = Vec::new();
        let mut curr = Vec::new();
        let mut id = 0;
        for l in reader.lines() {
            let line = l?;
            if line.is_empty() {
                let t = Tile::new(id, curr.split_off(0));
                println!("Created tile {:?}", t);
                tiles.push(t);
            } else if line.starts_with("Tile ") {
                id = line[5..9].parse()?;
            } else {
                curr.push(line.clone())
            }
        }

        Ok(Self{
            tiles: tiles,
        })
    }

    pub fn solve(&self) -> Vec<Vec<Tile>> {
        let mut tiles = self.tiles.clone();

        println!("Found {} corners", tiles.iter().filter(|t| self.is_corner(t)).count());

        let size = ((tiles.len() as f64).sqrt()) as usize;
        println!("Have {} tiles, size is {}", tiles.len(), size);

        let mut res = Vec::new();
        let mut line = Vec::new();
        println!("finding item in row 0 at 0");
        let top_left = self.find_top_left(&mut tiles);
        line.push(top_left);
        for x in 1..size {
            println!("finding item in row 0 at {}, need left of {}", x, line[x-1].right);
            line.push(self.find_match_no_top(&mut tiles, line[x-1].right));
        }
        res.push(line);

        for y in 1..size {
            line = Vec::new();
            println!("finding item in row {} at {}, need top of {}", y, 0, res[y-1][0].bottom);
            line.push(self.find_match_no_left(&mut tiles, res[y-1][0].bottom));
            for x in 1..size {
                println!("finding item in row {} at {}, need top, left of {}, {}", y, x, line[x-1].right, res[y-1][x].bottom);
                line.push(self.find_match(&mut tiles, line[x-1].right, res[y-1][x].bottom));
            }
            res.push(line);
        }
        res
    }

    fn find_top_left(&self, tiles: &mut Vec<Tile>) -> Tile {
        let mut corner = None;
        for i in 0..tiles.len() {
            if self.is_corner(&tiles[i]) {
                corner = Some(tiles.remove(i));
                break;
            }
        }

        let mut top_left = corner.unwrap();
        while self.border_has_match(top_left.left, top_left.id)
            || self.border_has_match(top_left.top, top_left.id)
        {
            top_left = top_left.rotate();
        }

        println!("Found match: {:?}", top_left);
        top_left
    }

    fn find_match_no_top(&self, tiles: &mut Vec<Tile>, left: usize) -> Tile {
        let mut tile = None;
        for i in 0..tiles.len() {
            if tiles[i].has_border(left) {
                tile = Some(tiles.remove(i));
                break;
            }
        }

        let mut res = tile.unwrap();
        while res.left != left {
            res = res.rotate();
        }

        if self.border_has_match(res.top, res.id) {
            res = res.flip();
        }
        assert!(!self.border_has_match(res.top, res.id));

        println!("Found match: {:?}", res);
        res
    }

    fn find_match_no_left(&self, tiles: &mut Vec<Tile>, top: usize) -> Tile {
        let mut tile = None;
        for i in 0..tiles.len() {
            if tiles[i].has_border(top) {
                tile = Some(tiles.remove(i));
                break;
            }
        }

        let mut res = tile.unwrap();
        while self.border_has_match(res.left, res.id) {
            res = res.rotate();
        }

        if res.top != top {
            res = res.flip();
        }
        println!("Found match: {:?}", res);
        assert_eq!(res.top, top);

        // println!("Found match: {:?}", res);
        res
    }

    fn find_match(&self, tiles: &mut Vec<Tile>, left: usize, top: usize) -> Tile {
        let mut tile = None;
        for i in 0..tiles.len() {
            if tiles[i].has_border(left) {
                tile = Some(tiles.remove(i));
                break;
            }
        }

        let mut res = tile.unwrap();
        while res.left != left {
            res = res.rotate();
        }

        if res.top != top {
            res = res.flip();
        }

        println!("Found match: {:?}", res);
        res
    }

    pub fn assemble(tiles: &Vec<Vec<Tile>>) -> Vec<Vec<u8>> {
        let mut res = Vec::new();
        for ty in tiles {
            for y in 1..9 {
                let mut row = Vec::new();
                for tx in ty {
                    for x in 1..9 {
                        row.push(tx.pixels[y][x]);
                    }
                }
                res.push(row);
            }
        }

        res
    }

    pub fn count_rough(input: &Vec<Vec<u8>>) -> (usize, Vec<Vec<u8>>) {
        let monster = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
        let mut monster_points = Vec::new();
        for (y, line) in monster.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                if b == b'#' {
                    monster_points.push((y, x));
                }
            }
        }

        let mut image = input.clone();
        let mut monsters = 0;
        while monsters == 0 {
            for y in 0..image.len() - monster.lines().count() {
                for x in 0..image[y].len() - monster.lines().nth(0).unwrap().len() {
                    if Self::replace_monster(&mut image, y, x, &monster_points) {
                        monsters += 1;
                    }
                }
            }
            image = rotate(&image);
        }

        let mut count = 0;
        for y in 0..image.len() {
            for x in 0..image[y].len() {
                if image[y][x] == b'#' {
                    count += 1;
                }
            }
        }

        (count, image)
    }

    fn replace_monster(image: &mut Vec<Vec<u8>>, y: usize, x:usize, monster_points: &Vec<(usize, usize)>) -> bool {
        if monster_points.iter().all(|(dy, dx)| image[y + dy][x + dx] == b'#') {
            monster_points.iter().for_each(|(dy, dx)| image[y + dy][x + dx] = b'O');
            return true;
        }
        false
    }

    fn border_has_match(&self, border: usize, tile_id: usize) -> bool {
        self.tiles.iter().any(|t| t.has_border(border) && t.id != tile_id)
    }

    fn is_corner(&self, tile: &Tile) -> bool {
        let matched_0 = self.border_has_match(tile.left, tile.id);
        let matched_1 = self.border_has_match(tile.top, tile.id);
        let matched_2 = self.border_has_match(tile.right, tile.id);
        let matched_3 = self.border_has_match(tile.bottom, tile.id);

        (!matched_0 && !matched_1)
            || (!matched_1 && !matched_2)
            || (!matched_2 && !matched_3)
            || (!matched_3 && !matched_0)
    }
}

#[derive(Clone, Debug)]
pub struct Tile {
    id: usize,
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
    pixels: Vec<Vec<u8>>,
}

impl Tile {
    pub fn new(id: usize, lines: Vec<String>) -> Self {
        let (top, bottom, left, right) = (
            Self::border_id(lines[0].bytes()),
            Self::border_id(lines[9].bytes()),
            Self::border_id(lines.iter().map(|l| l.bytes().nth(0).unwrap())),
            Self::border_id(lines.iter().map(|l| l.bytes().nth(9).unwrap()))
        );
        let mut pixels = Vec::new();
        for l in lines {
            let mut line = Vec::new();
            for b in l.bytes() {
                line.push(b);
            }
            pixels.push(line);
        }
        Self {
            id: id,
            left: left,
            top: top,
            right: right,
            bottom: bottom,
            pixels: pixels,
        }
    }

    fn border_id<T: Iterator<Item=u8>>(bytes: T) -> usize {
        let v : Vec<_> = bytes.collect();
        let mut res1 = 0;
        let mut res2 = 0;
        for i in 0..v.len() {
            if v[i] == b'#' {
                res1 += 1 << i;
                res2 += 1 << (v.len() - 1 - i);
            }
        }

        if res1 < res2 {
            res1
        } else {
            res2
        }
    }

    fn has_border(&self, border: usize) -> bool {
        self.left == border
            || self.top == border
            ||  self.right == border
            || self.bottom == border
    }

    fn rotate(&self) -> Self {
        let pixels = rotate(&self.pixels);

        Self {
            id: self.id,
            left: self.top,
            top: self.right,
            right: self.bottom,
            bottom: self.left,
            pixels: pixels,
        }
    }

    fn flip(&self) -> Self {
        let mut pixels = Vec::new();
        for i in 0..self.pixels.len() {
            pixels.push(self.pixels[self.pixels.len() - 1 - i].clone());
        }
        Self {
            id: self.id,
            left: self.left,
            top: self.bottom,
            right: self.right,
            bottom: self.top,
            pixels: pixels,
        }
    }
}

pub fn rotate<T: Copy>(incoming: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut matrix = Vec::new();
    for i in 0..incoming.len() {
        matrix.push(incoming[i].clone());
    }

    let n = matrix.len();
    for x in 0..n/2 {
        for y in x..n-x-1 {
            let temp = matrix[x][y];

            // Move values from right to top
            matrix[x][y] = matrix[y][n - 1 - x];

            // Move values from bottom to right
            matrix[y][n - 1 - x] = matrix[n - 1 - x][n - 1 - y];

            // Move values from left to bottom
            matrix[n - 1 - x][n - 1 - y] = matrix[n - 1 - y][x];

            // Assign temp to left
            matrix[n - 1 - y][x] = temp;
        }
    }

    matrix
}

#[cfg(test)]
mod tests_part1 {
    use std::io::Cursor;
    use super::*;

    #[test]
    fn test() -> Result<(), Error> {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...

";

        let image = Image::parse(&mut Cursor::new(input))?;
        let res = image.solve();
        let product = res[0][0].id * res[0][res[0].len() - 1].id * res[res.len() - 1][0].id * res[res.len() - 1][res[res.len() - 1].len() - 1].id;
        assert_eq!(20899048083289, product);
        let (count, _) = Image::count_rough(&Image::assemble(&res));
        assert_eq!(273, count);
        Ok(())
    }
}