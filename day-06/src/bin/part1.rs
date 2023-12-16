use day_06::operations::*;
use std::{env, fs, iter::Map, ops::Range};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Reading from file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lines = contents.lines();

    // Get times
    let times = lines.next().expect("Input had no times line");
    let times = values_from_line(times["Time:".len()..].trim());
    let distances = lines.next().expect("Input had no distances line");
    let distances = values_from_line(distances["Distance:".len()..].trim());
    let races = times.iter().zip(distances.iter());
    let result: u32 = races
        .map(|race| distances_greater_than_record(*race.0 as u64, *race.1 as u64).count() as u32)
        .product();

    println!("Result = {}", result);
}

fn values_from_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .filter(move |val| !val.is_empty())
        .map(move |str| str.parse::<u32>().expect("Failed parsing value as u32"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_distances_greater_than_record() {
        let times = "Time:      7  15   30";
        let distances = "Distance:  9  40  200";

        let times = values_from_line(times["Time:".len()..].trim());
        let distances = values_from_line(distances["Distance:".len()..].trim());
        let races = times.iter().zip(distances.iter());
        let mut ways_to_win_iter =
            races.map(|race| distances_greater_than_record(*race.0 as u64, *race.1 as u64));
        let race_1_ways_to_win = ways_to_win_iter.next().expect("Expected result for race 1");
        let race_2_ways_to_win = ways_to_win_iter.next().expect("Expected result for race 2");
        let race_3_ways_to_win = ways_to_win_iter.next().expect("Expected result for race 3");
        assert_eq!(4, race_1_ways_to_win.count());
        assert_eq!(8, race_2_ways_to_win.count());
        assert_eq!(9, race_3_ways_to_win.count());
    }
}
