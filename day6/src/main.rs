use nom::{
    bytes::complete::tag,
    character::complete::{i32, multispace0},
    error::{ErrorKind, ParseError},
    multi::many0,
    sequence::preceded,
    IResult,
};

use std::error::Error as RError;

use std::time::{Duration, Instant};

// quick custom error type
#[derive(Debug, PartialEq)]
pub enum RaceParseError<I> {
    NoTimes,
    NoDist,
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for RaceParseError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        RaceParseError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

/// Struct for different races
#[derive(Debug)]
struct IslandRaces {
    races: Vec<Race>,
}

impl IslandRaces {
    pub fn parse(input: &str) -> IResult<&str, Self, RaceParseError<&str>> {
        // parse times (discard any whitespace before 'Time:')
        let (input, times) = preceded(
            multispace0,
            preceded(tag("Time:"), many0(preceded(multispace0, i32))),
        )(input)?;

        // parse distances (discared any whitespace before 'Distance:'; like a \n)
        let (remaining, distances) = preceded(
            multispace0,
            preceded(tag("Distance:"), many0(preceded(multispace0, i32))),
        )(input)?;

        println!("times: {times:?}");
        println!("dists: {distances:?}");

        // zip them up
        let races = times
            .iter()
            .zip(distances)
            .map(|(t, d)| Race::new(*t, d))
            .collect();

        Ok((remaining, IslandRaces { races }))
    }

    fn compute_records(&self) -> i32 {
        // compute each race's possible ways to win, then multiply them all
        self.races.iter().map(|race| race.ways_to_win()).product()
    }

    fn remove_kerning(&self) -> Result<u64, Box<dyn RError>> {
        // convert to a string to just combine and avoid ugly decimal math
        // also, need to update the races to support larger numbers
        let new_time = self
            .races
            .iter()
            .fold(String::new(), |mut acc, x| {
                let s = x.total_time.to_string();
                acc.push_str(s.as_str());
                acc
            })
            .parse::<u64>()?;

        let new_dist = self
            .races
            .iter()
            .fold(String::new(), |mut acc, x| {
                let s = x.record_dist.to_string();
                acc.push_str(s.as_str());
                acc
            })
            .parse::<u64>()?;

        println!("New Time: {new_time:?}");
        println!("New Dist: {new_dist:?}");

        let r = BigRace::new(new_time, new_dist);

        let naive_start = Instant::now();
        let naive = r.naive();
        let naive_dur = naive_start.elapsed();

        let smart_start = Instant::now();
        let smart = r.smart();
        let smart_dur = smart_start.elapsed();

        println!("naive() = {naive}; Took {naive_dur:?}");
        println!("smart() = {smart}; Took {smart_dur:?}");

        Ok(smart)
    }
}

/// Struct to describe a race
#[derive(Debug)]
struct Race {
    total_time: i32,
    record_dist: i32,
}

impl Race {
    pub fn new(total_time: i32, record_dist: i32) -> Self {
        Race {
            total_time,
            record_dist,
        }
    }

    pub fn ways_to_win(&self) -> i32 {
        (0..=self.total_time)
            .filter_map(|ms_held| {
                let dist = (self.total_time - ms_held) * ms_held;
                if dist > self.record_dist {
                    Some(dist)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .len() as i32
    }
}

#[derive(Debug)]
struct BigRace {
    total_time: u64,
    record_dist: u64,
}

impl BigRace {
    pub fn new(total_time: u64, record_dist: u64) -> Self {
        BigRace {
            total_time,
            record_dist,
        }
    }

    pub fn naive(&self) -> u64 {
        (0..=self.total_time)
            .filter_map(|ms_held| {
                let dist = (self.total_time - ms_held) * ms_held;
                if dist > self.record_dist {
                    Some(dist)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .len() as u64
    }

    pub fn smart(&self) -> u64 {
        // can also just find the first occurence from the start of the range and then from the
        // back
        let mut first = 0;
        for s in 0..=self.total_time {
            if ((self.total_time - s) * s) > self.record_dist {
                first = s;
                break;
            }
        }
        let mut second = 0;
        for s in (0..=self.total_time).rev() {
            if ((self.total_time - s) * s) > self.record_dist {
                second = s;
                break;
            }
        }

        second - first + 1
    }
}

fn main() -> Result<(), Box<dyn RError>> {
    //let input = include_str!("../test.txt");
    let input = include_str!("../input.txt");
    // good opportunity to use nom because there isn't a clean break in the input
    let (_remaining, r) = IslandRaces::parse(input)?;
    println!("Races = {r:?}");
    let answer = r.compute_records();
    println!("Part 1: {answer:?}");
    // part 2 is annoying, but shouldn't be too bad. just need to convert the races vec in r
    let p2 = r.remove_kerning()?;
    println!("P2: {p2}");
    Ok(())
}
