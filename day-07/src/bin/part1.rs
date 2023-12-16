fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use day_07::play::Play;

    #[test]
    fn example() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        let input = input.split('\n');
        let mut plays: Vec<Play> = match input.map(|input| Play::try_from(input)).collect() {
            Ok(play) => play,
            Err(err) => panic!("{}", err),
        };
        plays.sort();
        let mut rank = 0;
        let mut sum = 0;
        let mut iter = plays.into_iter();
        while let Some(play) = iter.next() {
            rank += 1;
            sum += play.bid() * rank;
        }

        assert_eq!(6440, sum);
    }
}
