use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

impl Card {
    fn face_cards_map() -> HashMap<char, Card> {
        let mut face_cards_map = HashMap::<char, Card>::new();
        face_cards_map.insert('A', Card::Ace);
        face_cards_map.insert('K', Card::King);
        face_cards_map.insert('Q', Card::Queen);
        face_cards_map.insert('J', Card::Jack);
        face_cards_map.insert('T', Card::Ten);
        face_cards_map
    }

    fn number_cards_map() -> HashMap<char, Card> {
        let mut number_cards_map = HashMap::<char, Card>::new();
        number_cards_map.insert('9', Card::Nine);
        number_cards_map.insert('8', Card::Eight);
        number_cards_map.insert('7', Card::Seven);
        number_cards_map.insert('6', Card::Six);
        number_cards_map.insert('5', Card::Five);
        number_cards_map.insert('4', Card::Four);
        number_cards_map.insert('3', Card::Three);
        number_cards_map.insert('2', Card::Two);
        number_cards_map
    }
}

impl TryFrom<char> for Card {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Some(card) = Self::face_cards_map().get(&value) {
            return Ok(card.clone());
        } else if let Some(card) = Self::number_cards_map().get(&value) {
            return Ok(card.clone());
        }
        Err(format!("Card can not be created from char {}", value))
    }
}

#[cfg(test)]
mod a_card {
    use super::Card;
    use rstest::rstest;

    #[rstest]
    #[case('A', Card::Ace)]
    #[case('K', Card::King)]
    #[case('Q', Card::Queen)]
    #[case('J', Card::Jack)]
    #[case('T', Card::Ten)]
    #[case('9', Card::Nine)]
    #[case('8', Card::Eight)]
    #[case('7', Card::Seven)]
    #[case('6', Card::Six)]
    #[case('5', Card::Five)]
    #[case('4', Card::Four)]
    #[case('3', Card::Three)]
    #[case('2', Card::Two)]
    fn can_be_created_from_valid_chars(#[case] value: char, #[case] expected_card: Card) {
        let card = match Card::try_from(value) {
            Ok(val) => val,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(expected_card, card);
    }

    #[rstest]
    #[case('+')]
    #[case('Y')]
    #[case('1')]
    #[case('0')]
    fn returns_error_for_invalid_char(#[case] value: char) {
        assert!(Card::try_from(value).is_err())
    }
}
