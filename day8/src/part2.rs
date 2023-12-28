use anyhow::anyhow;
use anyhow::Error;
use std::collections::HashMap;
use std::env;
use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, char, multispace0},
    combinator::value,
    multi::many0,
    sequence::delimited,
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone)]
struct StepParsingError;

#[derive(Debug, Clone)]
struct MapParsingError;

impl std::fmt::Display for StepParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid step parsing")
    }
}

impl std::fmt::Display for MapParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid map parsing")
    }
}

#[derive(Debug, Clone)]
enum Step {
    Left,
    Right,
}

impl Step {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        // either a right or left
        alt((value(Step::Right, char('R')), value(Step::Left, char('L'))))(input)
    }
}

/// Helper type to assist in walking through the map
/// Impl's into iterator to create a wrap around ring buffer through
/// the Map's steps
struct Navigator<'a> {
    map: &'a Map,
    curr: usize,
}

impl<'a> Iterator for Navigator<'a> {
    type Item = &'a Step;

    fn next(&mut self) -> Option<Self::Item> {
        // return the self.curr % self.map.steps.len() step
        let ret = self.map.steps.get(self.curr % self.map.steps.len());
        self.curr += 1;
        ret
    }
}

#[derive(Debug)]
struct Map {
    /// Holds the steps to take through the map
    steps: Vec<Step>,

    /// Holds the actual mappings to walk through
    mapping: HashMap<String, (String, String)>,
}

impl Map {
    pub fn iter(&self) -> Navigator {
        Navigator { map: self, curr: 0 }
    }

    pub fn get_next_loc(&self, loc: &String, step: &Step) -> String {
        match self.mapping.get(loc) {
            None => "AAA".to_string(),
            Some((left, right)) => match step {
                Step::Left => left.clone(),
                Step::Right => right.clone(),
            },
        }
    }

    /// gather the starting positions (for part 2)
    pub fn starting_positions(&self) -> Vec<String> {
        let mut starting = Vec::new();
        // loop through all of the keys in the mapping
        for loc in self.mapping.keys() {
            if loc.ends_with('A') {
                starting.push(loc.clone());
            }
        }
        starting
    }

    /// returns true if these are all destinations (end with 'Z')
    pub fn destination(&self, positions: &Vec<String>) -> bool {
        for loc in positions {
            if !loc.ends_with('Z') {
                return false;
            }
        }
        true
    }
}

fn step_parser(input: &str) -> IResult<&str, Vec<Step>> {
    many0(preceded(multispace0, Step::parse))(input)
}

fn parse_dest(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        alphanumeric1,
        tag(","),
        preceded(multispace0, alphanumeric1),
    )(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alphanumeric1,
        delimited(multispace0, tag("="), multispace0),
        delimited(tag("("), parse_dest, tag(")")),
    )(input)
}

fn map_parser(input: &str) -> IResult<&str, HashMap<String, (String, String)>> {
    let (remain, maps) = many0(preceded(multispace0, parse_line))(input)?;
    let mut mapping = HashMap::new();
    // loop through each of the destinations we get and add each to a hashmap
    for m in &maps {
        match mapping.insert(m.0.to_string(), (m.1 .0.to_string(), m.1 .1.to_string())) {
            None => {
                continue;
            }
            Some(_) => {
                continue;
            }
        }
    }
    Ok((remain, mapping))
}

fn parse(input: &str) -> Result<Map, Error> {
    let (remain, steps) = match step_parser(input) {
        Err(_) => {
            return Err(anyhow!("Step parsing error"));
        }
        Ok((r, s)) => (r, s),
    };
    let (_remain, map) = match map_parser(remain) {
        Err(_) => {
            return Err(anyhow!("Map Parsing error"));
        }
        Ok((r, m)) => (r, m),
    };

    Ok(Map {
        steps,
        mapping: map,
    })
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();

    let choice = match args.get(1) {
        None => panic!("Bad arguments"),
        Some(c) => c.as_str(),
    };

    let filename = match choice {
        "1" => "test1.txt",
        "2" => "test2.txt",
        "3" => "test3.txt",
        "i" | "I" => "input.txt",
        _ => panic!("invalid choice: 1, 2, i/I"),
    };

    let file = fs::read_to_string(filename).unwrap();
    let map = parse(&file)?;
    println!("{map:?}");

    // could have included some state into the iterator, but it doesn't
    // necessarily make the most sense. the iterator truly should just move
    // one step along the path and expose the next one. we should keep state internally

    // gather the starting positions
    let mut positions = map.starting_positions();
    let mut steps: u64 = 0;
    let mut distances = Vec::new();

    // have to be smart :) need to figure out the length of the route from
    // each of the starting positions to their end (when each of them hit a 'Z')
    // we can remove the ones that have a hit their point from positions
    // once we know how many steps it takes to get to each of these distances,
    // we can then find how long it would take to get to each of them.
    // if it takes 3 steps to complete route A and 4 steps to complete route B, then it would take
    // 12 steps to finish both of them simultaneously (complete route A 4 times, route B 3 times
    //    and both will be at the end)
    // thus, we need to find the LCM for all of our starting positions

    for step in map.iter() {
        // grab the next location for each of the positions
        let mut next_pos = Vec::new();
        for p in &positions {
            next_pos.push(map.get_next_loc(p, step));
        }

        steps += 1;

        // keep only the positions that don't end in 'Z'
        next_pos.retain(|x| !x.ends_with('Z'));
        // add the difference in the lengths of the positions and the next_pos
        // that we kept
        // (this ensure that if two routes end up both ending, we still retain it)
        for _ in 0..(positions.len() - next_pos.len()) {
            distances.push(steps);
        }

        positions = next_pos;
        // if we don't have anything
        if positions.len() == 0 {
            break;
        }
    }
    println!("Got distances: {distances:?}");

    // now we need to find the LCM of all of these numbers together
    let p2 = distances.iter().fold(1_u64, |mut acc, x| {
        acc = num::integer::lcm(acc, *x);
        acc
    });

    println!("P2: {p2}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "AAA = (AAA, BBB)";
        let (_, p) = parse_line(line).unwrap();

        assert_eq!(p.0, "AAA");
        assert_eq!(p.1 .0, "AAA");
        assert_eq!(p.1 .1, "BBB");
    }
}
