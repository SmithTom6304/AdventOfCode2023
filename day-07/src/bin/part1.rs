use std::{env, fs};

use day_07::play::Play;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Reading from file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let plays: Vec<Play> = match contents
        .lines()
        .map(|input| Play::try_from(input))
        .collect()
    {
        Ok(play) => play,
        Err(err) => panic!("{}", err),
    };

    let result = calculate_sum_of_scores(plays);
    println!("Result = {}", result);
}

fn calculate_sum_of_scores(mut plays: Vec<Play>) -> u32 {
    plays.sort();
    let mut rank = 0;
    let mut sum = 0;
    let mut iter = plays.into_iter();
    while let Some(play) = iter.next() {
        rank += 1;
        sum += play.bid() * rank;
    }
    sum
}

#[cfg(test)]
mod tests {
    use day_07::play::Play;

    use crate::calculate_sum_of_scores;

    #[test]
    fn example() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let input = input.split('\n');
        let mut plays: Vec<Play> = match input.map(|input| Play::try_from(input)).collect() {
            Ok(play) => play,
            Err(err) => panic!("{}", err),
        };

        let sum = calculate_sum_of_scores(plays);

        assert_eq!(6440, sum);
    }
}
