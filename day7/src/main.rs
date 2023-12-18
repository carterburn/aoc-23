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

    /// Converts a card to a numerical representation for easier comparison
    pub fn card_value(&self) -> i32 {
        // assign a card value to each of the cards, a larger number means a 'stronger' card (or >)
        self.cards.chars().fold(0, |mut acc, x| {
            let cval = match x {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0,
            };
            // multiply by 100 to ensure we have distinct values
            acc = acc * 100 + cval;
            acc
        })
    }
}

impl<'a> Ord for Hand<'a> {
    /// Compare two cards to one another. A card is "less" if it is "weaker" than other.
    /// For example, if self is a hand with one pair and other is a hand with two pair, then
    /// self will be Less than other. If self is a three of a kind and other is a two pair, then
    /// self is Greater than other.
    fn cmp(&self, other: &Self) -> Ordering {
        // check if there are different HandTypes; if yes, go based on that
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }

        // otherwise, need to compare the cards one by one
        let scard = self.card_value();
        let ocard = other.card_value();
        scard.cmp(&ocard)
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> PartialEq for Hand<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.card_value() == other.card_value()
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
        cards.sort();
        Ok((remain, Self { cards }))
    }

    pub fn total_winnings(&self) -> i32 {
        self.cards
            .iter()
            .enumerate()
            .map(|(rank, hand)| ((rank + 1) as i32) * hand.bid)
            .sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");

    // begin by parsing the cards and their bids
    let (_remaining, camel_cards) = CamelCards::parse(input)?;

    let p1 = camel_cards.total_winnings();

    println!("P1: {p1}");

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
