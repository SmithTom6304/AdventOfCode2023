use super::card::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Hand { cards }
    }

    pub fn cards(&self) -> &[Card; 5] {
        &self.cards
    }
}

impl TryFrom<[char; 5]> for Hand {
    type Error = String;
    fn try_from(value: [char; 5]) -> Result<Self, Self::Error> {
        let cards: Vec<Card> = match value.into_iter().map(|val| Card::try_from(val)).collect() {
            Ok(card) => card,
            Err(err) => return Err(err),
        };
        let cards: [Card; 5] = match cards.try_into() {
            Ok(cards) => cards,
            Err(err) => {
                return Err(format!(
                    "Failed converting vector of cards {:?} into card array",
                    err
                ))
            }
        };
        Ok(Hand::new(cards))
    }
}
