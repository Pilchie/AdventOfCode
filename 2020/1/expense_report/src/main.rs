use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = parse_file(&args[1]);
    part1(&nums);
    part2(&nums);
}

fn parse_file(path: &str) -> Vec<i32> {
    let file = File::open(path);

    let reader = io::BufReader::new(file.unwrap());
    let mut nums: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let i = line.unwrap().parse::<i32>().unwrap();
        nums.push(i);
    }

    nums
}

fn part1(nums: &Vec<i32>) {
    for &i in nums {
        for &j in nums {
            if i + j == 2020 {
                println!("The values are {} and {}, result is {}", i, j, i * j);
            }
        }
    }
}

fn part2(nums: &Vec<i32>) {
    for &i in nums {
        for &j in nums {
            for &k in nums {
                if i + j + k == 2020 {
                    println!(
                        "The values are {}, {}, and {}, result is {}",
                        i,
                        j,
                        k,
                        i * j * k
                    );
                }
            }
        }
    }
}
