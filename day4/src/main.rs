use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Clone)]
struct CardSet {
    cards: HashSet<i64>,
}

impl CardSet {
    fn new(cards: &str) -> Result<Self, Box<dyn Error>> {
        let cards = cards.trim();
        let mut set = HashSet::with_capacity(cards.split_whitespace().collect::<Vec<_>>().len());
        for num in cards.split_whitespace() {
            set.insert(num.parse::<i64>()?);
        }
        Ok(Self { cards: set })
    }

    fn get_set(&self) -> &HashSet<i64> {
        &self.cards
    }
}

#[derive(Debug, Clone)]
struct CardCounter {
    cards: HashMap<usize, usize>,
}

impl CardCounter {
    fn new() -> Self {
        Self {
            cards: HashMap::new(),
        }
    }

    fn init_card(&mut self, card: usize) {
        if let Some(v) = self.cards.get_mut(&card) {
            *v += 1;
        } else {
            self.cards.insert(card, 1);
        }
    }

    fn add_copy(&mut self, card: usize, copy: usize) {
        let x = match self.cards.get(&card) {
            // return 0 if the card hasn't been visited yet
            None => 0,
            // use stored card value
            Some(v) => *v,
        };

        // if we don't have the card already, insert it with the current card's value, otherwise
        // add to the value currently there
        match self.cards.get_mut(&(card + 1 + copy)) {
            None => {
                self.cards.insert(card + 1 + copy, x);
            }
            Some(cval) => {
                *cval += x;
            }
        }
    }

    fn get_cards(&self) -> &HashMap<usize, usize> {
        &self.cards
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    //let input = include_str!("../test1.txt");
    let input = include_str!("../input.txt");

    let mut winning_sum = 0;

    let mut cc = CardCounter::new();

    for (i, round) in input.lines().enumerate() {
        let l: Vec<&str> = round.split(':').collect();
        let cards = l[1];
        let c: Vec<&str> = cards.split('|').collect();
        let winners = CardSet::new(c[0])?;
        let ours = CardSet::new(c[1])?;

        cc.init_card(i);

        let inter = winners
            .get_set()
            .intersection(ours.get_set())
            .collect::<Vec<_>>()
            .len();
        if inter == 0 {
            continue;
        }
        winning_sum += 2i64.pow((inter - 1).try_into()?);

        // add copies of cards if we won
        for j in 0..inter {
            cc.add_copy(i, j);
        }
    }

    println!("Sum: {winning_sum}");

    println!("Card Counter: {}", cc.get_cards().values().sum::<usize>());

    Ok(())
}
