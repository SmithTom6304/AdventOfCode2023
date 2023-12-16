use std::{env, fs};

use day_06::operations::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Reading from file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lines = contents.lines();

    // Get times
    let time = lines.next().expect("Input had no times line");
    let time = value_from_line(time["Time:".len()..].trim());
    let distance = lines.next().expect("Input had no distances line");
    let distance = value_from_line(distance["Distance:".len()..].trim());
    let race = (time, distance);
    let result = distances_greater_than_record(race.0, race.1).count();

    println!("Result = {}", result);
}

fn value_from_line(line: &str) -> u64 {
    line.replace(" ", "")
        .parse::<u64>()
        .expect("Error parsing value from line")
}
