use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let disk = Disk::parse(&contents);
    let mut part_one = disk.clone();
    part_one.part_one();
    println!("The checksum for part one is {}", part_one.calculate_checksum());

    let mut part_two = disk.clone();
    part_two.part_two();
    println!("The checksum for part two is {}", part_two.calculate_checksum());
}

#[derive(Clone)]
struct Disk {
    blocks: Vec<u16>,
}

impl Disk {
    fn parse(input: &str) -> Self {
        let mut blocks = Vec::new();
        let mut file_or_free = true;
        let mut fileid: u16 = 0;
    
        // Parse
        for ch in input.bytes() {
            let num = ch - b'0';
            match file_or_free {
                true => {
                    for _ in 0..num {
                        blocks.push(fileid);
                    }
                    fileid += 1;
                }
                false => {
                    for _ in 0..num {
                        blocks.push(u16::MAX);
                    }
                }
            };
    
            file_or_free = !file_or_free
        }
    
        Disk{ blocks }
    }

    fn calculate_checksum(&self) -> usize {
        let mut checksum: usize = 0;
        for i in 0..self.blocks.len() {
            if self.blocks[i] == u16::MAX {
                continue;
            }
            let num: usize = self.blocks[i].into();
            checksum = checksum + (i * num);
        }
        checksum
    }

    fn part_one(&mut self) {
        let mut end = self.blocks.len() - 1;
        for i in 0..self.blocks.len() {
            if i == end {
                break;
            }
    
            if self.blocks[i] == u16::MAX {
                self.blocks[i] = self.blocks[end];
                self.blocks[end] = u16::MAX;
                end -= 1;
                while self.blocks[end] == u16::MAX {
                    end -= 1;
                }
            }
        }
    }

    fn part_two(&mut self) {
        let files_in_reverse_order = self.enumerate_files_backwards();

        for (f, fstart, flen) in files_in_reverse_order {
            for (gstart, glen) in self.enumerate_gaps() {
                if gstart >= fstart {
                    break;
                }
                if glen >= flen {
                    for i in 0..flen {
                        self.blocks[gstart + i] = f;
                        self.blocks[fstart + i] = u16::MAX;
                    }
                    break;
                }
            }
        }
    }

    fn enumerate_gaps(&self) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let mut i = 0;

        while i < self.blocks.len() {
            if self.blocks[i] == u16::MAX {
                let mut len = 1;
                while i+len < self.blocks.len() && self.blocks[i+len] == u16::MAX {
                    len += 1;
                }
                res.push((i, len));
                i += len;
            } else {
                i += 1;
            }
        }

        res
    }

    fn enumerate_files_backwards(&self) -> Vec<(u16, usize, usize)> {
        let mut res = Vec::new();
        let mut i = 0;

        while i < self.blocks.len() {
            if self.blocks[i] == u16::MAX {
                i += 1;
            } else {
                let mut len = 1;
                while i+len < self.blocks.len() && self.blocks[i+len] == self.blocks[i] {
                    len += 1;
                }
                res.push((self.blocks[i], i, len));
                i += len;
            }
        }

        res.reverse();

        res
    }

    fn _dump(&self) {
        for n in &self.blocks {
            if *n == u16::MAX {
                print!(".");
            } else {
                print!("{}", n);
            }
        }
        println!();
    }
}
