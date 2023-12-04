fn main() {
    println!("Hello, world!");
}

fn combine_digits(input: &str) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::combine_digits;
    use rstest::rstest;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn it_works(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(expected, combine_digits(input));
    }
}
