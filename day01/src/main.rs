use std::{env, fs::File, process};
use std::io::{BufReader, BufRead};

const MAGIC_NUM: i32 = 2020;

fn get_input() -> BufReader<File> {  // For easy copy-pasta
    let filename = env::args().skip(1).next().expect("Expected filename as argument");
    BufReader::new(File::open(filename).expect("Error opening file"))
}

fn main() {
    let numbers: Vec<i32> = get_input().lines().map(|line| line.unwrap().parse().unwrap()).collect();
    for a in &numbers {
        for b in &numbers { // I just added an extra loop, w/e
            for c in &numbers {
                if a + b + c == MAGIC_NUM {
                    println!("{}", a * b * c);
                    process::exit(0);
                }
            }
        }
    }
}