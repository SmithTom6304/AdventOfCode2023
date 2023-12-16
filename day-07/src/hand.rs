use super::card::Card;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hand {
    cards: [Card; 5],
    score: HandScore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandScore {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        let score = HandScore::from(cards);
        Hand { score, cards }
    }

    pub fn cards(&self) -> &[Card; 5] {
        &self.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.score.cmp(&other.score) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        for card_pair in self.cards.iter().zip(other.cards.iter()) {
            match card_pair.0.cmp(card_pair.1) {
                core::cmp::Ordering::Equal => {}
                ord => return ord,
            }
        }
        core::cmp::Ordering::Equal
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

impl From<[Card; 5]> for HandScore {
    fn from(value: [Card; 5]) -> Self {
        todo!()
    }
}
