use std::{iter::Map, ops::Range};

fn main() {
    println!("Hello, world!");
}

fn distance(hold_time: u32, total_time: u32) -> u32 {
    let move_time = total_time - hold_time;
    let speed = hold_time;
    speed * move_time
}

fn iterate_distances(total_time: u32) -> impl Iterator<Item = u32> {
    (0..total_time).map(move |hold_time| distance(hold_time, total_time))
}

fn distances_greater_than_record(total_time: u32, record_distance: u32) -> impl Iterator {
    iterate_distances(total_time).filter(move |distance| *distance > record_distance)
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
            races.map(|race| distances_greater_than_record(*race.0, *race.1));
        let race_1_ways_to_win = ways_to_win_iter.next().expect("Expected result for race 1");
        let race_2_ways_to_win = ways_to_win_iter.next().expect("Expected result for race 2");
        let race_3_ways_to_win = ways_to_win_iter.next().expect("Expected result for race 3");
        assert_eq!(4, race_1_ways_to_win.count());
        assert_eq!(8, race_2_ways_to_win.count());
        assert_eq!(9, race_3_ways_to_win.count());
    }
}
