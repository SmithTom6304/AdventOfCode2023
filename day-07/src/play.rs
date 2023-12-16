use super::hand::Hand;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Play {
    hand: Hand,
    bid: u32,
}

impl Play {
    pub fn new(hand: Hand, bid: u32) -> Self {
        Play { hand, bid }
    }

    pub fn hand(&self) -> &Hand {
        &self.hand
    }

    pub fn bid(&self) -> &u32 {
        &self.bid
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl TryFrom<&str> for Play {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let hand: [char; 5] = match value[0..5].chars().collect::<Vec<char>>().try_into() {
            Ok(hand) => hand,
            Err(err) => {
                return Err(format!(
                    "Failed converting vector of chars {:?} into char array",
                    err
                ))
            }
        };
        let hand = match Hand::try_from(hand) {
            Ok(hand) => hand,
            Err(err) => return Err(err),
        };
        let bid = match value[6..].parse::<u32>() {
            Ok(bid) => bid,
            Err(err) => return Err(format!("Failed to parse bid {}, {}", &value[6..], err)),
        };
        Ok(Play::new(hand, bid))
    }
}
