use std::collections::HashMap;
use std::hash::Hash;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::card::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandScore {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

impl HandScore {
    pub fn from(value: [Card; 5]) -> Self {
        let pattern = PatternValue::get_pattern_values(value);
        HandScore::from_pattern(pattern)
    }

    fn from_pattern(pattern: Pattern) -> Self {
        Pattern::pattern_handscore_map()
            .get(&pattern)
            .expect(&format!(
                "No handscore for provided pattern {:?} found",
                pattern
            ))
            .clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pattern([PatternValue; 5]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Hash)]
enum PatternValue {
    CardA,
    CardB,
    CardC,
    CardD,
    CardE,
}

impl Pattern {
    fn pattern_handscore_map() -> HashMap<Pattern, HandScore> {
        let five_of_a_kind_pattern = Pattern([PatternValue::CardA; 5]);
        let four_of_a_kind_pattern = Pattern([
            PatternValue::CardA,
            PatternValue::CardA,
            PatternValue::CardA,
            PatternValue::CardA,
            PatternValue::CardB,
        ]);
        let full_house_pattern = Pattern([
            PatternValue::CardA,
            PatternValue::CardA,
            PatternValue::CardA,
            PatternValue::CardB,
            PatternValue::CardB,
        ]);
        let three_of_a_kind_pattern = Pattern([
            PatternValue::CardA,
            PatternValue::CardA,
            PatternValue::CardA,
            PatternValue::CardB,
            PatternValue::CardC,
        ]);
        let two_pair_pattern = Pattern([
            PatternValue::CardA,
            PatternValue::CardA,
            PatternValue::CardB,
            PatternValue::CardB,
            PatternValue::CardC,
        ]);
        let one_pair_pattern = Pattern([
            PatternValue::CardA,
            PatternValue::CardA,
            PatternValue::CardB,
            PatternValue::CardC,
            PatternValue::CardD,
        ]);
        let high_card_pattern = Pattern([
            PatternValue::CardA,
            PatternValue::CardB,
            PatternValue::CardC,
            PatternValue::CardD,
            PatternValue::CardE,
        ]);
        let mut pattern_handscore_map = HashMap::<Pattern, HandScore>::new();
        pattern_handscore_map.insert(five_of_a_kind_pattern, HandScore::FiveOfAKind);
        pattern_handscore_map.insert(four_of_a_kind_pattern, HandScore::FourOfAKind);
        pattern_handscore_map.insert(full_house_pattern, HandScore::FullHouse);
        pattern_handscore_map.insert(three_of_a_kind_pattern, HandScore::ThreeOfAKind);
        pattern_handscore_map.insert(two_pair_pattern, HandScore::TwoPair);
        pattern_handscore_map.insert(one_pair_pattern, HandScore::OnePair);
        pattern_handscore_map.insert(high_card_pattern, HandScore::HighCard);
        pattern_handscore_map
    }
}

impl Hash for Pattern {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for pattern in self.0 {
            pattern.hash(state);
        }
    }
}

impl PatternValue {
    fn get_pattern_values(value: [Card; 5]) -> Pattern {
        let sorted_cards = Card::sort_by_occurences(value);
        let mut patterns = vec![];
        let mut card_iter = sorted_cards.into_iter();
        let mut pattern_iter = PatternValue::iter();
        let mut current_pattern = pattern_iter.next().unwrap();
        // Always push CardA for first value
        patterns.push(current_pattern.clone());
        let mut last_card = card_iter.next().expect("Expected card");
        while let Some(card) = card_iter.next() {
            match card == last_card {
                true => {}
                false => current_pattern = pattern_iter.next().unwrap(),
            }
            patterns.push(current_pattern);
            last_card = card;
        }
        let pattern_values = patterns
            .try_into()
            .expect("Error creating patter value array");
        Pattern(pattern_values)
    }
}

impl Card {
    fn sort_by_occurences(cards: [Card; 5]) -> [Card; 5] {
        let mut occurences = HashMap::<Card, u8>::new();
        for card in cards {
            match occurences.get_key_value(&card) {
                Some(kvp) => _ = occurences.insert(card, kvp.1 + 1),
                None => _ = occurences.insert(card, 1),
            }
        }
        let mut occurence_pairs: Vec<(Card, u8)> = occurences.into_iter().collect();
        occurence_pairs.sort_by(|a, b| a.1.cmp(&b.1));
        occurence_pairs.reverse();
        let mut reconstructed_cards = vec![];
        for card in occurence_pairs.into_iter() {
            let mut occurences = card.1.clone();
            while occurences > 0 {
                reconstructed_cards.push(card.0);
                occurences -= 1;
            }
        }
        let cards: [Card; 5] = reconstructed_cards
            .try_into()
            .expect("Error creating new Card array while sorting by occurences");

        cards
    }
}
