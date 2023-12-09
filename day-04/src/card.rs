use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Card {
    pub id: u8,
    pub winning_numbers: HashSet<u8>,
    pub player_numbers: HashSet<u8>,
}

impl Card {
    pub fn from_line(line: &str) -> Self {
        let mut line = &line[4..]; // Remove 'Card'
        line = line.trim_start();
        let colon_index = line.find(':').expect("Colon not found");
        let pipe_index = line.find('|').expect("Pipe not found");

        let id = &line[..colon_index];
        let id = id.parse().expect("Could not parse ID");

        let winning_numbers = &line[colon_index + 1..pipe_index].trim();
        let winning_numbers = winning_numbers
            .split(' ')
            .filter(|val| !val.is_empty())
            .map(|val| val.parse::<u8>().expect("Could not parse winning number"))
            .collect();

        let player_numbers = &line[pipe_index + 1..].trim();
        let player_numbers = player_numbers
            .split(' ')
            .filter(|val| !val.is_empty())
            .map(|val| val.parse::<u8>().expect("Could not parse player number"))
            .collect();

        Card {
            id,
            winning_numbers,
            player_numbers,
        }
    }
}

#[cfg(test)]
mod a_card {
    use std::collections::HashSet;

    use super::Card;

    #[test]
    fn can_be_created_from_line() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let mut winning_numbers = HashSet::new();
        winning_numbers.insert(41);
        winning_numbers.insert(48);
        winning_numbers.insert(83);
        winning_numbers.insert(86);
        winning_numbers.insert(17);

        let mut player_numbers = HashSet::new();
        player_numbers.insert(83);
        player_numbers.insert(86);
        player_numbers.insert(6);
        player_numbers.insert(31);
        player_numbers.insert(17);
        player_numbers.insert(9);
        player_numbers.insert(48);
        player_numbers.insert(53);

        let expected_card = Card {
            id: 1,
            winning_numbers,
            player_numbers,
        };
        let actual_card = Card::from_line(line);

        assert_eq!(expected_card, actual_card);
    }
}
