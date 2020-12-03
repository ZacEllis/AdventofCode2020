use std::{env, fs::File};
use std::io::{BufReader, BufRead};

const INC: usize = 1;

fn get_input() -> BufReader<File> {  // For easy copy-pasta
    let filename = env::args().skip(1).next().expect("Expected filename as argument");
    BufReader::new(File::open(filename).expect("Error opening file"))
}

fn is_tree(mut line: String, offset: usize) -> bool {
    while line.chars().count() <= offset {
        line = format!("{}{}", line, line);
    }
    line.as_bytes()[offset] == "#".as_bytes()[0]
}

fn main() {
    let input = get_input().lines();
    let mut linecount: usize = 0;
    let mut offset: usize = 0;
    let mut treecount = 0;
    for raw_line in input {
        let line = raw_line.unwrap();
        if linecount % 2 == 0 {
            if is_tree(line, offset) == true {
                treecount += 1;
            }
            offset += INC;
        }
        linecount += 1;
    }
    println!("down 2 Offset: {}, treecount: {}", INC, treecount);
}