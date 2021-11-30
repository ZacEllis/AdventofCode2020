use std::{env, fs::File};
use std::io::{BufReader, BufRead};

fn get_input() -> BufReader<File> {  // For easy copy-pasta
    let filename = env::args().skip(1).next().expect("Expected filename as argument");
    BufReader::new(File::open(filename).expect("Error opening file"))
}

fn halve_range(range: Vec<i32>, side: u8) -> Vec<i32> {
    if side == "F".as_bytes()[0] || side == "L".as_bytes()[0] {
        return range[0..range.len()/2 + 1].to_owned();
    } else {
        return range[(range.len()/2)..range.len()].to_owned();
    }
}

fn parse_line(line: String) -> i32 {
    let mut rows: Vec<i32> = (0..128).collect();
    rows = halve_range(rows, line.as_bytes()[0]);
    rows = halve_range(rows, line.as_bytes()[1]);
    rows = halve_range(rows, line.as_bytes()[2]);
    rows = halve_range(rows, line.as_bytes()[3]);
    rows = halve_range(rows, line.as_bytes()[4]);
    rows = halve_range(rows, line.as_bytes()[5]);
    rows = halve_range(rows, line.as_bytes()[6]);

    let mut columns: Vec<i32> = (0..8).collect();
    columns = halve_range(columns, line.as_bytes()[7]);
    columns = halve_range(columns, line.as_bytes()[8]);
    columns = halve_range(columns, line.as_bytes()[9]);

    let seat_id = rows[0] * 8 + columns[0];
    println!("{} - Row: {:?}, Column: {:?}, Seat ID: {}", line, rows, columns, seat_id);
    seat_id
}

fn main() {
    println!("{}", parse_line("FBFBBFFRLR".to_string()));
    println!("{:#?}", halve_range(vec![0,1], "G".as_bytes()[0]));
    let input = get_input().lines();
    let mut highest_id = 0;
    let mut unseen_ids: Vec<i32> = (0..819).collect();
    for line in input {
        let id = parse_line(line.unwrap());
        unseen_ids.remove(unseen_ids.iter().position(|x| *x == id).expect("Val not in vec"));
        if id > highest_id {
            highest_id = id;
        }
    }
    println!("Highest ID: {}, missing seats: {:?}", highest_id, unseen_ids);

    
}
