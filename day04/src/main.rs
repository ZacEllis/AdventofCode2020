use std::{env, fs::File};
use std::io::{BufReader, BufRead};
use std::collections::HashMap;
use reformation::Reformation;

const PREFIXES: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

#[derive(Reformation, Debug)]
#[reformation(r"{year}")]
struct Year{
    year: u16,
}

#[derive(Reformation, Eq, PartialEq, Debug)]
enum Height{
    #[reformation(r"{}cm")]
    Cm(i32),
    #[reformation(r"{}in")]
    In(i32),
}

#[derive(Reformation, Debug)]
#[reformation(r"#{hex}")]
struct Colour{
    #[reformation("[0-9a-f]+")]
    hex: String,
}

#[derive(Reformation, Debug)]
#[reformation(r"{number}")]
struct PassportNo{
    #[reformation(r"[0-9]+")]
    number: String,
}


fn get_input() -> BufReader<File> {  // For easy copy-pasta
    let filename = env::args().skip(1).next().expect("Expected filename as argument");
    BufReader::new(File::open(filename).expect("Error opening file"))
}

fn is_valid_key(key: &str, val: &str) -> bool {
    match key {
        "byr" => {
            match Year::parse(val) {
                Ok(year) => year.year >= 1920 && year.year <= 2002,
                Err(_) => false
            }
        },
        "iyr" => {
            match Year::parse(val) {
                Ok(year) => year.year >= 2010 && year.year <= 2020,
                Err(_) => false
            }
        },
        "eyr" => {
            match Year::parse(val) {
                Ok(year) => year.year >= 2020 && year.year <= 2030,
                Err(_) => false
            }
        },
        "hgt" => {
            match Height::parse(val) {
                Ok(height) => {
                    match height {
                        Height::Cm(cm) => cm >= 150 && cm <= 193,
                        Height::In(inches) => inches >= 59 && inches <= 76
                    }
                },
                Err(_) => false
            }
        },
        "hcl" => {
            println!("Matching hcl with {}", val);
            match Colour::parse(val) {
                Ok(s) => s.hex.len() == 6,
                _ => false
            }
        },
        "ecl" => {
            match val {
                "amb" => true,
                "blu" => true,
                "brn" => true,
                "gry" => true,
                "grn" => true,
                "hzl" => true,
                "oth" => true,
                _ => false
            }
        },
        "pid" => {
            println!("Matching pid with {}", val);
            match PassportNo::parse(val) {
                Ok(s) => s.number.len() == 9,
                _ => false
            }
        },
        _ => {
            println!("False {} {}", key, val);
            false
        }
    }
}

fn validate_passport(passport: &String) -> usize {
    let entries = passport.split_ascii_whitespace();
    let mut hashmap = HashMap::new();
    for entry in entries {
        let mut kv_iter = entry.split(":");
        let key = kv_iter.next().unwrap();
        let val = kv_iter.next().unwrap();
        hashmap.insert(key, val);
    }
    println!("hashmap {:#?}", &hashmap);
    for prefix in PREFIXES.iter() {
        if !hashmap.contains_key(prefix) {
            println!("Doesn't contain key {}", prefix);
            return 0;
        }
        if !is_valid_key(prefix, hashmap[prefix]) {
            println!("{} - Invalid value for {}", hashmap[prefix], prefix);
            return 0;
        }
    }
    1
}

fn main() {
    let lines:Vec<String> = get_input().lines().map(|line| line.unwrap()).collect();
    let mut buffer = String::new();
    let mut valid: usize = 0;

    for line in lines {
        if line == "" {
            valid += validate_passport(&buffer);
            println!("Valid count {}", &valid);
            buffer = "".to_string();
        } else {
            buffer = format!("{} {}", buffer, line);
        }
    }

    valid += validate_passport(&buffer);

    println!("Valid: {}", &valid);
}
