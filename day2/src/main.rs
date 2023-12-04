use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit0, digit1, multispace0},
    combinator::map_res,
    multi::separated_list0,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Color {
    Red(i32),
    Green(i32),
    Blue(i32),
}

impl Color {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, (cnt, color)) = preceded(
            multispace0,
            separated_pair(
                map_res(digit0, str::parse),
                tag(" "),
                alt((tag("red"), tag("green"), tag("blue"))),
            ),
        )(input)?;

        let ret = match color {
            "red" => Color::Red(cnt),
            "green" => Color::Green(cnt),
            "blue" => Color::Blue(cnt),
            // shouldn't get here because nom would error out
            _ => unimplemented!(),
        };

        Ok((remaining, ret))
    }

    // return true if the color is within bounds
    pub fn validate(&self) -> bool {
        match self {
            Color::Red(val) => val <= &12,
            Color::Green(val) => val <= &13,
            Color::Blue(val) => val <= &14,
        }
    }
}

#[derive(Clone, Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl std::default::Default for Round {
    fn default() -> Self {
        Round {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl Round {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        //separated_list0(tag(","), Color::parse)(input)
        let (remaining, colors) = separated_list0(tag(","), Color::parse)(input)?;
        let mut r = Round::default();

        for c in colors {
            match c {
                Color::Red(val) => r.red = val,
                Color::Green(val) => r.green = val,
                Color::Blue(val) => r.blue = val,
            }
        }

        Ok((remaining, r))
    }
}

#[derive(Clone, Debug)]
struct Game {
    pub id: u32,
    rounds: Vec<Round>,
}

fn parse_id(input: &str) -> IResult<&str, u32> {
    terminated(
        preceded(tag("Game "), map_res(digit1, str::parse)),
        tag(": "),
    )(input)
}

impl Game {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (remaining, (id, rounds)) =
            tuple((parse_id, separated_list0(tag(";"), Round::parse)))(input)?;
        Ok((remaining, Game { id, rounds }))
    }

    // get the max for each color amongst each round
    pub fn power(&self) -> i32 {
        let red = self.rounds.iter().map(|r| r.red).max();
        let green = self.rounds.iter().map(|r| r.green).max();
        let blue = self.rounds.iter().map(|r| r.blue).max();
        red.unwrap() * green.unwrap() * blue.unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut power_sum = 0;

    let input = include_str!("../input.txt");

    let (_remaining, games) = separated_list0(tag("\n"), Game::parse)(input)?;

    for g in &games {
        let power = g.power();
        power_sum += power;
    }

    println!("Answer: {power_sum}");

    Ok(())
}
