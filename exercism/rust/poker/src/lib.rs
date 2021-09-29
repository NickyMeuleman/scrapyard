use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Rank {
    Num(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    fn new(input: &str) -> Self {
        match input {
            "J" => Self::Jack,
            "Q" => Self::Queen,
            "K" => Self::King,
            "A" => Self::Ace,
            _ => {
                let n = input.parse().expect("clean input data: only numbers");
                Self::Num(n)
            }
        }
    }

    fn adjacent(self, other: Self) -> bool {
        match self {
            Self::Ace => other == Self::King || other == Self::Num(2),
            Self::King => other == Self::Queen || other == Self::Ace,
            Self::Queen => other == Self::Jack || other == Self::King,
            Self::Jack => other == Self::Num(10) || other == Self::Queen,
            Self::Num(10) => other == Self::Num(9) || other == Self::Jack,
            Self::Num(2) => other == Self::Ace || other == Self::Num(3),
            Self::Num(n) => other == Self::Num(n - 1) || other == Self::Num(n + 1),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

impl Suit {
    fn new(input: &str) -> Self {
        match input {
            "H" => Self::Hearts,
            "D" => Self::Diamonds,
            "C" => Self::Clubs,
            "S" => Self::Spades,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(input: &str) -> Self {
        let (rank, suit) = input.split_at(input.len() - 1);
        Card {
            rank: Rank::new(rank),
            suit: Suit::new(suit),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandLevel {
    // high to low Rank
    HighCard(Rank, Rank, Rank, Rank, Rank),
    // pair rank, high to low rank
    OnePair(Rank, Rank, Rank, Rank),
    // highest pair rank, lowest pair rank, high to low rank
    TwoPair(Rank, Rank, Rank),
    // triple rank, high to low rank
    ThreeOfAKind(Rank, Rank, Rank),
    // highest rank
    Straight(Rank),
    // high to low rank
    Flush(Rank, Rank, Rank, Rank, Rank),
    // triple rank, pair rank, kicker rank
    FullHouse(Rank, Rank),
    // quadruple rank, kicker rank
    FourOfAKind(Rank, Rank),
    // highest rank
    StraightFlush(Rank),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Group {
    count: u8,
    rank: Rank,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(input: &str) -> Self {
        // uses Itertools::sorted
        Hand {
            cards: input
                .split_whitespace()
                .map(|s| Card::new(s))
                .sorted()
                .collect(),
        }
    }

    fn group(&self) -> Vec<Group> {
        // uses Itertools::sorted, Itertools::group_by
        // needs sorted input or group_by can make multiple groups with identical rank
        self.cards
            .iter()
            .group_by(|card| card.rank)
            .into_iter()
            .map(|(rank, group)| Group {
                count: group.count() as u8,
                rank,
            })
            // make sure groups with more cards come first, this is done by count being the first
            // field in the Group struct and it deriving Ord
            .sorted()
            .rev()
            .collect()
    }

    fn level(&self) -> HandLevel {
        // TODO: rewrite without nesting match statements like a christmas tree
        let groups = self.group();
        let straight = self.is_straight();
        let flush = self.is_flush();
        if straight && flush {
            HandLevel::StraightFlush(groups[0].rank)
        } else if groups[0].count == 4 {
            HandLevel::FourOfAKind(groups[0].rank, groups[1].rank)
        } else if groups[0].count == 3 && groups[1].count == 2 {
            HandLevel::FullHouse(groups[0].rank, groups[1].rank)
        } else if flush {
            HandLevel::Flush(
                groups[0].rank,
                groups[1].rank,
                groups[2].rank,
                groups[3].rank,
                groups[4].rank,
            )
        } else if straight {
            if groups[0].rank == Rank::Ace && self.cards[3].rank == Rank::Num(5) {
                HandLevel::Straight(Rank::Num(5))
            } else {
                HandLevel::Straight(groups[0].rank)
            }
        } else if groups[0].count == 3 {
            HandLevel::ThreeOfAKind(groups[0].rank, groups[1].rank, groups[2].rank)
        } else if groups[0].count == 2 && groups[1].count == 2 {
            HandLevel::TwoPair(groups[0].rank, groups[1].rank, groups[2].rank)
        } else if groups[0].count == 2 {
            HandLevel::OnePair(
                groups[0].rank,
                groups[1].rank,
                groups[2].rank,
                groups[3].rank,
            )
        } else {
            HandLevel::HighCard(
                groups[0].rank,
                groups[1].rank,
                groups[2].rank,
                groups[3].rank,
                groups[4].rank,
            )
        }
    }

    fn is_flush(&self) -> bool {
        let suit = &self.cards[0].suit;
        self.cards.iter().all(|card| card.suit == *suit)
    }

    fn is_straight(&self) -> bool {
        let has_low_ace = self.cards[4].rank == Rank::Ace && self.cards[0].rank == Rank::Num(2);
        self.cards
            .windows(2)
            .take(if has_low_ace { 3 } else { 4 })
            .all(|window| window[0].rank.adjacent(window[1].rank))
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let iter = hands.iter().map(|&s| (s, Hand::new(s).level()));
    let max = iter.clone().map(|(_, level)| level).max().unwrap();
    iter.filter_map(|(s, level)| if level == max { Some(s) } else { None })
        .collect()
}
