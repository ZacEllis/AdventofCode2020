use std::{env, fs::File};
use std::io::{BufReader, BufRead};

use regex::Regex;

const RE_PATTERN: &str = r"(?P<min>\d+)-(?P<max>\d+) (?P<letter>.): (?P<password>.+)";

fn get_input() -> BufReader<File> {  // For easy copy-pasta
    let filename = env::args().skip(1).next().expect("Expected filename as argument");
    BufReader::new(File::open(filename).expect("Error opening file"))
}

fn index_eq(password: &str, substr: &str, index: usize) -> bool {
    password.as_bytes()[index-1] == substr.as_bytes()[0]
}

fn main() {
    let mut valid = 0;
    let re = Regex::new(RE_PATTERN).unwrap();
    for result_line in get_input().lines() {
        let line = result_line.unwrap();
        let captures = re.captures(&line).unwrap();
        let first_pos: usize = captures.name("min").unwrap().as_str().parse().unwrap();
        let second_pos: usize = captures.name("max").unwrap().as_str().parse().unwrap();
        let letter = captures.name("letter").unwrap().as_str();
        let password = captures.name("password").unwrap().as_str();

        if (index_eq(password, letter, first_pos) || index_eq(password, letter, second_pos)) && !(index_eq(password, letter, first_pos) && index_eq(password, letter, second_pos)) {
            valid += 1;
        }
    }
    println!("{}", valid);
}