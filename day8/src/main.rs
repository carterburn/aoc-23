use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, char, multispace0},
    combinator::value,
    multi::many0,
    sequence::delimited,
    sequence::preceded,
    sequence::separated_pair,
    IResult,
};

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
}

fn step_parser(input: &str) -> IResult<&str, Vec<Step>> {
    many0(preceded(multispace0, Step::parse))(input)
}

fn parse_dest(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alpha1, tag(","), preceded(multispace0, alpha1))(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    separated_pair(
        alpha1,
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

fn parse(input: &str) -> Result<Map, Box<dyn Error>> {
    let (remain, steps) = match step_parser(input) {
        Err(_) => {
            // TODO: change these to custom error types eventually
            return Err(Box::new(std::io::Error::from_raw_os_error(9)));
        }
        Ok((r, s)) => (r, s),
    };
    let (_remain, map) = match map_parser(remain) {
        Err(_) => {
            // TODO: see TODO on 123
            return Err(Box::new(std::io::Error::from_raw_os_error(9)));
        }
        Ok((r, m)) => (r, m),
    };

    Ok(Map {
        steps,
        mapping: map,
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let choice = match args.get(1) {
        None => panic!("Bad arguments"),
        Some(c) => c.as_str(),
    };

    let filename = match choice {
        "1" => "test1.txt",
        "2" => "test2.txt",
        "i" | "I" => "input.txt",
        _ => panic!("invalid choice: 1, 2, i/I"),
    };

    let file = fs::read_to_string(filename).unwrap();
    let map = parse(&file)?;
    println!("{map:?}");

    // loop through the map and count each step
    let mut loc = "AAA".to_string();
    let mut steps = 0;

    for step in map.iter() {
        loc = map.get_next_loc(&loc, step);
        println!("Loc: {loc}, Step: {step:?}");
        steps += 1;
        if loc == "ZZZ".to_string() {
            break;
        }
    }
    println!("Part 1: {steps}");

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
