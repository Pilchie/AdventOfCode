use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");

    let mut disk = Vec::new();
    let mut file_or_free = true;
    let mut fileid: u16 = 0;

    // Parse
    for ch in contents.bytes() {
        let num = ch - b'0';
        match file_or_free {
            true => {
                for _ in 0..num {
                    disk.push(fileid);
                }
                fileid += 1;
            }
            false => {
                for _ in 0..num {
                    disk.push(u16::MAX);
                }
            }
        };

        file_or_free = !file_or_free
    }

    //_dump(&disk);

    // Defrag
    let mut end = disk.len() - 1;
    for i in 0..disk.len() {
        if i == end {
            break;
        }

        if disk[i] == u16::MAX {
            disk[i] = disk[end];
            disk[end] = u16::MAX;
            end -= 1;
            while disk[end] == u16::MAX {
                end -= 1;
            }
        }
    }
    //_dump(&disk);

    // Calculate
    let mut checksum: usize = 0;
    for i in 0..disk.len() {
        if disk[i] == u16::MAX {
            continue;
        }
        let num: usize = disk[i].into();
        checksum = checksum + (i * num);
    }

    println!("The checksum is {}", checksum);
}

fn _dump(disk: &[u16]) {
    for n in disk {
        if *n == u16::MAX {
            print!(".");
        } else {
            print!("{}", n);
        }
    }
    println!();
}
