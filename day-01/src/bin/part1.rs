use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Reading from file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut sum = 0;
    contents
        .lines()
        .into_iter()
        .for_each(|input| sum += combine_digits(input));
    println!("Result = {}", sum);
}

fn combine_digits(input: &str) -> u32 {
    let first_character = input
        .chars()
        .find(|&c| c.is_numeric())
        .expect("Input should contain a number")
        .to_digit(10)
        .unwrap();
    let last_character = input
        .chars()
        .rev()
        .find(|&c| c.is_numeric())
        .expect("Input should contain a number")
        .to_digit(10)
        .unwrap();
    return (first_character * 10) + last_character;
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
