use std::{collections::HashMap, env, fs};

use day_04::card::Card;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    println!("Reading from file '{}'", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut cards = HashMap::new();
    contents.lines().into_iter().for_each(|line: &str| {
        let card = Card::from_line(line);
        _ = cards.insert(card.id, card)
    });

    let mut total_cards: Vec<&Card> = Vec::new();
    for card in cards.values() {
        get_scratchcards(card, &cards)
            .iter()
            .for_each(|&c| _ = total_cards.push(c));
    }
    println!("Result = {}", total_cards.len());
}

fn get_scratchcards<'a>(card: &'a Card, cards: &'a HashMap<u8, Card>) -> Vec<&'a Card> {
    let mut total_cards = vec![];
    total_cards.push(card);
    let matches = card.get_no_of_winning_numbers();

    let mut i = 0;
    while i < matches {
        i += 1;
        let copied_card = cards
            .get(&(card.id + i))
            .expect("Id exceeded past end of table");
        total_cards.append(&mut get_scratchcards(copied_card, cards));
    }

    total_cards
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::Card;

    fn cards() -> HashMap<u8, Card> {
        let mut cards = HashMap::new();
        let card = Card::from_line("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        cards.insert(card.id, card);
        let card = Card::from_line("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19");
        cards.insert(card.id, card);
        let card = Card::from_line("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1");
        cards.insert(card.id, card);
        let card = Card::from_line("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83");
        cards.insert(card.id, card);
        let card = Card::from_line("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36");
        cards.insert(card.id, card);
        let card = Card::from_line("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        cards.insert(card.id, card);

        cards
    }

    #[test]
    fn can_get_scratchcards() {
        let cards = cards();
        let mut total_cards: Vec<&Card> = Vec::new();
        for card in cards.values() {
            super::get_scratchcards(card, &cards)
                .iter()
                .for_each(|&c| _ = total_cards.push(c));
        }
        assert_eq!(30, total_cards.len());
    }
}
