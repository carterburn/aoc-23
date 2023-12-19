use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric0, i32, multispace0},
    multi::many0,
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};

use std::error::Error;

use std::cmp::Ordering;

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct InvalidHandInput;

impl std::fmt::Display for InvalidHandInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid hand!")
    }
}

/// Helper method that splits up a &str into its logical chunks (i.e.: will return a vec with its
/// characters and the count in the string
fn chunk_string(input: &str) -> HashMap<char, usize> {
    let mut m = HashMap::new();
    for c in input.chars() {
        match m.get_mut(&c) {
            None => {
                let _ = m.insert(c.clone(), 1);
            }
            Some(count) => {
                *count += 1;
            }
        }
    }
    m
}

#[derive(Debug, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

/// Custom ord for HandType that leverages an internal value for the comparison
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_value = self.card_value();
        let other_value = other.card_value();

        self_value.cmp(&other_value)
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self.card_value() == other.card_value()
    }
}

impl HandType {
    /// Creates a new HandType based on the provided card
    pub fn new(card: &str) -> Self {
        let m = chunk_string(card);

        // this will operate on references to not move values out of m
        match m.values().len() {
            1 => {
                // only one card in the map; five of a kind
                HandType::FiveOfAKind
            }
            2 => {
                // either a four of a kind or a full house, based on the values
                let v = m.values().collect::<Vec<&usize>>();
                let candidate = match v.get(0) {
                    Some(v) => v,
                    None => &&0,
                };
                if candidate == &&4 || candidate == &&1 {
                    // in a four of a kind, one card will have 4 appearances and one will have 1
                    HandType::FourOfAKind
                } else {
                    HandType::FullHouse
                }
            }
            3 => {
                // either a three of a kind or a two pair; need to take a look of them all
                let v = m.values().collect::<Vec<&usize>>();
                let first = match v.get(0) {
                    Some(v) => v,
                    None => &&0,
                };
                let second = match v.get(1) {
                    Some(v) => v,
                    None => &&0,
                };
                let third = match v.get(2) {
                    Some(v) => v,
                    None => &&0,
                };

                if first == &&2 || second == &&2 || third == &&2 {
                    // if any of the cards have 2 matches, it's a two pair
                    HandType::TwoPair
                } else {
                    HandType::ThreeOfAKind
                }
            }
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    /// Gives a value that ranks the hands from lowest to highest
    pub fn card_value(&self) -> i32 {
        match self {
            Self::FiveOfAKind => 6,
            Self::FourOfAKind => 5,
            Self::FullHouse => 4,
            Self::ThreeOfAKind => 3,
            Self::TwoPair => 2,
            Self::OnePair => 1,
            Self::HighCard => 0,
        }
    }
}

#[derive(Debug, Eq)]
struct Hand<'a> {
    cards: &'a str,
    hand_type: HandType,
    bid: i32,
}

impl<'a> Hand<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (remain, (cards, bid)) = separated_pair(alphanumeric0, tag(" "), i32)(input)?;
        Ok((
            remain,
            Self {
                cards,
                hand_type: HandType::new(cards),
                bid,
            },
        ))
    }

    pub fn with_joker(&mut self) {
        // get a string chunking again for decision making on upgrades
        let chunks = chunk_string(self.cards);
        let num_jokers = match chunks.get(&'J') {
            None => {
                return;
            }
            Some(v) => *v,
        };

        // if there is a joker, try to upgrade the hand type
        self.hand_type = match self.hand_type {
            HandType::FiveOfAKind => {
                // still have a five of a kind
                HandType::FiveOfAKind
            }
            HandType::FourOfAKind => {
                // can always upgrade to a five of a kind
                HandType::FiveOfAKind
            }
            HandType::FullHouse => {
                // either situations give a five of a kind (either have 3 jokers 2 others to
                // upgrade to 5 or 2 jokers 3 others to upgrade to 5)
                HandType::FiveOfAKind
            }
            HandType::ThreeOfAKind => {
                // three of a kind means we can use the joker to upgrade to a foure of a kind (no
                // matter how many jokers; either have 3 jokers that can turn to 3 of one of the
                // remaining or 1 joker that can be either or)
                HandType::FourOfAKind
            }
            HandType::TwoPair => {
                if num_jokers == 2 {
                    // 2 jokers can move to the other pair and become a four of a kind
                    HandType::FourOfAKind
                } else {
                    // 1 joker can upgrade one of the two pairs to make the whole thing a fullhouse
                    HandType::FullHouse
                }
            }
            HandType::OnePair => {
                // the joker may be the pair (so you can match another one of the leftovers for
                // three) or the joker is alone and NOT the pair, so it can match the pair for a
                // three of a kind
                HandType::ThreeOfAKind
            }
            HandType::HighCard => {
                // best you can do is turn the joker into a pair
                HandType::OnePair
            }
        };
    }
}

impl<'a> Ord for Hand<'a> {
    /// Compare two cards to one another. A card is "less" if it is "weaker" than other.
    /// For example, if self is a hand with one pair and other is a hand with two pair, then
    /// self will be Less than other. If self is a three of a kind and other is a two pair, then
    /// self is Greater than other.
    fn cmp(&self, other: &Self) -> Ordering {
        // check if there are different HandTypes; if yes, go based on that
        if self.hand_type.cmp(&other.hand_type).is_ne() {
            return self.hand_type.cmp(&other.hand_type);
        }

        // otherwise, need to compare the cards one by one
        let scards = self.cards.chars().collect::<Vec<char>>();
        let ocards = other.cards.chars().collect::<Vec<char>>();
        // define the order we care about
        let char_order = [
            'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
        ];

        for (sc, oc) in scards.iter().zip(ocards.iter()) {
            // same cards don't matter
            if sc == oc {
                continue;
            }

            // get the indices of the two cards, compare those
            let scind = match char_order.iter().position(|&p| p == *sc) {
                None => {
                    return Ordering::Less;
                }
                Some(s) => s,
            };
            let ocind = match char_order.iter().position(|&p| p == *oc) {
                None => {
                    return Ordering::Greater;
                }
                Some(o) => o,
            };
            return scind.cmp(&ocind);
        }
        // based on the input, can't get here; but need it because rust
        Ordering::Equal
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

#[derive(Debug)]
struct CamelCards<'a> {
    cards: Vec<Hand<'a>>,
}

impl<'a> CamelCards<'a> {
    pub fn parse(input: &'a str) -> IResult<&str, Self> {
        let (remain, mut cards) = many0(preceded(multispace0, Hand::parse))(input)?;
        // sort immediately
        //cards.sort();
        Ok((remain, Self { cards }))
    }

    pub fn total_winnings(&self) -> i32 {
        self.cards
            .iter()
            .enumerate()
            .map(|(rank, hand)| ((rank + 1) as i32) * hand.bid)
            .sum()
    }

    pub fn with_joker(&mut self) -> i32 {
        // transform each card with a joker if applicable
        for c in self.cards.iter_mut() {
            c.with_joker();
        }
        // sort the cards based on updates
        self.cards.sort();
        // compute new value
        self.total_winnings()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    // begin by parsing the cards and their bids
    let (_remaining, mut camel_cards) = CamelCards::parse(input)?;

    let p2 = camel_cards.with_joker();
    println!("{:?}", camel_cards.cards);

    println!("P2: {p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_string() {
        let input = "AATKQ";
        let map = chunk_string(input);

        assert_eq!(map[&'A'], 2);
        assert_eq!(map[&'T'], 1);
        assert_eq!(map[&'K'], 1);
        assert_eq!(map[&'Q'], 1);
    }

    #[test]
    fn test_hand_type() {
        let five = "AAAAA";
        let four = "AA8AA";
        let full = "23332";
        let three = "TTT98";
        let two = "23432";
        let one = "A23A4";
        let high = "23456";

        assert_eq!(HandType::new(five), HandType::FiveOfAKind);
        assert_eq!(HandType::new(four), HandType::FourOfAKind);
        assert_eq!(HandType::new(full), HandType::FullHouse);
        assert_eq!(HandType::new(three), HandType::ThreeOfAKind);
        assert_eq!(HandType::new(two), HandType::TwoPair);
        assert_eq!(HandType::new(one), HandType::OnePair);
        assert_eq!(HandType::new(high), HandType::HighCard);
    }

    #[test]
    fn test_card_sorting() {
        let input = include_str!("../test.txt");

        let (_remaining, mut camel_cards) = CamelCards::parse(input).unwrap();

        camel_cards.cards.sort();

        // first card should be a one pair ("32T3K")
        assert_eq!(
            camel_cards.cards.get(0).unwrap().hand_type,
            HandType::OnePair
        );
        assert_eq!(camel_cards.cards.get(0).unwrap().cards, "32T3K");
        // second card should be a two pair ("KTJJT")
        assert_eq!(
            camel_cards.cards.get(1).unwrap().hand_type,
            HandType::TwoPair
        );
        assert_eq!(camel_cards.cards.get(1).unwrap().cards, "KTJJT");
        // third card should be a two pair ("KK677")
        assert_eq!(
            camel_cards.cards.get(2).unwrap().hand_type,
            HandType::TwoPair
        );
        assert_eq!(camel_cards.cards.get(2).unwrap().cards, "KK677");
        // fourth card should be a three of a kind ("T55J5")
        assert_eq!(
            camel_cards.cards.get(3).unwrap().hand_type,
            HandType::ThreeOfAKind
        );
        assert_eq!(camel_cards.cards.get(3).unwrap().cards, "T55J5");
        // third card should be a two pair ("QQQJA")
        assert_eq!(
            camel_cards.cards.get(4).unwrap().hand_type,
            HandType::ThreeOfAKind
        );
        assert_eq!(camel_cards.cards.get(4).unwrap().cards, "QQQJA");
    }
}
