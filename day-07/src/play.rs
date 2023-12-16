use super::hand::Hand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Play {
    hand: Hand,
    bet: u32,
}

impl Play {
    pub fn new(hand: Hand, bet: u32) -> Self {
        Play { hand, bet }
    }

    pub fn hand(&self) -> &Hand {
        &self.hand
    }

    pub fn bet(&self) -> &u32 {
        &self.bet
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}
