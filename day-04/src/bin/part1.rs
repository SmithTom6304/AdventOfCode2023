use std::{env, fs};

use day_04::card::Card;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Reading from file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut sum = 0;
    contents
        .lines()
        .into_iter()
        .for_each(|line| sum += Card::from_line(line).get_score());
    println!("Result = {}", sum);
}
