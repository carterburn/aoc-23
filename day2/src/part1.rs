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

fn get_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Copy, Clone, Debug)]
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
    colors: Vec<Color>,
}

impl Round {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        //separated_list0(tag(","), Color::parse)(input)
        let (remaining, colors) = separated_list0(tag(","), Color::parse)(input)?;
        Ok((remaining, Round { colors }))
    }

    // returns true if the round is within the bounds for each color
    pub fn validate(&self) -> bool {
        for c in &self.colors {
            if !c.validate() {
                return false;
            }
        }
        true
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

    pub fn validate(&self) -> bool {
        // validate each round
        for r in &self.rounds {
            if !r.validate() {
                return false;
            }
        }
        true
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut id_sum = 0;

    let input = include_str!("../input.txt");

    let (_remaining, games) = separated_list0(tag("\n"), Game::parse)(input)?;

    for g in &games {
        if g.validate() {
            id_sum += g.id;
        }
    }

    println!("Answer: {id_sum}");

    Ok(())
}
